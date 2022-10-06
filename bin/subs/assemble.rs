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


#[derive(clap::Args)]
#[clap(author, version, about = "Assemble triplets to cylindrical map", long_about = None)]
pub struct Assemble {
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
    blue_weight: Option<f32>
}   


fn xy_to_map_point<T:Lens>(x:usize, y:usize, framelet:&FrameletParameters, spc_mtx:&Matrix, lens:&T, strip:&Strip, q:&Quaternion) -> Point {
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

impl RunnableSubcommand for Assemble {
    fn run(&self) {

        let defaults = config::load_configuration().expect("Failed to load config file");

        if ! path::file_exists(&self.input) {
            eprintln!("ERROR: Input file not found!");
            process::exit(1);
        }

        let red_weight = match self.red_weight {
            Some(r) => r,
            None => defaults.defaults.red_weight
        };

        let green_weight = match self.green_weight {
            Some(g) => g,
            None => defaults.defaults.green_weight
        };

        let blue_weight = match self.blue_weight {
            Some(b) => b,
            None => defaults.defaults.blue_weight
        };


        vprintln!("Loading metadata from {}", self.metadata);
        let md = metadata::Metadata::new_from_file(&self.metadata).expect("Failed to load input metadata");

        vprintln!("Loading image file from {}", self.input);
        vprintln!("Decompanding with table '{:?}'", md.sample_bit_mode_id);
        let mut raw_image = rawimage::RawImage::new_from_image_with_decompand(&self.input, md.sample_bit_mode_id).unwrap();

        raw_image.apply_darknoise().expect("Error with dark/flat field correction");
        raw_image.apply_infill_correction().expect("Error with infill correction");
        raw_image.apply_hot_pixel_correction(5, 2.0).expect("Error wih hot pixel correction");
        raw_image.apply_weights(red_weight, green_weight, blue_weight).expect("Error applying channel weight values");
    
        jcspice::furnish_base();
        jcspice::furnish("kernels/spk/spk_rec_210127_210321_210329.bsp").expect("Failed to load spice kernel");
        jcspice::furnish("kernels/ck/juno_sc_rec_210221_210227_v01.bc").expect("Failed to load spice kernel");

        let interframe_delay = md.interframe_delay as f64;
        let interframe_delay_correction = 0.001;
        let start_time_correction = 0.06188;

        let start_time_utc = md.start_time;
        vprintln!("Start time from metadata: {:?}", start_time_utc);
        let start_time = start_time_utc.format("%Y-%h-%d %H:%M:%S%.3f").to_string();
        vprintln!("Spice-formatted start time: {}", start_time);
        let start_time_et = jcspice::string_to_et(&start_time) + start_time_correction;

        let stop_time_utc = md.stop_time;
        vprintln!("Stop time from metadata: {:?}", stop_time_utc);
        let stop_time = stop_time_utc.format("%Y-%h-%d %H:%M:%S%.3f").to_string();
        vprintln!("Spice-formatted stop time: {}", stop_time);
        let stop_time_et = jcspice::string_to_et(&stop_time) + start_time_correction;

        let mid_time_et = (start_time_et + stop_time_et) / 2.0;
        let midtime_matrix = jcspice::pos_transform_matrix("JUNO_JUNOCAM", "J2000", mid_time_et);

        let r = Quaternion::from_pitch_roll_yaw(90.0_f64.to_radians(), 0.0, 0.0);
        let p = Quaternion::from_pitch_roll_yaw(0.0, 90.0_f64.to_radians(), 0.0);
        let q = r.times(&p.times(&Quaternion::from_matrix(&midtime_matrix).invert()));

        let mut cyl_map = RgbImage::create(1024, 1024);
        //let lens = CylindricalLens::new(cyl_map.width, cyl_map.height, 90.0, -90.0, 0.0, 360.0);
        let lens = FisheyeEquisolidLens::new(cyl_map.width, cyl_map.height, 13.0, 180.0);


        for t in 0..raw_image.get_triplet_count() {
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
                        //vprintln!("Point: {:?}", tl);
                        cyl_map.paint_square(&tl, &bl, &br, &tr, false, 2 - s);

                    }
                }
            }
        }

        vprintln!("MinMax: {:?}", cyl_map.get_min_max_all_channel());
        //cyl_map.normalize_to_16bit();
        cyl_map.normalize_to_16bit_seperate_channels();
        vprintln!("MinMax: {:?}", cyl_map.get_min_max_all_channel());
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