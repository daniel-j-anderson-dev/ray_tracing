use crate::color::{Rgb8, RgbPercent, Rgba8};

impl RgbPercent {
    pub const fn from_tuple((red, green, blue): (f64, f64, f64)) -> Self {
        Self { red, green, blue }
    }
    pub const fn from_array([red, green, blue]: [f64; 3]) -> Self {
        Self { red, green, blue }
    }
    pub const fn to_tuple(self) -> (f64, f64, f64) {
        let Self { red, green, blue } = self;
        (red, green, blue)
    }
    pub const fn to_array(self) -> [f64; 3] {
        let Self { red, green, blue } = self;
        [red, green, blue]
    }
    pub const fn to_rgb8(self) -> Rgb8 {
        const SCALE: f64 = 255.999;
        let Self { red, green, blue } = self;
        Rgb8 {
            red: (red * SCALE) as u8,
            green: (green * SCALE) as u8,
            blue: (blue * SCALE) as u8,
        }
    }
}

impl Rgb8 {
    pub const fn from_tuple((red, green, blue): (u8, u8, u8)) -> Self {
        Self { red, green, blue }
    }
    pub const fn from_array([red, green, blue]: [u8; 3]) -> Self {
        Self { red, green, blue }
    }
    pub const fn to_tuple(self) -> (u8, u8, u8) {
        let Self { red, green, blue } = self;
        (red, green, blue)
    }
    pub const fn to_array(self) -> [u8; 3] {
        let Self { red, green, blue } = self;
        [red, green, blue]
    }
    pub const fn to_rgb_percent(self) -> RgbPercent {
        const SCALE: f64 = 255.0;
        let Self { red, green, blue } = self;
        RgbPercent {
            red: red as f64 / SCALE,
            green: green as f64 / SCALE,
            blue: blue as f64 / SCALE,
        }
    }
    pub const fn with_alpha(&self, alpha: u8) -> Rgba8 {
        let &Self { red, green, blue } = self;
        Rgba8 {
            red,
            green,
            blue,
            alpha,
        }
    }
}

impl Rgba8 {
    pub const fn from_tuple((red, green, blue, alpha): (u8, u8, u8, u8)) -> Self {
        Rgba8 {
            red,
            green,
            blue,
            alpha,
        }
    }
    pub const fn from_array([red, green, blue, alpha]: [u8; 4]) -> Self {
        Rgba8 {
            red,
            green,
            blue,
            alpha,
        }
    }
    pub const fn to_tuple(self) -> (u8, u8, u8, u8) {
        let Rgba8 {
            red,
            green,
            blue,
            alpha,
        } = self;
        (red, green, blue, alpha)
    }
    pub const fn to_array(self) -> [u8; 4] {
        let Rgba8 {
            red,
            green,
            blue,
            alpha,
        } = self;
        [red, green, blue, alpha]
    }
}

impl From<(f64, f64, f64)> for RgbPercent {
    fn from(tuple: (f64, f64, f64)) -> Self {
        Self::from_tuple(tuple)
    }
}
impl From<[f64; 3]> for RgbPercent {
    fn from(array: [f64; 3]) -> Self {
        Self::from_array(array)
    }
}
impl From<RgbPercent> for (f64, f64, f64) {
    fn from(value: RgbPercent) -> Self {
        value.to_tuple()
    }
}
impl From<RgbPercent> for [f64; 3] {
    fn from(value: RgbPercent) -> Self {
        value.to_array()
    }
}
impl From<RgbPercent> for Rgb8 {
    fn from(value: RgbPercent) -> Self {
        value.to_rgb8()
    }
}
impl From<Rgb8> for RgbPercent {
    fn from(value: Rgb8) -> Self {
        value.to_rgb_percent()
    }
}
impl From<(u8, u8, u8)> for Rgb8 {
    fn from(tuple: (u8, u8, u8)) -> Self {
        Self::from_tuple(tuple)
    }
}
impl From<Rgb8> for (u8, u8, u8) {
    fn from(value: Rgb8) -> Self {
        value.to_tuple()
    }
}
impl From<[u8; 3]> for Rgb8 {
    fn from(array: [u8; 3]) -> Self {
        Self::from_array(array)
    }
}
impl From<Rgb8> for [u8; 3] {
    fn from(value: Rgb8) -> Self {
        value.to_array()
    }
}
impl From<(u8, u8, u8, u8)> for Rgba8 {
    fn from(tuple: (u8, u8, u8, u8)) -> Self {
        Self::from_tuple(tuple)
    }
}
impl From<Rgba8> for (u8, u8, u8, u8) {
    fn from(value: Rgba8) -> Self {
        value.to_tuple()
    }
}
impl From<[u8; 4]> for Rgba8 {
    fn from(array: [u8; 4]) -> Self {
        Self::from_array(array)
    }
}
impl From<Rgba8> for [u8; 4] {
    fn from(value: Rgba8) -> Self {
        value.to_array()
    }
}
