use sciimg::{error, imagebuffer::ImageBuffer};

#[derive(Default)]
pub struct ImageCache {
    red: Option<ImageBuffer>,
    green: Option<ImageBuffer>,
    blue: Option<ImageBuffer>,
}

impl ImageCache {
    pub fn check_red(&mut self, path: &str) -> error::Result<ImageBuffer> {
        match &self.red {
            None => {
                self.red = Some(ImageBuffer::from_file(path).unwrap());
                Ok(self.red.as_ref().unwrap().to_owned())
            }
            Some(b) => Ok(b.to_owned()),
        }
    }

    pub fn check_green(&mut self, path: &str) -> error::Result<ImageBuffer> {
        match &self.green {
            None => {
                self.green = Some(ImageBuffer::from_file(path).unwrap());
                Ok(self.green.as_ref().unwrap().to_owned())
            }
            Some(b) => Ok(b.to_owned()),
        }
    }

    pub fn check_blue(&mut self, path: &str) -> error::Result<ImageBuffer> {
        match &self.blue {
            None => {
                self.blue = Some(ImageBuffer::from_file(path).unwrap());
                Ok(self.blue.as_ref().unwrap().to_owned())
            }
            Some(b) => Ok(b.to_owned()),
        }
    }
}
