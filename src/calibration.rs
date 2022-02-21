

fn determine_data_dir() -> String {
    if cfg!(debug_assertions) {
        String::from("src/calib")
    } else if cfg!(target_os = "macos") {
        String::from("/usr/local/share/junocamcal/data/")
    } else if cfg!(target_os = "windows") {
        String::from("junocamcal/caldata") // C:/something/something/something/darkside/
    } else {
        String::from("/usr/share/junocamcal/data/")
    }
}

pub fn locate_calibration_file(file_path:String) -> error::Result<String> {

    let mut fp = file_path;

    match env::var("JUNOCAM_DATA_HOME") {
        Ok(d) => {
            fp = format!("{}/{}", d, fp); 
        },
        Err(_) => {
            let d = determine_data_dir();
            fp = format!("{}/{}", d, fp);
        }
    };

    match path::file_exists(&fp) {
        true => Ok(fp),
        false => Err(constants::status::FILE_NOT_FOUND)
    }   
}