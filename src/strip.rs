
use crate::imagebuffer::ImageBuffer;
use crate::decompanding;
use crate::constants;

pub struct Strip {
    pub buffer : ImageBuffer,
    empty : bool,
}


impl Strip {
    pub fn decompand(&mut self) -> Result<&'static str, &'static str> {
        if self.empty {
            return Err(constants::STRUCT_IS_EMPTY)
        } 
        decompanding::decompand_buffer(&mut self.buffer)
    }

    pub fn is_empty(&self) -> bool {
        self.empty
    }

    pub fn apply_weight(&mut self, weight:f32) -> Result<&'static str, &'static str> {
        if self.empty {
            return Err(constants::STRUCT_IS_EMPTY)
        } 

        self.buffer = self.buffer.scale(weight).unwrap();

        Ok(constants::OK)
    }
}