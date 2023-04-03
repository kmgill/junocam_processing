use crate::constants;
use sciimg::error;
use sciimg::path;
use std::env;

pub fn locate_calibration_file(file_path: &String) -> error::Result<String> {
    // If the file exists as-is, return it
    if path::file_exists(file_path) {
        return Ok(file_path.clone());
    }

    // Some default locations
    let mut locations = vec![
        String::from("src/calib/"), // Running within the repo directory (dev: cargo run --bin ...)
        String::from("/usr/share/junocam/data/"), // Linux, installed via apt or rpm
    ];

    if let Ok(exe_path) = std::env::current_exe() {
        if cfg!(windows) {
            // I'm not even a little comfortable with this...
            // So, to figure out the installation path, we get the path to the running executable, then get the path, and then
            // append 'data' to it to get to the calibration files. We also have to get rid of those quotation marks.
            if let Some(filename) = exe_path.parent() {
                locations.insert(
                    0,
                    format!("{:?}", filename.with_file_name("data").as_os_str()).replace('\"', ""),
                );
            }
        }
    }

    // Allow for a custom data path to be defined during build.
    if let Some(v) = option_env!("JUNODATAROOT") {
        locations.insert(0, String::from(v));
    }

    // Spice, Juno specific
    if let Some(v) = option_env!("JUNOBASE") {
        locations.insert(0, String::from(v));
    }

    // Add a path based on the location of the running executable
    // Intended for Windows installations
    if let Ok(exe_path) = std::env::current_exe() {
        if cfg!(windows) {
            let bn = format!("{:?}/../data/", exe_path.file_name());
            locations.insert(0, bn);
        }
    }

    // Prepend a home directory if known
    if let Some(dir) = dirs::home_dir() {
        let homedatadir = format!("{}/.junodata", dir.to_str().unwrap());
        locations.insert(0, homedatadir);
    }

    // Prepend a location specified by environment variable
    if let Ok(dir) = env::var("JUNO_DATA") {
        locations.insert(0, dir);
    }

    // First match wins
    for loc in locations.iter() {
        let full_file_path = format!("{}/{}", loc, file_path);
        if path::file_exists(&full_file_path) {
            return Ok(full_file_path);
        }
    }

    // Oh nos!
    Err(constants::status::FILE_NOT_FOUND)
}
