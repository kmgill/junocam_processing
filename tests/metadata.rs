
use json;
use junocam_processing::{path, constants, metadata, enums};
use std::fs;
use chrono::prelude::*;

const TEST_JSON_FILE_PATH : &str = "test-data/JNCE_2021052_32C00054_V01/DataSet/10124-Metadata.json";

const TEST_START_TIME_STRING : &str = "2021-02-21T18:29:46.903";

#[test]
fn test_load_json_basic() {
    assert!(path::file_exists(TEST_JSON_FILE_PATH));

    let json_test_data = fs::read_to_string(TEST_JSON_FILE_PATH).expect(constants::status::ERROR_PARSING_JSON);
    let parsed_json = json::parse(&json_test_data).unwrap();

    assert_eq!(parsed_json[constants::metadata::COMPRESSION_TYPE], "INTEGER COSINE TRANSFORM");
    assert_eq!(parsed_json[constants::metadata::FILE_RECORDS], 9984);
    assert_eq!(parsed_json[constants::metadata::SUB_SPACECRAFT_LONGITUDE], "79.5828");

    assert_eq!(parsed_json[constants::metadata::IMAGE_TIME], TEST_START_TIME_STRING);

    let start_time = Utc.datetime_from_str(parsed_json[constants::metadata::IMAGE_TIME].as_str().unwrap(), "%Y-%m-%dT%H:%M:%S%.3f").unwrap();
    let test_time_1 = Utc.datetime_from_str(TEST_START_TIME_STRING, "%Y-%m-%dT%H:%M:%S%.3f").unwrap();

    assert_eq!(start_time, test_time_1);
    
    let test_date = Utc.ymd(2021, 2, 21).and_hms_micro(18, 29, 46, 903000);
    assert_eq!(start_time, test_date);
}




#[test]
fn test_load_metadata() {
    assert!(path::file_exists(TEST_JSON_FILE_PATH));

    let md = metadata::Metadata::new_from_file(TEST_JSON_FILE_PATH).unwrap();
    
    // Tests parsing of FILTER_NAME into a valid Filters struct
    assert_eq!(md.filters.red, true);
    assert_eq!(md.filters.green, true);
    assert_eq!(md.filters.blue, true);
    assert_eq!(md.filters.methane, false);

    // Tests date parsing
    let test_date = Utc.ymd(2021, 2, 21).and_hms_micro(18, 29, 46, 903000);
    assert_eq!(md.start_time, test_date);

    // Tests u32 parsing
    assert_eq!(md.orbit_number, 32);

    // Tests f32 parsing and removal of units
    assert_eq!(md.interframe_delay, 0.370);

    // Tests f32 parsing and ':' replacement
    assert_eq!(md.spacecraft_clock_start_count, 667204540.183);

    // Tests u8 parsing
    assert_eq!(md.processing_level_id, 2);

    // Tests parsing of SampleBitMode enum
    assert_eq!(md.sample_bit_mode_id, enums::SampleBitMode::SQROOT);
}

#[test]
#[should_panic(expected = "File not found")]
fn test_load_nonexistant_metadata() {
    metadata::Metadata::new_from_file("foo").unwrap();
}