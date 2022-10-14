use crate::{
    config, drawable::Drawable, drawable::Point, jcspice, junocam as jc,
    junocam::FrameletParameters, lens::cylindrical::CylindricalLens,
    lens::fisheye::FisheyeEquisolidLens, lens::lens::Lens, metadata, rawimage, strip::Strip,
    vprintln,
};

use sciimg::{matrix::Matrix, prelude::*, quaternion::Quaternion, vector::Vector};

pub enum SupportedLens {
    Cylindrical,
    Fisheye,
}

impl SupportedLens {
    pub fn from(s: &str) -> Option<SupportedLens> {
        match s.to_lowercase().as_str() {
            "cylindrical" => Some(SupportedLens::Cylindrical),
            "fisheye" => Some(SupportedLens::Fisheye),
            _ => None,
        }
    }
}

fn xy_to_map_point(
    x: usize,
    y: usize,
    framelet: &FrameletParameters,
    spc_mtx: &Matrix,
    lens: &Box<dyn Lens>,
    strip: &Strip,
    q: &Quaternion,
) -> Point {
    let mut v = framelet.xy_to_vector(x as f64, y as f64);
    v = spc_mtx.multiply_vector(&v);
    v = q.rotate_vector(&v);

    // Translate from spice coordinates to ours.
    v = Vector::new(v.x, v.z, v.y);

    let mut pt = lens.vector_to_point(&v);
    let tl_v = strip.buffer.get(x, y).unwrap() as f64;
    pt.v = tl_v;
    pt
}

pub struct ProcessOptions {
    pub input: String,
    pub metadata: String,
    pub output: Option<String>,
    pub red_weight: f32,
    pub green_weight: f32,
    pub blue_weight: f32,
    pub predicted: bool,
    pub width: usize,
    pub height: usize,
    pub fov: f64,
    pub pitch: f64,
    pub yaw: f64,
    pub roll: f64,
    pub lens: SupportedLens,
}

pub fn process_image(context: &ProcessOptions) -> error::Result<RgbImage> {
    let juno_config = match config::load_configuration() {
        Ok(jc) => jc,
        Err(why) => return Err(why),
    };

    vprintln!("Loading metadata from {}", context.metadata);
    let md = match metadata::Metadata::new_from_file(&context.metadata) {
        Ok(md) => md,
        Err(why) => return Err(why),
    };

    vprintln!("Loading image file from {}", context.input);
    vprintln!("Decompanding with table '{:?}'", md.sample_bit_mode_id);
    let mut raw_image = match rawimage::RawImage::new_from_image_with_decompand(
        &context.input,
        md.sample_bit_mode_id,
    ) {
        Ok(img) => img,
        Err(why) => return Err(why),
    };

    if juno_config.defaults.apply_calibration {
        vprintln!("Applying framelet calibration...");
        match raw_image.apply_darknoise() {
            Ok(_) => {}
            Err(why) => return Err(why),
        };
    }

    if juno_config.defaults.apply_infill_correction {
        vprintln!("Applying blemish infill correction...");
        match raw_image.apply_infill_correction() {
            Ok(_) => {}
            Err(why) => return Err(why),
        };
    }

    if juno_config.defaults.apply_hot_pixel_correction {
        vprintln!("Applying hot pixel detection and correction...");
        vprintln!(
            "Hot Pixel Correction Window Size: {}",
            juno_config.defaults.hpc_window_size
        );
        vprintln!(
            "Hot Pixel Threshold: {}",
            juno_config.defaults.hpc_threshold
        );
        match raw_image.apply_hot_pixel_correction(
            juno_config.defaults.hpc_window_size,
            juno_config.defaults.hpc_threshold,
        ) {
            Ok(_) => {}
            Err(why) => return Err(why),
        };
    }

    if juno_config.defaults.apply_weights {
        vprintln!(
            "Applying channel weight multiples ({}, {}, {} X R, G, B)...",
            context.red_weight,
            context.green_weight,
            context.blue_weight
        );
        match raw_image.apply_weights(
            context.red_weight,
            context.green_weight,
            context.blue_weight,
        ) {
            Ok(_) => {}
            Err(why) => return Err(why),
        };
    }

    vprintln!("Loading base kernels...");
    jcspice::furnish_base();

    let interframe_delay = md.interframe_delay as f64;
    let interframe_delay_correction = juno_config.defaults.interframe_delay_correction;
    let start_time_correction = juno_config.defaults.start_time_correction;
    vprintln!("Interframe delay: {}", interframe_delay);
    vprintln!(
        "Interframe delay correction: {}",
        interframe_delay_correction
    );
    vprintln!("Start time correction: {}", start_time_correction);

    let start_time_utc = md.start_time;
    vprintln!("Start time from metadata: {:?}", start_time_utc);
    let start_time = start_time_utc.format("%Y-%h-%d %H:%M:%S%.3f").to_string();
    vprintln!("Spice-formatted start time: {}", start_time);
    let start_time_et = jcspice::string_to_et(&start_time) + start_time_correction;

    let kernel_search_pattern = if context.predicted {
        juno_config.spice.ck_pre_pattern
    } else {
        juno_config.spice.ck_rec_pattern
    };

    vprintln!("Finding spacecraft pointing kernel...");
    match jcspice::find_kernel_with_date(&kernel_search_pattern, start_time_et) {
        Ok(kernel_path) => {
            vprintln!("Found CK kernel with matching time range: {}", kernel_path);
            // Note: I really don't like embedding match statements within match statements.
            match jcspice::furnish(&kernel_path) {
                Ok(_) => {}
                Err(why) => return Err(why),
            };
        }
        Err(why) => {
            eprintln!("Error: {:?}", why);
            return Err(why);
        }
    }

    let stop_time_utc = md.stop_time;
    vprintln!("Stop time from metadata: {:?}", stop_time_utc);
    let stop_time = stop_time_utc.format("%Y-%h-%d %H:%M:%S%.3f").to_string();
    vprintln!("Spice-formatted stop time: {}", stop_time);
    let stop_time_et = jcspice::string_to_et(&stop_time) + start_time_correction;

    let mid_time_et = (start_time_et + stop_time_et) / 2.0;
    let midtime_matrix = jcspice::pos_transform_matrix("JUNO_JUNOCAM", "J2000", mid_time_et);

    let r = Quaternion::from_pitch_roll_yaw(180.0_f64.to_radians(), 0.0, 0.0);
    let p = Quaternion::from_pitch_roll_yaw(0.0, 90.0_f64.to_radians(), 0.0);

    // We flip them to handle Spice's Z-up to our Y-up coordinates
    let user_roll = Quaternion::from_pitch_roll_yaw(context.roll, 0.0, 0.0);
    let user_yaw = Quaternion::from_pitch_roll_yaw(0.0, 0.0, context.pitch);
    let user_pitch = Quaternion::from_pitch_roll_yaw(0.0, context.yaw, 0.0);

    let q = user_roll.times(&user_yaw.times(
        &user_pitch.times(&r.times(&p.times(&Quaternion::from_matrix(&midtime_matrix).invert()))),
    ));

    let mut cyl_map = RgbImage::create(context.width, context.height);

    let lens: Box<dyn Lens> = match context.lens {
        SupportedLens::Cylindrical => Box::new(CylindricalLens::new(
            cyl_map.width,
            cyl_map.height,
            90.0,
            -90.0,
            0.0,
            360.0,
        )),
        SupportedLens::Fisheye => Box::new(FisheyeEquisolidLens::new(
            cyl_map.width,
            cyl_map.height,
            13.0,
            context.fov,
        )),
    };
    //let lens = CylindricalLens::new(cyl_map.width, cyl_map.height, 90.0, -90.0, 0.0, 360.0);
    //let lens = FisheyeEquisolidLens::new(cyl_map.width, cyl_map.height, 13.0, fov);

    vprintln!("Processing triplets...");
    for t in 0..raw_image.get_triplet_count() {
        vprintln!("Processing triplet #{}", (t + 1));
        let triplet = &raw_image.triplets[t as usize];

        let image_time_et =
            start_time_et + (t as f64 * (interframe_delay + interframe_delay_correction));
        let spc_mtx = jcspice::pos_transform_matrix("JUNO_JUNOCAM", "J2000", image_time_et);

        for y in 2..(128 - 2) {
            for x in 0..(1648 - 1) {
                for s in 0..3 {
                    let strip = &triplet.channels[s];

                    let framelet = match s {
                        0 => &jc::JUNO_JUNOCAM_BLUE,
                        1 => &jc::JUNO_JUNOCAM_GREEN,
                        2 => &jc::JUNO_JUNOCAM_RED,
                        4 => &jc::JUNO_JUNOCAM_METHANE,
                        _ => panic!("Invalid filter band"),
                    };
                    let tl = xy_to_map_point(x, y, &framelet, &spc_mtx, &lens, strip, &q);
                    let bl = xy_to_map_point(x, y + 1, &framelet, &spc_mtx, &lens, strip, &q);
                    let br = xy_to_map_point(x + 1, y + 1, &framelet, &spc_mtx, &lens, strip, &q);
                    let tr = xy_to_map_point(x + 1, y, &framelet, &spc_mtx, &lens, strip, &q);

                    cyl_map.paint_square(&tl, &bl, &br, &tr, true, 2 - s);
                }
            }
        }
    }

    vprintln!("Data range, pre-normalization:");
    vprintln!("MinMax: {:?}", cyl_map.get_min_max_all_channel());

    if juno_config.defaults.correlated_color_balancing {
        vprintln!("Applying color channel correlated value stretching/normalization");
        cyl_map.normalize_to_16bit();
    } else {
        vprintln!("Applying color channel isolated value stretching/normalization");
        cyl_map.normalize_to_16bit_seperate_channels();
    }

    vprintln!("Data range, post-normalization:");
    vprintln!("MinMax: {:?}", cyl_map.get_min_max_all_channel());

    match &context.output {
        Some(output) => {
            vprintln!("Writing output image to {}", output);
            cyl_map.save(output);
        }
        None => {}
    };

    Ok(cyl_map)
}

trait NormSeperateChannel {
    fn normalize_to_16bit_seperate_channels(&mut self);
}

impl NormSeperateChannel for RgbImage {
    fn normalize_to_16bit_seperate_channels(&mut self) {
        for b in 0..self.num_bands() {
            let band = self.get_band(b);
            self.set_band(&band.normalize(0.0, 65535.0).unwrap(), b);
        }
    }
}
