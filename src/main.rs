use image::{RgbImage};
mod config;
use config::*;
mod geometry;
use geometry::ray::Ray;
mod raytracer;
use raytracer::raytrace;
mod utils;
use utils::to_rgb;

fn main() {
    let mut image = RgbImage::new(WIDTH, HEIGHT);

    for x in 0..WIDTH {
        for y in 0.. HEIGHT {
            // Calculate ray direction relative to camera. Top left corner is
            // given by (-tan(FOV_ANGLE / 2), tan(FOV_ANGLE / 2) * HEIGHT / WIDTH)
            let ray_x = (FOV_ANGLE / 2.).tan() * (-1. + (x as f32 + 0.5) * 2. / WIDTH as f32);
            let ray_y = (FOV_ANGLE / 2.).tan() * HEIGHT as f32 / WIDTH as f32 * (1. - (y as f32 + 0.5) * 2. / HEIGHT as f32);

            // Transform the ray's direction coordinates from camera space to
            // world space
            let world_direction = (ray_x * RIGHT + ray_y * UP + FORWARD).normalize();
            let ray = Ray::new(CAMERA_POSITION, world_direction);

            let pixel_color = raytrace(&ray, BOUNCES);
            image.put_pixel(x, HEIGHT - y - 1, to_rgb(&pixel_color));
        }
    }

    match image.save("output.png") {
        Ok(()) => println!("Image saved"),
        Err(e) => eprintln!("Couldn't save image: {}", e)
    }
}
