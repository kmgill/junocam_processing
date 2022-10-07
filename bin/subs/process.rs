use crate::subs::runnable::RunnableSubcommand;

use junocam::{
    path,
    rawimage,
    junocam as jc,
    strip::Strip,
    jcspice,
    metadata,
    drawable::Drawable,
    drawable::Point,
    config,
    lens::lens::Lens,
    lens::cylindrical::CylindricalLens,
    lens::fisheye::FisheyeEquisolidLens,
    junocam::FrameletParameters,
    vprintln
};

use sciimg::{
    prelude::*,
    vector::Vector,
    matrix::Matrix,
    quaternion::Quaternion
};

use std::process;

enum SupportedLens {
    Cylindrical,
    Fisheye
}

impl SupportedLens {

    pub fn from(s:&str) -> Option<SupportedLens> {
        match s.to_lowercase().as_str() {
            "cylindrical" => Some(SupportedLens::Cylindrical),
            "fisheye" => Some(SupportedLens::Fisheye),
            _ => None
        }
    }
}

#[derive(clap::Args)]
#[clap(author, version, about = "Process RGB JunoCam image", long_about = None)]
pub struct Process {
    #[clap(long, short, help = "Input image")]
    input: String,

    #[clap(long, short, help = "Input metadata json")]
    metadata: String,

    #[clap(long, short, help = "Output image")]
    output: String,

    #[clap(long, short = 'R', help = "Red weight")]
    red_weight: Option<f32>,

    #[clap(long, short = 'G', help = "Green weight")]
    green_weight: Option<f32>,

    #[clap(long, short = 'B', help = "Blue weight")]
    blue_weight: Option<f32>,

    #[clap(long, short, help = "Use predicted kernels")]
    predicted: bool,

    #[clap(long, short, help = "Output width")]
    width: Option<usize>,

    #[clap(long, short = 'H', help = "Output height")]
    height: Option<usize>,

    #[clap(long, short, help = "Fisheye camera field of view, in degrees")]
    fov: Option<f64>,

    #[clap(long, short = 'P', help = "Camera pitch, in degrees", allow_hyphen_values(true))]
    pitch: Option<f64>,

    #[clap(long, short, help = "Camera yaw, in degrees", allow_hyphen_values(true))]
    yaw: Option<f64>,

    #[clap(long, short, help = "Camera lens (cylindrical, fisheye)")]
    lens: Option<String>
}   


fn xy_to_map_point(x:usize, y:usize, framelet:&FrameletParameters, spc_mtx:&Matrix, lens:&Box<dyn Lens>, strip:&Strip, q:&Quaternion) -> Point {
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

impl RunnableSubcommand for Process {
    fn run(&self) {

        let juno_config = config::load_configuration().expect("Failed to load config file");

        

        if ! path::file_exists(&self.input) {
            eprintln!("ERROR: Input file not found!");
            process::exit(1);
        }

        let red_weight = match self.red_weight {
            Some(r) => r,
            None => juno_config.defaults.red_weight
        };

        let green_weight = match self.green_weight {
            Some(g) => g,
            None => juno_config.defaults.green_weight
        };

        let blue_weight = match self.blue_weight {
            Some(b) => b,
            None => juno_config.defaults.blue_weight
        };

        let output_width = match self.width {
            Some(w) => w,
            None => 1024
        };
        vprintln!("Output image width: {}", output_width);

        let output_height = match self.height {
            Some(h) => h,
            None => 1024
        };
        vprintln!("Output image height: {}", output_height);

        let camera_lens = match &self.lens {
            Some(l) => {
                if let Some(lens) = SupportedLens::from(&l.as_str()) {
                    lens
                } else {
                    eprintln!("Error: Invalid camera lens requested: {}", l);
                    eprintln!("Use either 'cylidrical' or 'fisheye'");
                    process::exit(1);
                }
            },
            None => SupportedLens::Fisheye
        };



        vprintln!("Loading metadata from {}", self.metadata);
        let md = metadata::Metadata::new_from_file(&self.metadata).expect("Failed to load input metadata");

        vprintln!("Loading image file from {}", self.input);
        vprintln!("Decompanding with table '{:?}'", md.sample_bit_mode_id);
        let mut raw_image = rawimage::RawImage::new_from_image_with_decompand(&self.input, md.sample_bit_mode_id).unwrap();

        let fov = match self.fov {
            Some(f) => f,
            None => 180.0
        };
        vprintln!("Fisheye field of view: {}", fov);

        let pitch = match self.pitch {
            Some(p) => p.to_radians() * -1.0, // Make it positive up
            None => 0.0
        };
        vprintln!("Fisheye camera pitch: {}", pitch.to_degrees());

        let yaw = match self.yaw {
            Some(y) => y.to_radians() * -1.0, // Make it positive right
            None => 0.0
        };
        vprintln!("Fisheye camera yaw: {}", yaw.to_degrees());



        vprintln!("Applying framelet calibration...");
        raw_image.apply_darknoise().expect("Error with dark/flat field correction");

        vprintln!("Applying blemish infill correction...");
        raw_image.apply_infill_correction().expect("Error with infill correction");

        vprintln!("Applying hot pixel detection and correction...");
        raw_image.apply_hot_pixel_correction(5, 2.0).expect("Error wih hot pixel correction");
        
        vprintln!("Applying channel weight multiples ({}, {}, {} X R, G, B)...", red_weight, green_weight, blue_weight);
        raw_image.apply_weights(red_weight, green_weight, blue_weight).expect("Error applying channel weight values");
    
        vprintln!("Loading base kernels...");
        jcspice::furnish_base();

        let interframe_delay = md.interframe_delay as f64;
        let interframe_delay_correction = juno_config.defaults.interframe_delay_correction;
        let start_time_correction = juno_config.defaults.start_time_correction;
        vprintln!("Interframe delay: {}", interframe_delay);
        vprintln!("Interframe delay correction: {}", interframe_delay_correction);
        vprintln!("Start time correction: {}", start_time_correction);

        let start_time_utc = md.start_time;
        vprintln!("Start time from metadata: {:?}", start_time_utc);
        let start_time = start_time_utc.format("%Y-%h-%d %H:%M:%S%.3f").to_string();
        vprintln!("Spice-formatted start time: {}", start_time);
        let start_time_et = jcspice::string_to_et(&start_time) + start_time_correction;

        let kernel_search_pattern = if self.predicted {
            juno_config.spice.ck_pre_pattern
        } else {
            juno_config.spice.ck_rec_pattern
        };

        vprintln!("Finding spacecraft pointing kernel...");
        match jcspice::find_kernel_with_date(&kernel_search_pattern, start_time_et) {
            Ok(kernel_path) => {
                vprintln!("Found CK kernel with matching time range: {}", kernel_path);
                jcspice::furnish(&kernel_path).expect("Failed to load kernel");
            },
            Err(why) => {
                eprintln!("Error: {:?}", why);
                process::exit(1);
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
        let user_yaw = Quaternion::from_pitch_roll_yaw(0.0, 0.0, pitch);
        let user_pitch = Quaternion::from_pitch_roll_yaw(0.0, yaw, pitch);

        let q = user_yaw.times(&user_pitch.times(&r.times(&p.times(&Quaternion::from_matrix(&midtime_matrix).invert()))));

        let mut cyl_map = RgbImage::create(output_width, output_height);

        let lens: Box<dyn Lens> = match camera_lens {
            SupportedLens::Cylindrical => Box::new(CylindricalLens::new(cyl_map.width, cyl_map.height, 90.0, -90.0, 0.0, 360.0)),
            SupportedLens::Fisheye => Box::new(FisheyeEquisolidLens::new(cyl_map.width, cyl_map.height, 13.0, fov))
        };
        //let lens = CylindricalLens::new(cyl_map.width, cyl_map.height, 90.0, -90.0, 0.0, 360.0);
        //let lens = FisheyeEquisolidLens::new(cyl_map.width, cyl_map.height, 13.0, fov);

        vprintln!("Processing triplets...");
        for t in 0..raw_image.get_triplet_count() {
            vprintln!("Processing triplet #{}", (t + 1));
            let triplet = &raw_image.triplets[t as usize];

            let image_time_et = start_time_et + (t as f64 * (interframe_delay +  interframe_delay_correction));
            let spc_mtx = jcspice::pos_transform_matrix("JUNO_JUNOCAM", "J2000", image_time_et);
            
            for y in 2..(128 - 2){
                for x in 0..(1648 - 1) {

                    for s in 0..3 {
                        let strip = &triplet.channels[s];

                        let framelet = match s {
                            0 => &jc::JUNO_JUNOCAM_BLUE,
                            1 => &jc::JUNO_JUNOCAM_GREEN,
                            2 => &jc::JUNO_JUNOCAM_RED,
                            4 => &jc::JUNO_JUNOCAM_METHANE,
                            _ => panic!("Invalid filter band")
                        };
                        let tl = xy_to_map_point(x, y, &framelet, &spc_mtx, &lens, strip, &q);
                        let bl = xy_to_map_point(x, y+1, &framelet, &spc_mtx, &lens, strip, &q);
                        let br = xy_to_map_point(x+1, y+1, &framelet, &spc_mtx, &lens, strip, &q);
                        let tr = xy_to_map_point(x+1, y, &framelet, &spc_mtx, &lens, strip, &q);
                        
                        cyl_map.paint_square(&tl, &bl, &br, &tr, false, 2 - s);

                    }
                }
            }
        }

        vprintln!("Data range, pre-normalization:");
        vprintln!("MinMax: {:?}", cyl_map.get_min_max_all_channel());
        //cyl_map.normalize_to_16bit();
        cyl_map.normalize_to_16bit_seperate_channels();

        vprintln!("Data range, post-normalization:");
        vprintln!("MinMax: {:?}", cyl_map.get_min_max_all_channel());

        vprintln!("Writing output image to {}", self.output);
        cyl_map.save(&self.output);
    }
}

trait NormSeperateChannel {
    fn normalize_to_16bit_seperate_channels(&mut self);
}

impl NormSeperateChannel for  RgbImage {
    fn normalize_to_16bit_seperate_channels(&mut self) {

        for b in 0..self.num_bands() {
            let band = self.get_band(b);
            self.set_band(&band.normalize(0.0, 65535.0).unwrap(), b);

        }

    }
}