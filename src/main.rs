extern crate image;
extern crate rand;
extern crate rayon;
#[macro_use]
extern crate structopt;

use image::*;
use rand::*;
use rayon::prelude::*;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Options {
    #[structopt(short = "o", long = "output-dir", default_value = ".", parse(from_os_str))]
    output_directory: PathBuf,

    #[structopt(long = "max-width", default_value = "1024")]
    max_width: u32,

    #[structopt(long = "max-height", default_value = "1024")]
    max_height: u32,

    #[structopt(name = "COUNT", default_value = "1")]
    count: u32,
}

fn main() {
    let options = Options::from_args();
    let count = options.count;

    println!("Generating {} random images", count);

    (0..count).into_par_iter().for_each(|i| {
        let mut rng = rand::thread_rng();
        let mut path = options.output_directory.clone();

        let filename = gen_filename(&mut rng, ".jpg");
        println!("  [{}/{}] {}", i + 1, count, filename);
        path.push(filename);

        let buf = gen_image(&mut rng, &options);

        if let Err(e) = buf.save(path) {
            println!("error: {}", e);
        }
    });
}

fn gen_filename<R: Rng>(rng: &mut R, extension: &str) -> String {
    let length = rng.gen_range(4, 128);
    let mut name = String::with_capacity(length + extension.len());

    for c in rng.gen_ascii_chars().take(length) {
        name.push(c);
    }

    name.push_str(extension);

    name
}

fn gen_image<R: Rng>(rng: &mut R, options: &Options) -> ImageBuffer<Rgb<u8>, Vec<u8>>  {
    let width = rng.gen_range(32, options.max_width);
    let height = rng.gen_range(32, options.max_height);

    let mut buf = image::ImageBuffer::new(width, height);

    for (_, _, pixel) in buf.enumerate_pixels_mut() {
        *pixel = image::Rgb {
            data: [rng.gen_range(0, 255), rng.gen_range(0, 255), rng.gen_range(0, 255)],
        };
    }

    buf
}
