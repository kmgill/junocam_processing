use crate::{
    constants, 
    enums,
    cache
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
                Ok(CACHE.lock().unwrap().check_red(constants::cal::JNO_DARKFIELD_RED).unwrap()),
        enums::Camera::GREEN => 
                Ok(CACHE.lock().unwrap().check_green(constants::cal::JNO_DARKFIELD_GREEN).unwrap()), 
        enums::Camera::BLUE => 
                Ok(CACHE.lock().unwrap().check_blue(constants::cal::JNO_DARKFIELD_BLUE).unwrap()),
        _ => Err(constants::status::UNSUPPORTED_COLOR_CHANNEL)
    }
}