use image::{RgbImage};
mod config;
use config::*;
mod geometry;
mod raytracer;
use raytracer::pixel_color;
mod utils;
use utils::to_rgb;

fn main() {
    let mut image = RgbImage::new(WIDTH, HEIGHT);

    for x in 0..WIDTH {
        for y in 0.. HEIGHT {
            image.put_pixel(x, HEIGHT - y - 1, to_rgb(&pixel_color(x, y)));
        }
    }

    match image.save("output.png") {
        Ok(()) => println!("Image saved"),
        Err(e) => eprintln!("Couldn't save image: {}", e)
    }
}
