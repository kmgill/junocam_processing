use spice;

use crate::{
    path,
    vprintln,
    config
};
use sciimg::error;
use sciimg::matrix::Matrix;

pub static JUNO : i32 = -61;

pub static JUNO_JUNOCAM_METHANE : i32 = -61504;
pub static JUNO_JUNOCAM_BLUE : i32 = -61501;
pub static JUNO_JUNOCAM : i32 = -61500;
pub static JUNO_JUNOCAM_GREEN : i32 = -61502;
pub static JUNO_JUNOCAM_RED  : i32 = -61503;


pub enum Channel {
    RED,
    GREEN,
    BLUE,
    METHANE
}

impl Channel {

    pub fn to_id(&self) -> i32 {
        match self {
            Channel::RED => JUNO_JUNOCAM_RED,
            Channel::GREEN => JUNO_JUNOCAM_GREEN,
            Channel::BLUE => JUNO_JUNOCAM_BLUE,
            Channel::METHANE => JUNO_JUNOCAM_METHANE
        }
    }
}


pub fn furnish(kernel_path:&str) -> error::Result<&str> {
    match path::locate_calibration_file(&kernel_path.to_string()) {
        Ok(f) => {
            vprintln!("Loading {}", f);
            spice::furnsh(&f);
            Ok("ok")
        },
        Err(why) => {
            eprintln!("Failed to locate kernel: {}", kernel_path);
            Err(why)
        }
    }
}

pub fn furnish_base() {
    match config::load_configuration() {
        Ok(c) => {

            for k in c.spice.kernels {
                furnish(&k.as_str()).expect("Failed to load spice kernel");
            }

        },
        Err(why) => {
            eprintln!("Failed to load configuration file: {}", why);
            panic!("Failed to load configuration file prior to base kernel loading");
        }
    }
}


pub fn string_to_et(s:&String) -> f64 {
    spice::str2et(&s.as_str())
}


trait MatrixFrom3x3 {
    fn from_3x3(m:&[[f64; 3]; 3]) -> Matrix;
}

impl MatrixFrom3x3 for Matrix {
    fn from_3x3(m:&[[f64; 3]; 3]) -> Matrix {
        Matrix::new_with_values(m[0][0], m[1][0], m[2][0], 0.0,
                                    m[0][1], m[1][1], m[2][1], 0.0,
                                    m[0][2], m[1][2], m[2][2], 0.0,
                                    0.0, 0.0, 0.0, 1.0)
            
    }
}

//spice::pxform
//spice::pxform("JUNO_JUNOCAM", "J2000", image_time_et);
pub fn pos_transform_matrix(from:&str, to:&str, et:f64) -> Matrix {
    let mtx = spice::pxform(from, to, et);
    Matrix::from_3x3(&mtx)
}
