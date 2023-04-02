use crate::{constants, enums};
use json;

use sciimg::error;
use sciimg::path;

use chrono::prelude::*;
use std::fs;

pub struct Filters {
    pub red: bool,
    pub green: bool,
    pub blue: bool,
    pub methane: bool, // Gonna ignore methane for now.
}

impl Filters {
    pub fn new(r: bool, g: bool, b: bool) -> Self {
        Filters {
            red: r,
            green: g,
            blue: b,
            methane: false,
        }
    }
}

pub struct Metadata {
    pub compression_type: String,
    pub data_set_id: String,
    pub description: String,
    pub exposure_duration: f32, // Seconds
    pub file_name: String,
    pub file_records: u32,
    pub filters: Filters,             // Derived from FILTER_NAME
    pub focal_plane_temperature: f32, // Kelvin
    pub image_time: DateTime<chrono::Utc>,
    pub instrument_host_name: String,
    pub instrument_id: String,
    pub instrument_name: String,
    pub interframe_delay: f32, // Seconds
    pub jno_tdi_stages_count: u32,
    pub lines: u32,
    pub line_prefix_bytes: u32,
    pub line_samples: u32,
    pub line_suffix_bytes: u32,
    pub mission_phase_name: String,
    pub orbit_number: u32,
    pub pj: String, // String version of orbit_number for some reason
    pub processing_level_id: u8,
    pub producer_id: String,
    pub product_creation_time: DateTime<chrono::Utc>,
    pub product_version_id: u8,
    pub rationale_desc: String,
    pub record_bytes: u32,
    pub sample_bits: u8,
    pub sample_bit_mask: String,
    pub sample_bit_mode_id: enums::SampleBitMode,
    pub sample_type: String,
    pub sampling_factor: u8,
    pub sequence_id: String,
    pub software_name: String,
    pub solar_distance: f32, // kilometers
    pub source_product_id: String,
    pub spacecraft_altitude: f32,            // kilometers
    pub spacecraft_clock_start_count: f32,   // seconds
    pub spacecraft_clock_stop_count: String, // Probably not a string when non-zero (N/A)
    pub spacecaft_name: String,
    pub standard_data_product_id: String,
    pub start_time: DateTime<chrono::Utc>,
    pub stop_time: DateTime<chrono::Utc>,
    pub sub_spacecraft_latitude: f32,
    pub sub_spacecraft_longitude: f32,
    pub title: String,
    pub token_id: u8, // What even is this?
}

const DATE_FORMAT_STRING: &str = "%Y-%m-%dT%H:%M:%S%.3f";

fn parse_date(date_str: &str) -> DateTime<chrono::Utc> {
    Utc.datetime_from_str(date_str, DATE_FORMAT_STRING).unwrap()
}

fn strip_units(s: &str) -> error::Result<String> {
    let idx = s.find('<');
    let r = s.replace(':', ".");

    if let Some(i) = idx {
        let t = &r[..(i - 1)];
        Ok(t.to_string())
    } else {
        Ok(r)
    }
}

macro_rules! _S {
    ($a:expr) => {
        String::from($a.as_str().unwrap())
    };
}

macro_rules! _D {
    ($a:expr) => {
        parse_date($a.as_str().unwrap())
    };
}

macro_rules! _F32 {
    ($a:expr) => {
        strip_units($a.as_str().unwrap())
            .unwrap()
            .as_str()
            .parse::<f32>()
            .unwrap()
    };
}

macro_rules! _U32 {
    ($a:expr) => {
        $a.as_u32().unwrap()
    };
}

macro_rules! _U8 {
    ($a:expr) => {
        $a.as_u8().unwrap()
    };
}

impl Metadata {
    pub fn new_from_file(file_path: &str) -> error::Result<Metadata> {
        if !path::file_exists(file_path) {
            return Err(constants::status::FILE_NOT_FOUND);
        }

        let json_string_data =
            fs::read_to_string(file_path).expect(constants::status::ERROR_PARSING_JSON);
        let parsed_json = json::parse(&json_string_data).unwrap();

        Ok(Metadata {
            image_time: _D!(parsed_json[constants::metadata::IMAGE_TIME]),
            start_time: _D!(parsed_json[constants::metadata::START_TIME]),
            stop_time: _D!(parsed_json[constants::metadata::STOP_TIME]),
            compression_type: _S!(parsed_json[constants::metadata::COMPRESSION_TYPE]),
            data_set_id: _S!(parsed_json[constants::metadata::DATA_SET_ID]),
            description: _S!(parsed_json[constants::metadata::DESCRIPTION]),
            exposure_duration: _F32!(parsed_json[constants::metadata::EXPOSURE_DURATION]), // Seconds
            file_name: _S!(parsed_json[constants::metadata::FILE_NAME]),
            file_records: _U32!(parsed_json[constants::metadata::FILE_RECORDS]),
            filters: Filters::new(
                parsed_json[constants::metadata::FILTER_NAME].contains(constants::filters::RED),
                parsed_json[constants::metadata::FILTER_NAME].contains(constants::filters::GREEN),
                parsed_json[constants::metadata::FILTER_NAME].contains(constants::filters::BLUE),
            ), // Derived from FILTER_NAME
            focal_plane_temperature: _F32!(
                parsed_json[constants::metadata::FOCAL_PLANE_TEMPERATURE]
            ), // Kelvin
            instrument_host_name: _S!(parsed_json[constants::metadata::INSTRUMENT_HOST_NAME]),
            instrument_id: _S!(parsed_json[constants::metadata::INSTRUMENT_ID]),
            instrument_name: _S!(parsed_json[constants::metadata::INSTRUMENT_NAME]),
            interframe_delay: _F32!(parsed_json[constants::metadata::INTERFRAME_DELAY]), // Seconds
            jno_tdi_stages_count: _U32!(parsed_json[constants::metadata::JNO_TDI_STAGES_COUNT]),
            lines: _U32!(parsed_json[constants::metadata::LINES]),
            line_prefix_bytes: _U32!(parsed_json[constants::metadata::LINE_PREFIX_BYTES]),
            line_samples: _U32!(parsed_json[constants::metadata::LINE_SAMPLES]),
            line_suffix_bytes: _U32!(parsed_json[constants::metadata::LINE_SUFFIX_BYTES]),
            mission_phase_name: _S!(parsed_json[constants::metadata::MISSION_PHASE_NAME]),
            orbit_number: _U32!(parsed_json[constants::metadata::ORBIT_NUMBER]),
            pj: _S!(parsed_json[constants::metadata::PJ]), // String version of orbit_number for some reason
            processing_level_id: _U8!(parsed_json[constants::metadata::PROCESSING_LEVEL_ID]),
            producer_id: _S!(parsed_json[constants::metadata::PRODUCER_ID]),
            product_creation_time: _D!(parsed_json[constants::metadata::PRODUCT_CREATION_TIME]),
            product_version_id: _U8!(parsed_json[constants::metadata::PRODUCT_VERSION_ID]),
            rationale_desc: _S!(parsed_json[constants::metadata::RATIONALE_DESC]),
            record_bytes: _U32!(parsed_json[constants::metadata::RECORD_BYTES]),
            sample_bits: _U8!(parsed_json[constants::metadata::SAMPLE_BITS]),
            sample_bit_mask: _S!(parsed_json[constants::metadata::SAMPLE_BIT_MASK]),
            sample_bit_mode_id: enums::SampleBitMode::from(
                parsed_json[constants::metadata::SAMPLE_BIT_MODE_ID]
                    .as_str()
                    .unwrap(),
            ),
            sample_type: _S!(parsed_json[constants::metadata::SAMPLE_TYPE]),
            sampling_factor: _U8!(parsed_json[constants::metadata::SAMPLING_FACTOR]),
            sequence_id: _S!(parsed_json[constants::metadata::SEQUENCE_ID]),
            software_name: _S!(parsed_json[constants::metadata::SOFTWARE_NAME]),
            solar_distance: _F32!(parsed_json[constants::metadata::SOLAR_DISTANCE]), // kilometers
            source_product_id: _S!(parsed_json[constants::metadata::SOURCE_PRODUCT_ID]),
            spacecraft_altitude: _F32!(parsed_json[constants::metadata::SPACECRAFT_ALTITUDE]), // kilometers
            spacecraft_clock_start_count: _F32!(
                parsed_json[constants::metadata::SPACECRAFT_CLOCK_START_COUNT]
            ), // seconds
            spacecraft_clock_stop_count: _S!(
                parsed_json[constants::metadata::SPACECRAFT_CLOCK_STOP_COUNT]
            ), // Probably not a string when non-zero (N/A)
            spacecaft_name: _S!(parsed_json[constants::metadata::SPACECRAFT_NAME]),
            standard_data_product_id: _S!(
                parsed_json[constants::metadata::STANDARD_DATA_PRODUCT_ID]
            ),
            sub_spacecraft_latitude: _F32!(
                parsed_json[constants::metadata::SUB_SPACECRAFT_LATITUDE]
            ),
            sub_spacecraft_longitude: _F32!(
                parsed_json[constants::metadata::SUB_SPACECRAFT_LONGITUDE]
            ),
            title: _S!(parsed_json[constants::metadata::TITLE]),
            token_id: 0, // What even is this?
        })
    }
}
