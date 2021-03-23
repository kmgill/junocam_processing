
use crate::strip::Strip;
use crate::imagebuffer::ImageBuffer;
use crate::decompanding;
use crate::constants;

pub struct Triplet {
    pub buffer : ImageBuffer,
    red : Strip,
    green : Strip,
    blue : Strip,
    empty : bool,
}

impl Triplet {
    pub fn decompand(&mut self) -> Result<&'static str, &'static str> {
        if self.empty {
            return Err(constants::STRUCT_IS_EMPTY);
        } 
        
        decompanding::decompand_buffer(&mut self.buffer).unwrap();

        if !self.red.is_empty() {
            self.red.decompand().unwrap();
        }
        if !self.green.is_empty() {
            self.green.decompand().unwrap();
        }
        if !self.blue.is_empty() {
            self.blue.decompand().unwrap();
        }
        
        Ok(constants::OK)
    }
}