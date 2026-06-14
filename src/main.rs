use std::{fs::OpenOptions, io::Write};

use ray_tracing::generate_red_green_gradient_ppm;

fn main() -> Result<(), Box<dyn core::error::Error>> {
    let width = 256;
    let height = 256;
    let gradient_path = format!("{}x{}_red_green_gradient.ppm", width, height);

    eprintln!("generating {}x{} red green gradient", width, height,);

    let gradient_image = generate_red_green_gradient_ppm(width, height);

    eprintln!("saving gradient");
    OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(&gradient_path)?
        .write_all(&gradient_image)?;
    eprintln!("gradient saved to ./{}", gradient_path);

    Ok(())
}
