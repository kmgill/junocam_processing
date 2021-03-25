use junocam_processing::{path, constants, rawset};

const TEST_RAW_IMAGE_FILE_PATH : &str = "test-data/JNCE_2021052_32C00054_V01/ImageSet/JNCE_2021052_32C00054_V01-raw.png";
const TEST_JSON_FILE_PATH : &str = "test-data/JNCE_2021052_32C00054_V01/DataSet/10124-Metadata.json";

#[test]
fn test_load_image_set() {

    // Make sure the test files exist
    assert!(path::file_exists(TEST_RAW_IMAGE_FILE_PATH));
    assert!(path::file_exists(TEST_JSON_FILE_PATH));

    // Open the test set
    let mut rs = rawset::RawSet::open(TEST_JSON_FILE_PATH, TEST_RAW_IMAGE_FILE_PATH).unwrap();

    // Triplet count should be zero since we haven't split them out from
    // the raw image
    assert_eq!(rs.image.get_triplet_count(), 0);

    // Split the raw into triplet and verify the count
    rs.image.split_triplets().unwrap();
    assert_eq!(rs.image.get_triplet_count(), 26);

    // Make sure the triplet count jives with what we'd 
    // predict from the metadata.
    let expected_triplets = rs.metadata.lines as u32 / (constants::STRIP_HEIGHT as u32 * 3);
    assert_eq!(rs.image.get_triplet_count() as u32, expected_triplets);
    
}
