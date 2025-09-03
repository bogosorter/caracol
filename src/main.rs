use image::{RgbImage, Rgb};
use rayon::prelude::*;
use std::sync::atomic::{AtomicUsize, Ordering};
use caracol::renderer::raytracer::Raytracer;
use caracol::utils::utils::{self, to_rgb};
use caracol::config::*;

fn main() {
    let raytracer = Raytracer::new(create_scene());
    
    let progress = AtomicUsize::new(0);
    utils::print_progress(0.);
    
    // Render the image in parallel
    let pixels: Vec<(u32, u32, Rgb<u8>)> = (0..WIDTH).into_par_iter().flat_map(|x| -> Vec<(u32, u32, Rgb<u8>)> {
        let result = (0..HEIGHT).map(|y| {
            let color = raytracer.pixel_color(x, y);
            (x, y, to_rgb(&color))
        }).collect();

        let done = progress.fetch_add(1, Ordering::Relaxed);
        utils::print_progress(done as f64 / WIDTH as f64);

        result
    }).collect();
    
    utils::print_progress(1.);
    println!("");
    
    // Save the image as a .png file
    let mut image = RgbImage::new(WIDTH, HEIGHT);
    for (x, y, color) in pixels {
        image.put_pixel(x, y, color);
    }
    match image.save("output.png") {
        Ok(()) => println!("Image saved"),
        Err(e) => eprintln!("Couldn't save image: {}", e)
    }
}
