
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Camera {
    RED,
    GREEN,
    BLUE,
    METHANE,
    NONE
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum SampleBitMode {
    SQROOT,
    LIN1,
    LIN8,
    LIN16,
    UNKNOWN
}

impl SampleBitMode {

    pub fn from(s:&str) -> SampleBitMode {
        match s {
            "SQROOT" => SampleBitMode::SQROOT,
            "LIN1" => SampleBitMode::LIN1,
            "LIN8" => SampleBitMode::LIN8,
            "LIN16" => SampleBitMode::LIN16,
            _ => SampleBitMode::UNKNOWN
        }
    }
}

// Image data value range. Doesn't enforce actual
// value data types in the structs
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ImageMode {
    U8BIT,
    U12BIT,
    U16BIT
}