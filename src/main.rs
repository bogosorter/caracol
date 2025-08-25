use image::{RgbImage};
mod config;
use config::*;
mod geometry;
use geometry::bvh::build_bvh;
mod raytracer;
use raytracer::pixel_color;
mod camera;
use camera::Camera;
mod utils;

fn main() {
    let camera = Camera::new();
    let mut image = RgbImage::new(WIDTH, HEIGHT);

    let bvh = build_bvh(create_scene());

    for x in 0..WIDTH {
        utils::print_progress(x as f64 / WIDTH as f64);

        for y in 0.. HEIGHT {
            let color = pixel_color(&camera, bvh.clone(), x, y);
            image.put_pixel(x, y, utils::to_rgb(&color));
        }
    }

    utils::print_progress(1.);
    println!("");
    
    match image.save("output.png") {
        Ok(()) => println!("Image saved"),
        Err(e) => eprintln!("Couldn't save image: {}", e)
    }
}
