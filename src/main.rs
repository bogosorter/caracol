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
use rand::random;
use crate::geometry::vector::Vector;
use crate::geometry::sphere::Sphere;
use crate::geometry::material::{DiffuseMaterial};
use crate::geometry::scene_element::SceneElement;

fn main() {
    let camera = Camera::new();
    let mut image = RgbImage::new(WIDTH, HEIGHT);

    let mut elements: Vec<Box<dyn SceneElement + Send + Sync>> = Vec::new();

    elements.push(Box::new(Sphere::new(
        Vector::new(60., 60., 0.), 40.,
        Box::new(DiffuseMaterial::new(Vector::new(1., 1., 1.), 3.)))
    ));

    for i in 0..3 {
        for j in 0..3 {
            let radius = random::<f64>();
            elements.push(Box::new(Sphere::new(
                Vector::new((i * 2 - 5 * 2) as f64, radius, (j * 2 - 5 * 2) as f64), radius,
                Box::new(DiffuseMaterial::new(Vector::new(random::<f64>(), random::<f64>(), random::<f64>()), 0.)))
            ));
        }
    }

    if let Some(bvh) = build_bvh(elements) {
        for x in 0..WIDTH {
            utils::print_progress(x as f64 / WIDTH as f64);
    
            for y in 0.. HEIGHT {
                let color = pixel_color(&camera, &bvh, x, y);
                image.put_pixel(x, y, utils::to_rgb(&color));
            }
        }
    
        utils::print_progress(1.);
        println!("");
        
        match image.save("output.png") {
            Ok(()) => println!("Image saved"),
            Err(e) => eprintln!("Couldn't save image: {}", e)
        }
    } else {
        println!("Couldn't build bvh");
    }
}
