use crate::{
    path
};

use std::fs::File;
use std::io::Read;

use sciimg::error;

//use serde_derive::Deserialize;
use serde::{
    Deserialize
};

#[derive(Deserialize)]
pub struct Defaults {
    pub red_weight: f32,
    pub green_weight: f32,
    pub blue_weight: f32,
    pub interframe_delay_correction: f64,
    pub start_time_correction: f64
}

#[derive(Deserialize)]
pub struct CalibrationFiles {
    pub dark_red: String,
    pub dark_green: String,
    pub dark_blue: String,

    pub flat_red: String,
    pub flat_green: String,
    pub flat_blue: String,

    pub inpaint_red: String,
    pub inpaint_green: String,
    pub inpaint_blue: String
}

#[derive(Deserialize)]
pub struct Spice {
    pub kernels : Vec<String>,
    pub ck_rec_pattern: String,
    pub ck_pre_pattern: String
}

#[derive(Deserialize)]
pub struct JunoConfig {
    pub spice: Spice,
    pub calibration: CalibrationFiles,
    pub defaults: Defaults
}

pub fn load_configuration() -> error::Result<JunoConfig> {

    let confile_file_path = path::locate_calibration_file(&String::from("config.toml"));

    match confile_file_path {
        Ok(config_toml) => {
            let mut file = match File::open(&config_toml) {
                Err(why) => panic!("couldn't open {}", why),
                Ok(file) => file,
            };
        
            let mut buf : Vec<u8> = Vec::default();
            file.read_to_end(&mut buf).unwrap();
            let toml = String::from_utf8(buf).unwrap();
        
            let config: JunoConfig = toml::from_str(&toml).unwrap();
        
            Ok(config)
        },
        Err(_) => {
            panic!("Unable to locate juno configuration file")
        }
    }
}