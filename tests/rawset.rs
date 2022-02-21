use junocam_processing::{
    path, 
    constants, 
    rawset
};

mod common;

#[test]
fn test_load_image_set() {

    // Make sure the test files exist
    assert!(path::file_exists(common::constants::TEST_RAW_IMAGE_FILE_PATH));
    assert!(path::file_exists(common::constants::TEST_JSON_FILE_PATH));

    // Open the test set
    let rs = rawset::RawSet::open(common::constants::TEST_JSON_FILE_PATH, common::constants::TEST_RAW_IMAGE_FILE_PATH).unwrap();

    // Split the raw into triplet and verify the count
    assert_eq!(rs.image.get_triplet_count(), 26);

    // Make sure the triplet count jives with what we'd 
    // predict from the metadata.
    let expected_triplets = rs.metadata.lines as u32 / (constants::STRIP_HEIGHT as u32 * 3);
    assert_eq!(rs.image.get_triplet_count() as u32, expected_triplets);
    
}
