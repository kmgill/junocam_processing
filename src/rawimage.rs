
use crate::{
    path, 
    triplet, 
    constants,
    enums
};

use sciimg::{
    imagebuffer::ImageBuffer, 
    enums::ImageMode,
    error
};


pub struct RawImage {
    pub rawdata : ImageBuffer,
    pub triplets : Vec<triplet::Triplet>
}


impl RawImage {

    pub fn new_from_image(raw_image_path:&str) -> error::Result<RawImage> {

        if ! path::file_exists(raw_image_path) {
            return Err(constants::status::FILE_NOT_FOUND);
        }

        let mut rawimage = RawImage{
            rawdata: match ImageBuffer::from_file(raw_image_path) {
                Ok(b) => b,
                Err(e) => { return Err(e); }
            },
            triplets: Vec::new()
        };

        rawimage.split_triplets();

        Ok(rawimage)
    }

    pub fn assemble(&self) -> ImageBuffer {
        let mut assembled_buffer = ImageBuffer::new_with_fill(self.rawdata.width, self.rawdata.height, 0.0).unwrap();
        assembled_buffer.mode = ImageMode::U16BIT;

        let mut y : usize = 0;
        for triplet in self.triplets.iter() {
            triplet.paste_into(&mut assembled_buffer, y).expect("Failed to paste into assembled buffer");
            y = y + constants::STRIP_HEIGHT * 3;
        }

        assembled_buffer
    }

    fn split_triplets(&mut self) {

        if self.triplets.len() > 0 {
            panic!("Triplets already split out");
        }
        let triplet_count = self.rawdata.height / (constants::STRIP_HEIGHT * 3);

        for i in 0..triplet_count {
            let triplet_data = self.rawdata.get_slice(i * (constants::STRIP_HEIGHT * 3), constants::STRIP_HEIGHT * 3).unwrap();
            let triplet = triplet::Triplet::new_from_imagebuffer(&triplet_data).unwrap();
            self.triplets.push(triplet);
        }
    }

    pub fn get_triplet_count(&self) -> u8 {
        self.triplets.len() as u8
    }

    pub fn apply_darknoise(&mut self)  -> error::Result<&'static str> {
        for triplet in self.triplets.iter_mut() {
            triplet.apply_darknoise().expect("Error adark/flat field correction");
        }

        Ok("ok")
    }

    pub fn apply_hot_pixel_correction(&mut self, hpc_window_size:i32, hpc_threshold:f32)  -> error::Result<&'static str> {

        for triplet in self.triplets.iter_mut() {
            triplet.apply_hot_pixel_correction(hpc_window_size, hpc_threshold).expect("Error applying hot pixel correction");
        }

        Ok("ok")
    }

    pub fn apply_infill_correction(&mut self)  -> error::Result<&'static str> {

        for triplet in self.triplets.iter_mut() {
            triplet.infill().expect("Error applying infill correction");
        }

        Ok("ok")
    }

    pub fn appy_decomanding(&mut self, ilttype:enums::SampleBitMode)  -> error::Result<&'static str> {

        for triplet in self.triplets.iter_mut() {
            triplet.decompand(ilttype).expect("Error applying decompanding");
        }

        Ok("ok")
    }

    pub fn apply_weights(&mut self, red_weight:f32, green_weight:f32, blue_weight:f32) -> error::Result<&'static str> {
        for triplet in self.triplets.iter_mut() {
            triplet.apply_weights(red_weight, green_weight, blue_weight).expect("Error applying decompanding");
        }

        Ok("ok")
    }


    
}