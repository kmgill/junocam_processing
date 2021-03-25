
use junocam_processing::{path, constants, rawimage, metadata};

const TEST_RAW_IMAGE_FILE_PATH : &str = "test-data/JNCE_2021052_32C00054_V01/ImageSet/JNCE_2021052_32C00054_V01-raw.png";
const TEST_JSON_FILE_PATH : &str = "test-data/JNCE_2021052_32C00054_V01/DataSet/10124-Metadata.json";

#[test]
fn test_load_image() {

    // Make sure the required files exist
    assert!(path::file_exists(TEST_RAW_IMAGE_FILE_PATH));
    assert!(path::file_exists(TEST_JSON_FILE_PATH));

    // Load test image
    let mut raw_image = rawimage::RawImage::new_from_image(TEST_RAW_IMAGE_FILE_PATH).unwrap();

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
    let md = metadata::Metadata::new_from_file(TEST_JSON_FILE_PATH).unwrap();
    let expected_triplets = md.lines as u32 / (constants::STRIP_HEIGHT as u32 * 3);
    assert_eq!(raw_image.get_triplet_count() as u32, expected_triplets);
}