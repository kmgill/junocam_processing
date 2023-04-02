use crate::{cache, config, constants, enums, path};

use sciimg::{inpaint, prelude::*};
use std::sync::Mutex;

lazy_static! {
    static ref DARK_CACHE: Mutex<cache::ImageCache> = Mutex::new(cache::ImageCache::default());
    static ref FLAT_CACHE: Mutex<cache::ImageCache> = Mutex::new(cache::ImageCache::default());
    static ref MASK_CACHE: Mutex<cache::ImageCache> = Mutex::new(cache::ImageCache::default());
}

pub fn load_mask(camera: enums::Camera) -> error::Result<ImageBuffer> {
    let c = config::load_configuration().expect("Failed to load configuration");
    match camera {
        enums::Camera::RED => Ok(MASK_CACHE
            .lock()
            .unwrap()
            .check_red(&path::locate_calibration_file(&c.calibration.inpaint_red).unwrap())
            .unwrap()),
        enums::Camera::GREEN => Ok(MASK_CACHE
            .lock()
            .unwrap()
            .check_green(&path::locate_calibration_file(&c.calibration.inpaint_green).unwrap())
            .unwrap()),
        enums::Camera::BLUE => Ok(MASK_CACHE
            .lock()
            .unwrap()
            .check_blue(&path::locate_calibration_file(&c.calibration.inpaint_blue).unwrap())
            .unwrap()),
        _ => Err(constants::status::UNSUPPORTED_COLOR_CHANNEL),
    }
}

pub fn load_flat_file(camera: enums::Camera) -> error::Result<ImageBuffer> {
    let c = config::load_configuration().expect("Failed to load configuration");

    let flat = match camera {
        enums::Camera::RED => Ok(FLAT_CACHE
            .lock()
            .unwrap()
            .check_red(&path::locate_calibration_file(&c.calibration.flat_red).unwrap())
            .unwrap()),
        enums::Camera::GREEN => Ok(FLAT_CACHE
            .lock()
            .unwrap()
            .check_green(&path::locate_calibration_file(&c.calibration.flat_green).unwrap())
            .unwrap()),
        enums::Camera::BLUE => Ok(FLAT_CACHE
            .lock()
            .unwrap()
            .check_blue(&path::locate_calibration_file(&c.calibration.flat_blue).unwrap())
            .unwrap()),
        _ => Err(constants::status::UNSUPPORTED_COLOR_CHANNEL),
    }
    .unwrap();

    let mask = match load_mask(camera) {
        Ok(m) => m,
        Err(_) => return Err("Error loading mask"),
    };

    // Loading our grayscale data into a 3 band RgbImage. Will need to modify the sciimg inpaint method to take in imagebuffer
    let rgb = Image::new_from_buffers_rgb(&flat, &flat, &flat, ImageMode::U16BIT).unwrap();

    let filled = match inpaint::apply_inpaint_to_buffer(&rgb, &mask) {
        Ok(b) => b,
        Err(e) => return Err(e),
    };

    Ok(filled.get_band(0).clone())
}

pub fn load_dark_file(camera: enums::Camera) -> error::Result<ImageBuffer> {
    let c = config::load_configuration().expect("Failed to load configuration");

    let dark = match camera {
        enums::Camera::RED => Ok(DARK_CACHE
            .lock()
            .unwrap()
            .check_red(&path::locate_calibration_file(&c.calibration.dark_red).unwrap())
            .unwrap()),
        enums::Camera::GREEN => Ok(DARK_CACHE
            .lock()
            .unwrap()
            .check_green(&path::locate_calibration_file(&c.calibration.dark_green).unwrap())
            .unwrap()),
        enums::Camera::BLUE => Ok(DARK_CACHE
            .lock()
            .unwrap()
            .check_blue(&path::locate_calibration_file(&c.calibration.dark_blue).unwrap())
            .unwrap()),
        _ => Err(constants::status::UNSUPPORTED_COLOR_CHANNEL),
    }
    .unwrap();

    let mask = match load_mask(camera) {
        Ok(m) => m,
        Err(_) => return Err("Error loading mask"),
    };

    // Loading our grayscale data into a 3 band RgbImage. Will need to modify the sciimg inpaint method to take in imagebuffer
    let rgb = Image::new_from_buffers_rgb(&dark, &dark, &dark, ImageMode::U16BIT).unwrap();

    let filled = match inpaint::apply_inpaint_to_buffer(&rgb, &mask) {
        Ok(b) => b,
        Err(e) => return Err(e),
    };

    Ok(filled.get_band(0).clone())
}
