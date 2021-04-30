use crate::{
    constants, 
    imagebuffer::ImageBuffer, 
    error, 
    enums
};

pub fn load_flat(camera:enums::Camera) -> error::Result<ImageBuffer> {
    match camera {
        enums::Camera::RED => 
                Ok(ImageBuffer::from_file(constants::cal::JNO_FLATFIELD_RED).unwrap()),
        enums::Camera::GREEN => 
                Ok(ImageBuffer::from_file(constants::cal::JNO_FLATFIELD_GREEN).unwrap()), 
        enums::Camera::BLUE => 
                Ok(ImageBuffer::from_file(constants::cal::JNO_FLATFIELD_BLUE).unwrap()),
        _ => Err(constants::status::UNSUPPORTED_COLOR_CHANNEL)
    }
}