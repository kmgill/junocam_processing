use spice;

use crate::{config, filelocate, veprintln, vprintln};
use sciimg::matrix::Matrix;

use anyhow::anyhow;
use anyhow::Result;

use chrono::{TimeZone, Utc};
use glob::glob;
use std::path::Path;

pub static JUNO: i32 = -61;

pub static JUNO_JUNOCAM_METHANE: i32 = -61504;
pub static JUNO_JUNOCAM_BLUE: i32 = -61501;
pub static JUNO_JUNOCAM: i32 = -61500;
pub static JUNO_JUNOCAM_GREEN: i32 = -61502;
pub static JUNO_JUNOCAM_RED: i32 = -61503;

pub enum Channel {
    RED,
    GREEN,
    BLUE,
    METHANE,
}

impl Channel {
    pub fn to_id(&self) -> i32 {
        match self {
            Channel::RED => JUNO_JUNOCAM_RED,
            Channel::GREEN => JUNO_JUNOCAM_GREEN,
            Channel::BLUE => JUNO_JUNOCAM_BLUE,
            Channel::METHANE => JUNO_JUNOCAM_METHANE,
        }
    }
}

pub fn furnish(kernel_path: &str) -> Result<&str> {
    match filelocate::locate_calibration_file(&kernel_path.to_string()) {
        Ok(f) => {
            vprintln!("Loading {}", f);
            spice::furnsh(&f);
            Ok("ok")
        }
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
                furnish(k.as_str()).expect("Failed to load spice kernel");
            }
        }
        Err(why) => {
            eprintln!("Failed to load configuration file: {}", why);
            panic!("Failed to load configuration file prior to base kernel loading");
        }
    }
}

fn kernel_name_date_to_et(name_date: &String) -> Result<f64> {
    match Utc.datetime_from_str(&format!("{} 00:00:00", name_date), "%y%m%d %T") {
        Ok(dt) => {
            let dt_reformatted = dt.format("%Y-%h-%d %H:%M:%S%.3f").to_string();
            Ok(string_to_et(&dt_reformatted))
        }
        Err(why) => {
            veprintln!("Error: {:?}  -- '{}'", why, &name_date.as_str());
            Err(anyhow!("Failed to parse kernel datetime"))
        }
    }
}

fn kernel_name_nth_part(ck_file: &String, n: usize) -> Option<String> {
    // Kinda hacky... (this whole thing is 'kinda' hacky...)
    let path = Path::new(&ck_file);
    if let Some(s) = Path::new(path.file_name().unwrap()).file_stem() {
        let filename = s.to_str().unwrap().to_string();
        let mut split = filename.split('_');
        Some(split.nth(n).unwrap().to_string())
    } else {
        None
    }
}

fn get_kernel_range_et(ck_file: &String) -> Option<(f64, f64)> {
    let start_date_s = kernel_name_nth_part(ck_file, 3).unwrap();
    let end_date_s = kernel_name_nth_part(ck_file, 4).unwrap();

    let kernel_start_et = kernel_name_date_to_et(&start_date_s).unwrap();
    let kernel_end_et = kernel_name_date_to_et(&end_date_s).unwrap();

    Some((kernel_start_et, kernel_end_et))
}

pub fn find_kernel_with_date(search_pattern: &String, time_et: f64) -> Result<String> {
    match option_env!("JUNOBASE") {
        Some(v) => {
            let abs_search_pattern = format!("{}/{}", v, search_pattern);
            vprintln!("spice search pattern: {}", abs_search_pattern);

            for entry in glob(&abs_search_pattern).expect("Failed to read glob pattern") {
                match entry {
                    Ok(path) => {
                        if let Some(range) =
                            get_kernel_range_et(&path.to_str().unwrap().to_string())
                        {
                            if range.0 <= time_et && time_et <= (range.1 + 86400.0) {
                                return Ok(path.to_str().unwrap().to_string());
                            }
                        }
                    }
                    Err(e) => vprintln!("{:?}", e),
                }
            }

            Err(anyhow!("Matching kernel not found"))
        }
        None => Err(anyhow!("JUNOBASE not specified")),
    }
}

pub fn string_to_et(s: &str) -> f64 {
    spice::str2et(s)
}

trait MatrixFrom3x3 {
    fn from_3x3(m: &[[f64; 3]; 3]) -> Matrix;
}

impl MatrixFrom3x3 for Matrix {
    fn from_3x3(m: &[[f64; 3]; 3]) -> Matrix {
        Matrix::new_with_values(
            m[0][0], m[1][0], m[2][0], 0.0, m[0][1], m[1][1], m[2][1], 0.0, m[0][2], m[1][2],
            m[2][2], 0.0, 0.0, 0.0, 0.0, 1.0,
        )
    }
}

//spice::pxform
//spice::pxform("JUNO_JUNOCAM", "J2000", image_time_et);
pub fn pos_transform_matrix(from: &str, to: &str, et: f64) -> Matrix {
    let mtx = spice::pxform(from, to, et);
    Matrix::from_3x3(&mtx)
}
