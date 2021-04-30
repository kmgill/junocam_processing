
pub const _16_BIT_MAX : f32 = std::u16::MAX as f32;

pub const STRIP_HEIGHT : usize = 128;
pub const STRIP_WIDTH : usize = 1648;

pub const DEFAULT_RED_WEIGHT : f32 = 0.902;
pub const DEFAULT_GREEN_WEIGHT : f32 = 1.0;
pub const DEFAULT_BLUE_WEIGHT : f32 = 1.8889;

// Strings
pub mod status {
    pub const EMPTY : &str = "";
    pub const OK : &str = "ok";
    pub const STRUCT_IS_EMPTY : &str = "Structure is empty";
    pub const INVALID_PIXEL_COORDINATES : &str = "Invalid pixel coordinates";
    pub const PARENT_NOT_EXISTS_OR_UNWRITABLE : &str = "Parent does not exist or cannot be written";
    pub const FILE_NOT_FOUND: &str = "File not found";
    pub const ARRAY_SIZE_MISMATCH : &str = "Array size mismatch";
    pub const NOT_IMPLEMENTED : &str = "Not yet implemented";
    pub const DIMENSIONS_DO_NOT_MATCH_VECTOR_LENGTH : &str = "Image dimensions do not match supplied vector length";
    pub const ERROR_PARSING_JSON: &str = "Error parsing JSON";
    pub const INVALID_ENUM_VALUE: &str = "Invalid enum value";
    pub const INVALID_RAW_VALUE: &str = "Invalid raw image value";
    pub const UNSUPPORTED_INSTRUMENT: &str = "Unsupported instrument";
    pub const UNSUPPORTED_COLOR_CHANNEL: &str = "Unsupported color channel";
}


// Parameters
pub mod param {
    pub const PARAM_VERBOSE : &str = "v";
    pub const PARAM_OUTPUT : &str = "output";
    pub const PARAM_OUTPUT_SHORT : &str = "o";
    pub const PARAM_DARK : &str = "dark";
    pub const PARAM_DARK_SHORT : &str = "d";
    pub const PARAM_FLAT : &str = "flat";
    pub const PARAM_FLAT_SHORT : &str = "f";
    pub const PARAM_INPUTS : &str = "inputs";
    pub const PARAM_INPUTS_SHORT : &str = "i";
}



pub mod metadata {
    pub const COMPRESSION_TYPE : &str = "COMPRESSION_TYPE";
    pub const DATA_SET_ID : &str = "DATA_SET_ID";
    pub const DESCRIPTION : &str = "DESCRIPTION";
    pub const EXPOSURE_DURATION : &str = "EXPOSURE_DURATION";
    pub const FILE_NAME : &str = "FILE_NAME";
    pub const FILE_RECORDS : &str = "FILE_RECORDS";
    pub const FILTER_NAME : &str = "FILTER_NAME";
    pub const FOCAL_PLANE_TEMPERATURE : &str = "FOCAL_PLANE_TEMPERATURE";
    pub const IMAGE_TIME : &str = "IMAGE_TIME";
    pub const INSTRUMENT_HOST_NAME : &str = "INSTRUMENT_HOST_NAME";
    pub const INSTRUMENT_ID : &str = "INSTRUMENT_ID";
    pub const INSTRUMENT_NAME : &str = "INSTRUMENT_NAME";
    pub const INTERFRAME_DELAY : &str = "INTERFRAME_DELAY";
    pub const JNO_TDI_STAGES_COUNT : &str = "JNO:TDI_STAGES_COUNT";
    pub const LINES : &str = "LINES";
    pub const LINE_PREFIX_BYTES : &str = "LINE_PREFIX_BYTES";
    pub const LINE_SAMPLES : &str = "LINE_SAMPLES";
    pub const LINE_SUFFIX_BYTES : &str = "LINE_SUFFIX_BYTES";
    pub const MISSION_PHASE_NAME : &str = "MISSION_PHASE_NAME";
    pub const ORBIT_NUMBER : &str = "ORBIT_NUMBER";
    pub const PJ : &str = "PJ";
    pub const PROCESSING_LEVEL_ID : &str = "PROCESSING_LEVEL_ID";
    pub const PRODUCER_ID : &str = "PRODUCER_ID";
    pub const PRODUCT_CREATION_TIME : &str = "PRODUCT_CREATION_TIME";
    pub const PRODUCT_ID : &str = "PRODUCT_ID";
    pub const PRODUCT_VERSION_ID : &str = "PRODUCT_VERSION_ID";
    pub const RATIONALE_DESC : &str = "RATIONALE_DESC";
    pub const RECORD_BYTES : &str = "RECORD_BYTES";
    pub const SAMPLE_BITS : &str = "SAMPLE_BITS";
    pub const SAMPLE_BIT_MASK : &str = "SAMPLE_BIT_MASK";
    pub const SAMPLE_BIT_MODE_ID : &str = "SAMPLE_BIT_MODE_ID";
    pub const SAMPLE_TYPE : &str = "SAMPLE_TYPE";
    pub const SAMPLING_FACTOR : &str = "SAMPLING_FACTOR";
    pub const SEQUENCE_ID : &str = "SEQUENCE_ID";
    pub const SOFTWARE_NAME : &str = "SOFTWARE_NAME";
    pub const SOLAR_DISTANCE : &str = "SOLAR_DISTANCE";
    pub const SOURCE_PRODUCT_ID : &str = "SOURCE_PRODUCT_ID";
    pub const SPACECRAFT_ALTITUDE : &str = "SPACECRAFT_ALTITUDE";
    pub const SPACECRAFT_CLOCK_START_COUNT : &str = "SPACECRAFT_CLOCK_START_COUNT";
    pub const SPACECRAFT_CLOCK_STOP_COUNT : &str = "SPACECRAFT_CLOCK_STOP_COUNT";
    pub const SPACECRAFT_NAME : &str = "SPACECRAFT_NAME";
    pub const STANDARD_DATA_PRODUCT_ID : &str = "STANDARD_DATA_PRODUCT_ID";
    pub const START_TIME : &str = "START_TIME";
    pub const STOP_TIME : &str = "STOP_TIME";
    pub const SUB_SPACECRAFT_LATITUDE : &str = "SUB_SPACECRAFT_LATITUDE";
    pub const SUB_SPACECRAFT_LONGITUDE : &str = "SUB_SPACECRAFT_LONGITUDE";
    pub const TARGET_NAME : &str = "TARGET_NAME";
    pub const TITLE : &str = "TITLE";
    pub const TOKEN_ID : &str = "TOKEN_ID";
}

pub mod filters {
    pub const RED: &str = "RED";
    pub const GREEN: &str = "GREEN";
    pub const BLUE: &str = "BLUE";
    pub const METHANE: &str = "METHANE";
}

pub mod cal {
    const fn data_dir() -> &'static str {
        if cfg!(debug_assertions) {
            return "src/cal";
        } else {
            return "/usr/share/junocam_processing/data/";
        }
    }

    //pub const M20_INPAINT_MASK_RIGHT_PATH : &str = const_format::formatcp!("{}/{}", data_dir(), "M20_MCZ_RIGHT_INPAINT_MASK_V1.png");
    pub const JNO_INPAINT_MASK_RED : &str = const_format::formatcp!("{}/{}", data_dir(), "junocam_inpaint_mask_pj32_v1_red.png");
    pub const JNO_INPAINT_MASK_GREEN : &str = const_format::formatcp!("{}/{}", data_dir(), "junocam_inpaint_mask_pj32_v1_green.png");
    pub const JNO_INPAINT_MASK_BLUE : &str = const_format::formatcp!("{}/{}", data_dir(), "junocam_inpaint_mask_pj32_v1_blue.png");

    pub const JNO_FLATFIELD_RED : &str = const_format::formatcp!("{}/{}", data_dir(), "junocam_rgb_flatfield_v3_2.png");
    pub const JNO_FLATFIELD_GREEN : &str = const_format::formatcp!("{}/{}", data_dir(), "junocam_rgb_flatfield_v3_1.png");
    pub const JNO_FLATFIELD_BLUE : &str = const_format::formatcp!("{}/{}", data_dir(), "junocam_rgb_flatfield_v3_0.png");

    pub const JNO_DARKFIELD_RED : &str = const_format::formatcp!("{}/{}", data_dir(), "junocam_dark_pj28_v1_red.png");
    pub const JNO_DARKFIELD_GREEN : &str = const_format::formatcp!("{}/{}", data_dir(), "junocam_dark_pj28_v1_green.png");
    pub const JNO_DARKFIELD_BLUE : &str = const_format::formatcp!("{}/{}", data_dir(), "junocam_dark_pj28_v1_blue.png");


}