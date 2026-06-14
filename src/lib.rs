use crate::color::RgbPercent;

pub mod color;
pub mod error;
pub mod netpbm;
pub mod r3_vector;
pub mod ray;

pub const CLEAR_LINE: &str = "\r\x1B[K";

pub fn generate_red_green_gradient_ppm(width: u32, height: u32) -> Vec<u8> {
    netpbm::Header {
        format: netpbm::Format::P6,
        height,
        width,
        maximum: 255,
    }
    .to_bytes()
    .into_iter()
    .chain(
        (0..height)
            .inspect(|&row| {
                eprint!(
                    "{CLEAR_LINE}Scanlines remaining: {}{}",
                    height - row,
                    if row == height - 1 {
                        format!("{CLEAR_LINE}Scanlines remaining: 0\nDone\n")
                    } else {
                        String::new()
                    }
                )
            })
            .flat_map(|row| {
                (0..width).flat_map(move |column| {
                    let horizontal_ratio = column as f64 / width as f64;
                    let vertical_ratio = row as f64 / height as f64;
                    RgbPercent {
                        red: horizontal_ratio,
                        green: vertical_ratio,
                        blue: 0.0,
                    }
                    .to_rgb8()
                    .to_ppm_binary()
                })
            }),
    )
    .collect()
}
