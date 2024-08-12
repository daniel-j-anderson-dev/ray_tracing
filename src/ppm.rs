use crate::CLEAR_LINE;

use std::num::NonZeroUsize;

fn generate_red_green_gradient_ppm(image_width: NonZeroUsize, image_height: NonZeroUsize) -> String {
    // header
    let mut ppm = format!("P3\n{} {}\n255\n", image_width, image_height);

    eprintln!("Lines remaining: ");

    // pixel data
    for row_index in 0..image_height.get() {
        eprint!("{CLEAR_LINE}{}", image_height.get() - row_index);

        for column_index in 0..image_width.get() {
            
            let red = column_index % 256;
            let green = row_index % 256;

            ppm.push_str(&format!("{} {} 0\n", red, green));
        }
    }

    eprintln!("{CLEAR_LINE}0");

    return ppm;
}

#[test]
fn save_red_green_gradient_ppm() {
    use std::{fs::OpenOptions, io::Write};

    let size = NonZeroUsize::new(256).unwrap();

    eprintln!("generating {}x{} red green gradient", size.get(), size.get());

    let gradient_image = generate_red_green_gradient_ppm(size, size);
    let gradient_path = format!("{}x{}_red_green_gradient.ppm", size.get(), size.get());

    eprintln!("saving gradient");

    OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(&gradient_path)
        .unwrap()
        .write_all(gradient_image.as_bytes())
        .unwrap();

    eprintln!("gradient saved to ./{}", gradient_path);
}