
use crate::{
    strip::Strip, 
    constants, 
    enums,
    vprintln
};

use sciimg::{ 
    imagebuffer::ImageBuffer, 
    error
};


pub struct Triplet {
    pub buffer : ImageBuffer,
    pub channels : Vec<Strip>
    // Will need timing & pointing
}

const RED_CHANNEL : usize = 2;
const GREEN_CHANNEL : usize = 1;
const BLUE_CHANNEL : usize = 0;

impl Triplet {

    pub fn new_from_imagebuffer(buffer:&ImageBuffer) -> error::Result<Triplet> {

        let mut new_triplet = Triplet{
            buffer: buffer.clone(),
            channels: Vec::with_capacity(3)
        };

        let red_data = new_triplet.buffer.get_slice(2 * constants::STRIP_HEIGHT, constants::STRIP_HEIGHT).unwrap();
        let green_data = new_triplet.buffer.get_slice(constants::STRIP_HEIGHT, constants::STRIP_HEIGHT).unwrap();
        let blue_data = new_triplet.buffer.get_slice(0, constants::STRIP_HEIGHT).unwrap();
        
        new_triplet.channels.push(Strip::new_from_imagebuffer(&blue_data, enums::Camera::BLUE).unwrap());
        new_triplet.channels.push(Strip::new_from_imagebuffer(&green_data, enums::Camera::GREEN).unwrap());
        new_triplet.channels.push(Strip::new_from_imagebuffer(&red_data, enums::Camera::RED).unwrap());

        Ok(new_triplet)
    }

    pub fn paste_into(&self, into:&mut ImageBuffer, y:usize) -> error::Result<&'static str>  {
        if self.channels.len() != 3 {
            return  Err("Empty data, cannot paste");
        }
        self.channels[BLUE_CHANNEL].paste_into(into, y);
        self.channels[GREEN_CHANNEL].paste_into(into, y + constants::STRIP_HEIGHT);
        self.channels[RED_CHANNEL].paste_into(into, y + constants::STRIP_HEIGHT + constants::STRIP_HEIGHT);

        Ok("ok")
    }

    pub fn apply_darknoise(&mut self)  -> error::Result<&'static str> {
        for i in self.channels.iter_mut() {
            match i.apply_darknoise() {
                Ok(_) => {},
                Err(e) => { return Err(e); }
            }
        }

        Ok(constants::status::OK)
    }

    pub fn infill(&mut self) -> error::Result<&'static str> {
        for i in self.channels.iter_mut() {
            match i.infill() {
                Ok(_) => {},
                Err(e) => { return Err(e); }
            }
        }

        Ok(constants::status::OK)
    }

    pub fn apply_hot_pixel_correction(&mut self, window_size:i32, threshold:f32) -> error::Result<&'static str> {
        for i in self.channels.iter_mut() {
            match i.apply_hot_pixel_correction(window_size, threshold) {
                Ok(_) => {},
                Err(e) => { return Err(e); }
            }
        }

        Ok(constants::status::OK)
    }

    pub fn decompand(&mut self, ilttype:enums::SampleBitMode) -> error::Result<&'static str> {
        for i in self.channels.iter_mut() {
            match i.decompand(ilttype) {
                Ok(_) => {},
                Err(e) => { return Err(e); }
            }
        }

        Ok(constants::status::OK)
    }

    pub fn apply_weights(&mut self, red_weight:f32, green_weight:f32, blue_weight:f32) -> error::Result<&'static str> {
        match self.channels[RED_CHANNEL].apply_weight(red_weight) {
            Ok(_) => {},
            Err(e) => { return Err(e); }
        }

        match self.channels[GREEN_CHANNEL].apply_weight(green_weight) {
            Ok(_) => {},
            Err(e) => { return Err(e); }
        }


        match self.channels[BLUE_CHANNEL].apply_weight(blue_weight) {
            Ok(_) => {},
            Err(e) => { return Err(e); }
        }
        
        Ok(constants::status::OK)

    }


}