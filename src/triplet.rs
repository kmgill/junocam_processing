
use crate::{strip::Strip, imagebuffer::ImageBuffer, decompanding, constants, enums};


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
            return Err(constants::status::STRUCT_IS_EMPTY);
        } 
        
        // Don't assume SQROOT
        decompanding::decompand_buffer(&mut self.buffer, enums::SampleBitMode::SQROOT).unwrap();

        // Think about throwing an error if one or more strips are empty, too
        if !self.red.is_empty() {
            self.red.decompand().unwrap();
        }
        if !self.green.is_empty() {
            self.green.decompand().unwrap();
        }
        if !self.blue.is_empty() {
            self.blue.decompand().unwrap();
        }
        
        Ok(constants::status::OK)
    }

    pub fn apply_weights(&mut self, red_weight:f32, green_weight:f32, blue_weight:f32) -> Result<&'static str, &'static str> {
        // Think about throwing an error if one or more strips are empty.
        if !self.red.is_empty() {
            self.red.apply_weight(red_weight).unwrap();
        }
        if !self.green.is_empty() {
            self.green.apply_weight(green_weight).unwrap();
        }
        if !self.blue.is_empty() {
            self.blue.apply_weight(blue_weight).unwrap();
        }
        
        Ok(constants::status::OK)

    }

    pub fn extract_triplet_from_buffer() -> Result<&'static str, &'static str> {

        Err(constants::status::NOT_IMPLEMENTED)
    }
}