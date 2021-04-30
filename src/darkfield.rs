use crate::{
    constants, 
    imagebuffer::ImageBuffer, 
    error, 
    enums
};

pub fn load_dark(camera:enums::Camera) -> error::Result<ImageBuffer> {
    match camera {
        enums::Camera::RED => 
                Ok(ImageBuffer::from_file(constants::cal::JNO_DARKFIELD_RED).unwrap()),
        enums::Camera::GREEN => 
                Ok(ImageBuffer::from_file(constants::cal::JNO_DARKFIELD_GREEN).unwrap()), 
        enums::Camera::BLUE => 
                Ok(ImageBuffer::from_file(constants::cal::JNO_DARKFIELD_BLUE).unwrap()),
        _ => Err(constants::status::UNSUPPORTED_COLOR_CHANNEL)
    }
}