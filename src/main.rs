use image::{RgbImage, Rgb};
mod config;
use config::*;
mod geometry;
use geometry::ray::Ray;

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

            let mut distance: Option<f32> = None;
            let mut color: Option<Rgb<u8>> = None;
            for object in OBJECTS.iter() {
                if let Some(d) = ray.shoot(object) {
                    if distance.is_none() || d < distance.unwrap() {
                        distance = Some(d);
                        color = Some(object.color);
                    }
                }
            }

            if distance.is_none() {
                image.put_pixel(x, y, Rgb([0, 0, 0]));
            } else {
                image.put_pixel(x, y, color.unwrap());
            }

        }
    }

    match image.save("output.png") {
        Ok(()) => println!("Image saved"),
        Err(e) => eprintln!("Couldn't save image: {}", e)
    }
}
