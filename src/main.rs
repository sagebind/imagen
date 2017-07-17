extern crate image;
extern crate rand;

use image::*;
use rand::*;
use std::env;

const MAX_WIDTH: u32 = 1024;
const MAX_HEIGHT: u32 = 1024;


fn main() {
    let mut rng = weak_rng();
    let count = env::args().nth(1).and_then(|s| s.parse().ok()).unwrap_or(1);

    println!("Generating {} random images", count);

    for i in 0..count {
        let filename = gen_filename(&mut rng, ".jpg");
        println!("  [{}/{}] {}", i + 1, count, filename);

        let buf = gen_image(&mut rng);

        if let Err(e) = buf.save(filename) {
            println!("error: {}", e);
        }
    }
}

fn gen_filename(rng: &mut XorShiftRng, extension: &str) -> String {
    let length = rng.gen_range(4, 128);
    let mut name = String::with_capacity(length + extension.len());

    for c in rng.gen_ascii_chars().take(length) {
        name.push(c);
    }

    name.push_str(extension);

    name
}

fn gen_image(rng: &mut XorShiftRng) -> ImageBuffer<Rgb<u8>, Vec<u8>>  {
    let width = rng.gen_range(32, MAX_WIDTH);
    let height = rng.gen_range(32, MAX_HEIGHT);

    let mut buf = image::ImageBuffer::new(width, height);

    for (_, _, pixel) in buf.enumerate_pixels_mut() {
        *pixel = image::Rgb {
            data: [rng.gen_range(0, 255), rng.gen_range(0, 255), rng.gen_range(0, 255)],
        };
    }

    buf
}
