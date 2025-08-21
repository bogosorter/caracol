use image::{RgbImage};
mod config;
use config::*;
mod geometry;
mod raytracer;
use raytracer::pixel_color;
mod utils;

fn main() {
    let mut image = RgbImage::new(WIDTH, HEIGHT);

    for x in 0..WIDTH {
        utils::print_progress(x as f32 / WIDTH as f32);

        for y in 0.. HEIGHT {
            image.put_pixel(x, HEIGHT - y - 1, utils::to_rgb(&pixel_color(x, y)));
        }
    }

    utils::print_progress(1.);
    println!("");
    
    match image.save("output.png") {
        Ok(()) => println!("Image saved"),
        Err(e) => eprintln!("Couldn't save image: {}", e)
    }
}
