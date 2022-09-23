use crate::{
    constants, 
    enums,
    cache,
    path
};

use sciimg::{
    imagebuffer::ImageBuffer, 
    error, 
};

use std::sync::Mutex;

lazy_static! {
    static ref CACHE:Mutex<cache::ImageCache> = Mutex::new(cache::ImageCache::default());
}



pub fn load_dark(camera:enums::Camera) -> error::Result<ImageBuffer> {
    match camera {
        enums::Camera::RED => 
                Ok(CACHE.lock().unwrap().check_red(&path::locate_calibration_file(&constants::cal::JNO_DARKFIELD_RED.to_string()).unwrap()).unwrap()),
        enums::Camera::GREEN => 
                Ok(CACHE.lock().unwrap().check_green(&path::locate_calibration_file(&constants::cal::JNO_DARKFIELD_GREEN.to_string()).unwrap()).unwrap()), 
        enums::Camera::BLUE => 
                Ok(CACHE.lock().unwrap().check_blue(&path::locate_calibration_file(&constants::cal::JNO_DARKFIELD_BLUE.to_string()).unwrap()).unwrap()),
        _ => Err(constants::status::UNSUPPORTED_COLOR_CHANNEL)
    }
}