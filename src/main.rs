use std::{fs::File, io::Write};

use ray_tracing::{Resolution, color::Rgb8, netpbm, red_green_gradient};

fn main() -> Result<(), Box<dyn core::error::Error>> {
    let resolution = Resolution {
        height: 256,
        width: 256,
    };
    let output_path = format!("output/{}_red_green_gradient.ppm", resolution);

    eprintln!("generating {} red green gradient", resolution);

    let header = netpbm::Header {
        format: netpbm::Format::PIXEL_MAP_BINARY,
        resolution,
        maximum: 255,
    };
    let pixels = red_green_gradient(resolution).flat_map(Rgb8::to_ppm_binary);
    let output = header
        .to_bytes()
        .into_iter()
        .chain(pixels)
        .collect::<Box<_>>();

    eprintln!("saving gradient");
    File::options()
        .create(true)
        .write(true)
        .truncate(true)
        .open(&output_path)?
        .write_all(&output)?;
    eprintln!("gradient saved to ./{}", output_path);

    Ok(())
}
