
use crate::{imagebuffer::ImageBuffer, decompanding, constants, enums, error};


pub struct Strip {
    pub buffer : ImageBuffer,
    empty : bool,
    // Strip should know which band it is along with timing and pointing
}


impl Strip {
    pub fn new_empty() -> error::Result<Strip> {
        Ok(Strip {
            buffer :ImageBuffer::new_empty().unwrap(),
            empty: true
        })
    }

    pub fn new_from_imagebuffer(buffer:&ImageBuffer) -> error::Result<Strip> {
        Ok(Strip{
            buffer: buffer.clone(),
            empty: false
        })
    }

    pub fn decompand(&mut self) -> error::Result<&'static str> {
        if self.empty {
            return Err(constants::status::STRUCT_IS_EMPTY)
        } 

        // Don't assume SQROOT
        decompanding::decompand_buffer(&mut self.buffer, enums::SampleBitMode::SQROOT) 
    }

    pub fn is_empty(&self) -> bool {
        self.empty
    }

    pub fn apply_weight(&mut self, weight:f32) -> error::Result<&'static str> {
        if self.empty {
            return Err(constants::status::STRUCT_IS_EMPTY)
        } 

        self.buffer = self.buffer.scale(weight).unwrap();

        Ok(constants::status::OK)
    }
}