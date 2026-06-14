use crate::color::{Rgb8, RgbPercent};

pub mod color;
pub mod error;
pub mod netpbm;
pub mod r3_vector;
pub mod ray;

#[derive(Debug, Clone, Copy)]
pub struct Resolution {
    pub height: u32,
    pub width: u32,
}
impl Resolution {
    pub fn aspect_ratio(self) -> f64 {
        self.width as f64 / self.height as f64
    }
}
impl core::fmt::Display for Resolution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}x{}", self.width, self.height)
    }
}

pub const CLEAR_LINE: &str = "\r\x1B[K";

pub fn red_green_gradient(Resolution { height, width }: Resolution) -> impl Iterator<Item = Rgb8> {
    (0..height).flat_map(move |row| {
        (0..width).map(move |column| {
            let horizontal_ratio = column as f64 / width as f64;
            let vertical_ratio = row as f64 / height as f64;
            RgbPercent {
                red: horizontal_ratio,
                green: vertical_ratio,
                blue: 0.0,
            }
            .to_rgb8()
        })
    })
}
