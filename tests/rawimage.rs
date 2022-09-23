
use junocam::{
    path, 
    constants, 
    rawimage, 
    metadata,
    enums
};

mod common;

#[test]
fn test_load_image() {

    // Make sure the required files exist
    assert!(path::file_exists(common::constants::TEST_RAW_IMAGE_FILE_PATH));
    assert!(path::file_exists(common::constants::TEST_JSON_FILE_PATH));

    // Load test image
    let mut raw_image = rawimage::RawImage::new_from_image(common::constants::TEST_RAW_IMAGE_FILE_PATH).unwrap();

    // Split the raw image out into triplets (and individual strips
    // under the hood). Then check the count
    assert_eq!(raw_image.get_triplet_count(), 26);

    // Load the metadata file so we can try to predict the number
    // of triplets from that then check the actual count
    // from the raw image
    let md = metadata::Metadata::new_from_file(common::constants::TEST_JSON_FILE_PATH).unwrap();
    let expected_triplets = md.lines as u32 / (constants::STRIP_HEIGHT as u32 * 3);
    assert_eq!(raw_image.get_triplet_count() as u32, expected_triplets);

    // Image calibration routines. These will take a while in test
    raw_image.apply_infill_correction().expect("Error with infill correction");
    raw_image.appy_decomanding(enums::SampleBitMode::SQROOT).expect("Error with decompanding");
    raw_image.apply_darknoise().expect("Error with dark/flat field correction");
    raw_image.apply_hot_pixel_correction(5, 2.0).expect("Error wih hot pixel correction");
    raw_image.apply_weights(0.9, 0.9, 0.9).expect("Error applying channel weight values");


    // Reassemble all data back into a full strip stack
    let _assembled_final = raw_image.assemble();

    //_assembled_final.save("test.png", ImageMode::U16BIT);
}
