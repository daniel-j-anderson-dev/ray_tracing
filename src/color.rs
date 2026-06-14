pub mod conversion;

/// Represents a ratio of red, green, and blue color channels.
/// The fields are brightness percentages (0.0 no color, 1.0 full color).
#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct RgbPercent {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Rgb8 {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Rgba8 {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}
