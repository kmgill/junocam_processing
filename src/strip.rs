
use crate::{imagebuffer::ImageBuffer, decompanding, constants, enums};


pub struct Strip {
    pub buffer : ImageBuffer,
    empty : bool,
}


impl Strip {
    pub fn decompand(&mut self) -> Result<&'static str, &'static str> {
        if self.empty {
            return Err(constants::status::STRUCT_IS_EMPTY)
        } 

        // Don't assume SQROOT
        decompanding::decompand_buffer(&mut self.buffer, enums::SampleBitMode::SQROOT) 
    }

    pub fn is_empty(&self) -> bool {
        self.empty
    }

    pub fn apply_weight(&mut self, weight:f32) -> Result<&'static str, &'static str> {
        if self.empty {
            return Err(constants::status::STRUCT_IS_EMPTY)
        } 

        self.buffer = self.buffer.scale(weight).unwrap();

        Ok(constants::status::OK)
    }
}