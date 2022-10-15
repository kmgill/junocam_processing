use crate::{path, vprintln};

use std::fs::File;
use std::io::Read;

use sciimg::error;

//use serde_derive::Deserialize;
use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct Defaults {
    pub red_weight: f32,
    pub green_weight: f32,
    pub blue_weight: f32,
    pub camera_lens_projection: String,
    pub fisheye_field_of_view: f64,
    pub interframe_delay_correction: f64,
    pub start_time_correction: f64,
    pub apply_calibration: bool,
    pub apply_infill_correction: bool,
    pub apply_hot_pixel_correction: bool,
    pub hpc_window_size: i32,
    pub hpc_threshold: f32,
    pub apply_weights: bool,
    pub correlated_color_balancing: bool,
}

#[derive(Deserialize, Clone)]
pub struct CalibrationFiles {
    pub dark_red: String,
    pub dark_green: String,
    pub dark_blue: String,

    pub flat_red: String,
    pub flat_green: String,
    pub flat_blue: String,

    pub inpaint_red: String,
    pub inpaint_green: String,
    pub inpaint_blue: String,
}

#[derive(Deserialize, Clone)]
pub struct Spice {
    pub kernels: Vec<String>,
    pub ck_rec_pattern: String,
    pub ck_pre_pattern: String,
    pub spk_rec_pattern: String,
    pub spk_pre_pattern: String,
    pub ck_nth_start_date: usize,
    pub ck_nth_end_date: usize,
    pub spk_nth_start_date: usize,
    pub spk_nth_end_date: usize,
}

#[derive(Deserialize, Clone)]
pub struct JunoConfig {
    pub spice: Spice,
    pub calibration: CalibrationFiles,
    pub defaults: Defaults,
}

static mut JUNO_CONFIG: Option<JunoConfig> = None;

pub fn load_configuration() -> error::Result<JunoConfig> {
    unsafe {
        if let Some(c) = &JUNO_CONFIG {
            return Ok(c.clone());
        }
    }

    let config_file_path = path::locate_calibration_file(&String::from("config.toml"));

    match config_file_path {
        Ok(config_toml) => {
            vprintln!("Loading configuration from {}", config_toml);

            let mut file = match File::open(&config_toml) {
                Err(why) => panic!("couldn't open {}", why),
                Ok(file) => file,
            };

            let mut buf: Vec<u8> = Vec::default();
            file.read_to_end(&mut buf).unwrap();
            let toml = String::from_utf8(buf).unwrap();

            unsafe {
                let config: JunoConfig = toml::from_str(&toml).unwrap();
                JUNO_CONFIG = Some(config);
                Ok(JUNO_CONFIG.clone().unwrap())
            }
        }
        Err(_) => {
            panic!("Unable to locate juno configuration file")
        }
    }
}
