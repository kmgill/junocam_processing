use crate::{constants, metadata, rawimage};

use anyhow::anyhow;
use anyhow::Result;
use sciimg::path;

pub struct RawSet {
    pub image: rawimage::RawImage,
    pub metadata: metadata::Metadata,
}

/* Represents the raw image and metadata sets
 *
 */
impl RawSet {
    pub fn open(metadata_path: &str, image_path: &str) -> Result<RawSet> {
        if !path::file_exists(metadata_path) {
            return Err(anyhow!(constants::status::FILE_NOT_FOUND));
        }

        if !path::file_exists(image_path) {
            return Err(anyhow!(constants::status::FILE_NOT_FOUND));
        }

        Ok(RawSet {
            image: rawimage::RawImage::new_from_image(image_path).unwrap(),
            metadata: metadata::Metadata::new_from_file(metadata_path).unwrap(),
        })
    }
}
