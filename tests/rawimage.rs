
use junocam_processing::{
    path, 
    constants, 
    rawimage, 
    metadata
};

mod common;

#[test]
fn test_load_image() {

    // Make sure the required files exist
    assert!(path::file_exists(common::constants::TEST_RAW_IMAGE_FILE_PATH));
    assert!(path::file_exists(common::constants::TEST_JSON_FILE_PATH));

    // Load test image
    let mut raw_image = rawimage::RawImage::new_from_image(common::constants::TEST_RAW_IMAGE_FILE_PATH).unwrap();

    // Triplet count should be zero since we haven't split them out from
    // the raw image
    assert_eq!(raw_image.get_triplet_count(), 0);

    // Split the raw image out into triplets (and individual strips
    // under the hood). Then check the count
    raw_image.split_triplets().unwrap();
    assert_eq!(raw_image.get_triplet_count(), 26);

    // Load the metadata file so we can try to predict the number
    // of triplets from that then check the actual count
    // from the raw image
    let md = metadata::Metadata::new_from_file(common::constants::TEST_JSON_FILE_PATH).unwrap();
    let expected_triplets = md.lines as u32 / (constants::STRIP_HEIGHT as u32 * 3);
    assert_eq!(raw_image.get_triplet_count() as u32, expected_triplets);
}