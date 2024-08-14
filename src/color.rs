use crate::{percent::Percent};

mod inter_color_conversion;

/// Represents a ratio of red, green, and blue color channels. The fields are brightness percentages (0.0 black, 1.0 full color).
/// ## Invariant
/// Each color channel will be in the interval `[0.0, 1.0]`
#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct RgbPercent {
    red: Percent,
    green: Percent,
    blue: Percent,
}
impl RgbPercent {
    pub fn red(&self) -> f64 {
        self.red.get()
    }
    pub fn green(&self) -> f64 {
        self.green.get()
    }
    pub fn blue(&self) -> f64 {
        self.blue.get()
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RgbU8 {
    red: u8,
    green: u8,
    blue: u8,
}
impl RgbU8 {
    pub fn red(&self) -> u8 {
        self.red
    }
    pub fn green(&self) -> u8 {
        self.green
    }
    pub fn blue(&self) -> u8 {
        self.blue
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RgbaU8 {
    red: u8,
    green: u8,
    blue: u8,
    alpha: u8,
}
impl RgbaU8 {
    pub fn red(&self) -> u8 {
        self.red
    }
    pub fn green(&self) -> u8 {
        self.green
    }
    pub fn blue(&self) -> u8 {
        self.blue
    }
    pub fn alpha(&self) -> u8 {
        self.alpha
    }
}
