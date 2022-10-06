
use std::path::Path;
use sciimg::error;
use std::env;
use crate::constants;

// Checks if file exists.
pub fn file_exists(chk_path:&str) -> bool {
    Path::new(&chk_path).exists()
}

pub fn file_writable(chk_path:&str) -> bool {
    let path = Path::new(&chk_path);
    !path.metadata().unwrap().permissions().readonly()
}

pub fn get_parent(chk_path:&str) -> String {
    let path = Path::new(&chk_path);
    let parent = path.parent().unwrap();
    String::from(parent.to_str().unwrap())
}

pub fn parent_exists(chk_path:&str) -> bool {
    let parent = get_parent(chk_path);
    file_exists(parent.as_str())
}

pub fn parent_writable(chk_path:&str) -> bool {
    let parent = get_parent(chk_path);
    file_writable(parent.as_str())
}

pub fn parent_exists_and_writable(chk_path:&str) -> bool {
    parent_exists(chk_path) && parent_writable(chk_path)
}


pub fn locate_calibration_file(file_path:&String) -> error::Result<String> {

    // If the file exists as-is, return it
    if file_exists(&file_path) {
        return Ok(file_path.clone());
    }

    // Some default locations
    let mut locations = vec![
        String::from("src/calib/"), // Running within the repo directory (dev: cargo run --bin ...)
        String::from("/usr/share/junocam/data/") // Linux, installed via apt or rpm
    ];

    match std::env::current_exe() {
        Ok(exe_path) => {
            if cfg!(windows) {
                // I'm not even a little comfortable with this...
                // So, to figure out the installation path, we get the path to the running executable, then get the path, and then 
                // append 'data' to it to get to the calibration files. We also have to get rid of those quotation marks.
                if let Some(filename) = exe_path.parent() {
                    locations.insert(0, format!("{:?}", filename.with_file_name("data").as_os_str()).replace("\"", ""));
                }
                
            }
        },
        Err(_) => { }
    };

    // Allow for a custom data path to be defined during build. 
    match option_env!("JUNODATAROOT") {
        Some(v) => locations.insert(0, String::from(v)),
        None => {}
    };    

    // Spice, Juno specific
    match option_env!("JUNOBASE") {
        Some(v) => locations.insert(0, String::from(v)),
        None => {}
    };    

    // Add a path based on the location of the running executable
    // Intended for Windows installations
    match std::env::current_exe() {
        Ok(exe_path) => {
            if cfg!(windows) {
                let bn = format!("{:?}/../data/", exe_path.file_name());
                locations.insert(0, bn);
            }
        },
        Err(_) => { }
    };

    // Prepend a home directory if known
    match dirs::home_dir() {
        Some(dir) => {
            let homedatadir = format!("{}/.junodata", dir.to_str().unwrap());
            locations.insert(0, homedatadir);
        },
        None => {}
    };

    // Prepend a location specified by environment variable 
    match env::var("JUNO_DATA") {
        Ok(dir) => {
            locations.insert(0, dir);
        },
        Err(_) => { }
    };
    
    // First match wins
    for loc in locations.iter() {
        let full_file_path = format!("{}/{}", loc, file_path);
        if file_exists(&full_file_path) {
            return Ok(full_file_path);
        }
    }

    // Oh nos!
    Err(constants::status::FILE_NOT_FOUND)
}