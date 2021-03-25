
use crate::{strip::Strip, imagebuffer::ImageBuffer, decompanding, constants, enums, error};


pub struct Triplet {
    pub buffer : ImageBuffer,
    red : Strip,
    green : Strip,
    blue : Strip,
    empty : bool,
    // Will need timing & pointing
}

impl Triplet {

    pub fn new_from_imagebuffer(buffer:&ImageBuffer) -> error::Result<Triplet> {
        Ok(Triplet{
            buffer: buffer.clone(),
            red: Strip::new_empty().unwrap(),
            green: Strip::new_empty().unwrap(),
            blue: Strip::new_empty().unwrap(),
            empty: false
        })
    }

    pub fn is_empty(&self) -> bool {
        self.empty
    }

    pub fn decompand(&mut self) -> error::Result<&'static str> {
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

    pub fn apply_weights(&mut self, red_weight:f32, green_weight:f32, blue_weight:f32) -> error::Result<&'static str> {
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

    pub fn extract_triplet_from_buffer(&mut self) -> error::Result<&'static str> {

        let blue_data = self.buffer.get_slice(0, constants::STRIP_HEIGHT).unwrap();
        let green_data = self.buffer.get_slice(constants::STRIP_HEIGHT, constants::STRIP_HEIGHT).unwrap();
        let red_data = self.buffer.get_slice(2 * constants::STRIP_HEIGHT, constants::STRIP_HEIGHT).unwrap();

        self.blue = Strip::new_from_imagebuffer(&blue_data).unwrap();
        self.green = Strip::new_from_imagebuffer(&green_data).unwrap();
        self.red = Strip::new_from_imagebuffer(&red_data).unwrap();

        Ok(constants::status::OK)
    }
}