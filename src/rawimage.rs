
use crate::{
    imagebuffer::ImageBuffer, 
    print, 
    path, 
    triplet, 
    error, 
    constants
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

        let rawdata = ImageBuffer::from_file(raw_image_path).unwrap();

        Ok(RawImage{
            rawdata: rawdata,
            triplets: Vec::new()
        })
    }

    pub fn split_triplets(&mut self) -> error::Result<&str> {

        let triplet_count = self.rawdata.height / (constants::STRIP_HEIGHT * 3);

        for i in 0..triplet_count {
            let triplet_data = self.rawdata.get_slice(i * (constants::STRIP_HEIGHT * 3), constants::STRIP_HEIGHT * 3).unwrap();
            let mut triplet = triplet::Triplet::new_from_imagebuffer(&triplet_data).unwrap();
            triplet.extract_triplet_from_buffer().unwrap();
            self.triplets.push(triplet);
        }

        Ok(constants::status::OK)
    }

    pub fn get_triplet_count(&self) -> u8 {
        self.triplets.len() as u8
    }
}