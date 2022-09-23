
use crate::{
    constants, 
    enums, 
    cache,
    decompanding as ilttables,
    path
};

use sciimg::{
    imagebuffer::ImageBuffer, 
    decompanding,
    inpaint,
    error, 
    enums::ImageMode,
    rgbimage::RgbImage,
    hotpixel
};

pub struct Strip {
    pub buffer : ImageBuffer,
    pub camera : enums::Camera,
    ilt_applied : bool,
    darknoise_applied : bool,
    infill_applied : bool,
    hpc_applied : bool
    // Strip should know which band it is along with timing and pointing
}

use std::sync::Mutex;

lazy_static! {
    static ref CACHE:Mutex<cache::ImageCache> = Mutex::new(cache::ImageCache::default());
}

pub fn determine_mask_file(instrument:enums::Camera) -> error::Result<&'static str> {
    match instrument {
        enums::Camera::RED => Ok(constants::cal::JNO_INPAINT_MASK_RED),
        enums::Camera::GREEN => Ok(constants::cal::JNO_INPAINT_MASK_GREEN),
        enums::Camera::BLUE => Ok(constants::cal::JNO_INPAINT_MASK_BLUE),
        _ => Err(constants::status::INVALID_ENUM_VALUE)
    }
}

pub fn inpaint_supported_for_camera(camera:enums::Camera) -> bool {
    let r = determine_mask_file(camera);
    match r {
        Ok(_) => true,
        Err(_) => false
    }
}

fn load_mask(camera:enums::Camera) -> error::Result<ImageBuffer> {
    match camera {
        enums::Camera::RED => 
                Ok(CACHE.lock().unwrap().check_red(&path::locate_calibration_file(&constants::cal::JNO_INPAINT_MASK_RED.to_string()).unwrap()).unwrap()),
        enums::Camera::GREEN => 
                Ok(CACHE.lock().unwrap().check_green(&path::locate_calibration_file(&constants::cal::JNO_INPAINT_MASK_GREEN.to_string()).unwrap()).unwrap()), 
        enums::Camera::BLUE => 
                Ok(CACHE.lock().unwrap().check_blue(&path::locate_calibration_file(&constants::cal::JNO_INPAINT_MASK_BLUE.to_string()).unwrap()).unwrap()),
        _ => Err(constants::status::UNSUPPORTED_COLOR_CHANNEL)
    }
}

fn load_flat_file(camera:enums::Camera) -> error::Result<ImageBuffer> {
    match camera {
        enums::Camera::RED => 
                Ok(CACHE.lock().unwrap().check_red(&path::locate_calibration_file(&constants::cal::JNO_FLATFIELD_RED.to_string()).unwrap()).unwrap()),
        enums::Camera::GREEN => 
                Ok(CACHE.lock().unwrap().check_green(&path::locate_calibration_file(&constants::cal::JNO_FLATFIELD_GREEN.to_string()).unwrap()).unwrap()), 
        enums::Camera::BLUE => 
                Ok(CACHE.lock().unwrap().check_blue(&path::locate_calibration_file(&constants::cal::JNO_FLATFIELD_BLUE.to_string()).unwrap()).unwrap()),
        _ => Err(constants::status::UNSUPPORTED_COLOR_CHANNEL)
    }
}



fn load_dark_file(camera:enums::Camera) -> error::Result<ImageBuffer> {
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


impl Strip {

    pub fn new_from_imagebuffer(buffer:&ImageBuffer, camera:enums::Camera) -> error::Result<Strip> {
        Ok(Strip{
            buffer: buffer.clone(),
            camera: camera,
            ilt_applied : false,
            darknoise_applied : false,
            infill_applied : false,
            hpc_applied : false
        })
    }

    pub fn apply_darknoise(&mut self)  -> error::Result<&'static str> {

        if self.darknoise_applied {
            return Err("Dark/Noise calibration already applied");
        }

        let dark = match load_dark_file(self.camera) {
            Ok(m) => m,
            Err(_) => return Err("Error loading dark field")
        };

        let flat = match load_flat_file(self.camera) {
            Ok(m) => m,
            Err(_) => return Err("Error loading flat field")
        };


        let darkflat = flat.subtract(&dark).unwrap();
        let mean_flat = darkflat.mean();
        let frame_minus_dark = self.buffer.subtract(&dark).unwrap();
        self.buffer = frame_minus_dark.scale(mean_flat).unwrap().divide(&flat).unwrap();

        self.darknoise_applied = true;

        Ok("ok")
    }

    pub fn paste_into(&self, into:&mut ImageBuffer, y:usize) {
        into.paste_mut(&self.buffer, 0, y);
    }

    pub fn infill(&mut self) -> error::Result<&'static str> {
        if self.infill_applied {
            return Err("Infill correction already applied");
        }

        let mask = match load_mask(self.camera) {
            Ok(m) => m,
            Err(_) => return Err("Error loading mask")
        };

        // Loading our grayscale data into a 3 band RgbImage. Will need to modify the sciimg inpaint method to take in imagebuffer
        let rgb = RgbImage::new_from_buffers_rgb(&self.buffer, &self.buffer, &self.buffer, ImageMode::U16BIT).unwrap();
        
        let filled = match inpaint::apply_inpaint_to_buffer(&rgb, &mask) {
            Ok(b) => b,
            Err(e) => return Err(e)
        };

        self.buffer = filled.get_band(0).clone();

        self.infill_applied = true;

        Ok("ok")
    }

    pub fn decompand(&mut self, ilttype:enums::SampleBitMode) -> error::Result<&'static str> {
        if self.ilt_applied {
            return Err("ILT decompression already applied");
        }

        self.buffer.clip_mut(0.0, 255.0);

        let ilttable = match ilttype {
            enums::SampleBitMode::SQROOT => ilttables::SQROOT,
            enums::SampleBitMode::LIN1 => ilttables::LIN1,
            enums::SampleBitMode::LIN8 => ilttables::LIN8,
            enums::SampleBitMode::LIN16 => ilttables::LIN16,
            enums::SampleBitMode::UNKNOWN => {
                return Err("Unknown/unsupported ILT, cannot decompand");
            }
        };

        // Don't assume SQROOT
        decompanding::decompand_buffer(&mut self.buffer, &ilttable);

        self.ilt_applied = true;
        Ok("ok")
    }   

    pub fn apply_hot_pixel_correction(&mut self, window_size:i32, threshold:f32) -> error::Result<&'static str> {
        if self.hpc_applied {
            return Err("Hot pixel correction already applied");
        }

        match hotpixel::hot_pixel_detection(&self.buffer, window_size, threshold) {
            Ok(r) => {
                self.buffer = r.buffer;
                self.hpc_applied = true;
                Ok("ok")
            },
            Err(e) => Err(e)
        }
    }

    pub fn apply_weight(&mut self, weight:f32) -> error::Result<&'static str>  {
        self.buffer = self.buffer.scale(weight).unwrap();

        Ok(constants::status::OK)
    }
}