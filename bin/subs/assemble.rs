use crate::subs::runnable::RunnableSubcommand;

use junocam::junocam::FrameletParameters;
use junocam::{
    path,
    rawimage,
    junocam as jc,
    strip::Strip,
    jcspice,
    enums::SampleBitMode,
    metadata,
    drawable,
    drawable::Drawable,
    drawable::Point
};

use junocam::vprintln;

use std::process;

use sciimg::prelude::*;
use sciimg::vector::Vector;
use sciimg::matrix::Matrix;
use std::fs;


pub struct LatLon{
    lat:f64,
    lon:f64
}

trait VectorToCylindrical {
    fn to_cylindrical(&self) -> LatLon;
    fn to_xy(&self, top_lat:f64, bottom_lat:f64, left_lon:f64, right_lon:f64, image_width:usize, image_height:usize) -> drawable::Point;
}

impl VectorToCylindrical for Vector {
    fn to_cylindrical(&self) -> LatLon {
        LatLon{
            lat:self.z.atan2((self.x * self.x + self.y * self.y).sqrt()).to_degrees(),
            lon:self.y.atan2(self.x).to_degrees() + 180.0
        }
    }

    fn to_xy(&self, top_lat:f64, bottom_lat:f64, left_lon:f64, right_lon:f64, image_width:usize, image_height:usize) -> drawable::Point {
        let ll = self.to_cylindrical();
        
        let lat = ll.lat;
        let lon = ll.lon;
        
        let mut out_y_f = (lat - bottom_lat) / (top_lat - bottom_lat) * image_height as f64;
        let mut out_x_f = (lon - left_lon) / (right_lon - left_lon) * image_width as f64;
        
        while out_y_f < 0.0 {
            out_y_f += image_height as f64;
        }

        while out_y_f >= image_height as f64 {
            out_y_f -= image_height as f64;
        }

        while out_x_f < 0.0 {
            out_x_f += image_width as f64;
        }

        while out_x_f >= image_width as f64 {
            out_x_f -= image_width as f64;
        }

        drawable::Point {
            x: out_x_f,
            y: out_y_f,
            v: 0.0
        }
    }
}


#[derive(clap::Args)]
#[clap(author, version, about = "Assemble triplets to cylindrical map", long_about = None)]
pub struct Assemble {
    #[clap(long, short, help = "Input image")]
    input: String,

    #[clap(long, short, help = "Input metadata json")]
    metadata: String,

    #[clap(long, short, help = "Output image")]
    output: String,
}   


fn xy_to_map_point(x:usize, y:usize, framelet:&FrameletParameters, spc_mtx:&Matrix, cyl_map:&RgbImage, strip:&Strip) -> Point {
    let mut v = framelet.xy_to_vector(x as f64, y as f64);
    v = spc_mtx.multiply_vector(&v);
    v = Vector::new(v.x, v.z, v.y);
    let mut tl = v.to_xy(90.0, -90.0, 0.0, 360.0, cyl_map.width, cyl_map.height);
    let tl_v = strip.buffer.get(x, y).unwrap() as f64;
    tl.v = tl_v;
    tl
}

impl RunnableSubcommand for Assemble {
    fn run(&self) {
        if ! path::file_exists(&self.input) {
            eprintln!("ERROR: Input file not found!");
            process::exit(1);
        }

        vprintln!("Loading metadata from {}", self.metadata);
        let md = metadata::Metadata::new_from_file(&self.metadata).expect("Failed to load input metadata");

        vprintln!("Loading image file from {}", self.input);
        vprintln!("Decompanding with table '{:?}'", md.sample_bit_mode_id);
        let mut raw_image = rawimage::RawImage::new_from_image_with_decompand(&self.input, md.sample_bit_mode_id).unwrap();

        //raw_image.apply_darknoise().expect("Error with dark/flat field correction");
        raw_image.apply_infill_correction().expect("Error with infill correction");
        raw_image.apply_hot_pixel_correction(5, 2.0).expect("Error wih hot pixel correction");
        raw_image.apply_weights(0.902, 1.0, 1.8879).expect("Error applying channel weight values");
    
        jcspice::furnish_base();
        jcspice::furnish("kernels/spk/spk_rec_210127_210321_210329.bsp").expect("Failed to load spice kernel");
        jcspice::furnish("kernels/ck/juno_sc_rec_210220_210222_v01.bc").expect("Failed to load spice kernel");
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
        let _midtime_matrix = jcspice::pos_transform_matrix("JUNO_JUNOCAM", "J2000", mid_time_et);

        let mut cyl_map = RgbImage::create(4096, 2048);

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
                        let tl = xy_to_map_point(x, y, &framelet, &spc_mtx, &cyl_map, strip);
                        let bl = xy_to_map_point(x, y+1, &framelet, &spc_mtx, &cyl_map, strip);
                        let br = xy_to_map_point(x+1, y+1, &framelet, &spc_mtx, &cyl_map, strip);
                        let tr = xy_to_map_point(x+1, y, &framelet, &spc_mtx, &cyl_map, strip);

                        cyl_map.paint_square(&tl, &bl, &br, &tr, false, 2 - s);

                    }
                }
            }
        }

        vprintln!("MinMax: {:?}", cyl_map.get_min_max_all_channel());
        cyl_map.normalize_to_16bit();
        vprintln!("MinMax: {:?}", cyl_map.get_min_max_all_channel());
        cyl_map.save(&self.output);
    }
}