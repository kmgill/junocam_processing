#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Camera {
    RED,
    GREEN,
    BLUE,
    METHANE,
    NONE,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum SampleBitMode {
    SQROOT,
    LIN1,
    LIN8,
    LIN16,
    UNKNOWN,
}

impl SampleBitMode {
    pub fn from(s: &str) -> SampleBitMode {
        match s {
            "SQROOT" => SampleBitMode::SQROOT,
            "LIN1" => SampleBitMode::LIN1,
            "LIN8" => SampleBitMode::LIN8,
            "LIN16" => SampleBitMode::LIN16,
            _ => SampleBitMode::UNKNOWN,
        }
    }
}
