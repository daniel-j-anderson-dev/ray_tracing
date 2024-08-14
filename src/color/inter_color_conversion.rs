use crate::{
    color::{RgbPercent, RgbU8, RgbaU8},
    error::Error,
};

impl TryFrom<(f64, f64, f64)> for RgbPercent {
    type Error = Error;
    fn try_from(value: (f64, f64, f64)) -> Result<Self, Self::Error> {
        Ok(RgbPercent {
            red: value.0.try_into()?,
            green: value.1.try_into()?,
            blue: value.2.try_into()?,
        })
    }
}
impl TryFrom<[f64; 3]> for RgbPercent {
    type Error = Error;
    fn try_from(value: [f64; 3]) -> Result<Self, Self::Error> {
        Ok(RgbPercent {
            red: value[0].try_into()?,
            green: value[1].try_into()?,
            blue: value[2].try_into()?,
        })
    }
}
impl From<RgbPercent> for (f64, f64, f64) {
    fn from(value: RgbPercent) -> Self {
        (value.red(), value.green(), value.blue())
    }
}
impl From<RgbPercent> for [f64; 3] {
    fn from(value: RgbPercent) -> Self {
        [value.red(), value.green(), value.blue()]
    }
}

impl From<RgbPercent> for RgbU8 {
    fn from(value: RgbPercent) -> Self {
        const SCALE: f64 = 255.999;
        Self {
            red: (value.red() * SCALE) as u8,
            green: (value.green() * SCALE) as u8,
            blue: (value.blue() * SCALE) as u8,
        }
    }
}
impl From<(u8, u8, u8)> for RgbU8 {
    fn from(value: (u8, u8, u8)) -> Self {
        RgbU8 {
            red: value.0,
            green: value.1,
            blue: value.2,
        }
    }
}
impl From<RgbU8> for (u8, u8, u8) {
    fn from(value: RgbU8) -> Self {
        (value.red, value.green, value.blue)
    }
}
impl From<[u8; 3]> for RgbU8 {
    fn from(value: [u8; 3]) -> Self {
        RgbU8 {
            red: value[0],
            green: value[1],
            blue: value[2],
        }
    }
}
impl From<RgbU8> for [u8; 3] {
    fn from(value: RgbU8) -> Self {
        [value.red, value.green, value.blue]
    }
}

impl From<(u8, u8, u8, u8)> for RgbaU8 {
    fn from(value: (u8, u8, u8, u8)) -> Self {
        RgbaU8 {
            red: value.0,
            green: value.1,
            blue: value.2,
            alpha: value.3,
        }
    }
}
impl From<RgbaU8> for (u8, u8, u8, u8) {
    fn from(value: RgbaU8) -> Self {
        (value.red, value.green, value.blue, value.alpha)
    }
}
impl From<[u8; 4]> for RgbaU8 {
    fn from(value: [u8; 4]) -> Self {
        RgbaU8 {
            red: value[0],
            green: value[0],
            blue: value[0],
            alpha: value[0],
        }
    }
}
impl From<RgbaU8> for [u8; 4] {
    fn from(value: RgbaU8) -> Self {
        [value.red, value.green, value.blue, value.alpha]
    }
}
