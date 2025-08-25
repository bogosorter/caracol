use image::RgbImage;
use bogotracer::renderer::raytracer::Raytracer;
use bogotracer::utils::utils;
use bogotracer::config::*;

fn main() {
    let raytracer = Raytracer::new(create_scene());
    let mut image = RgbImage::new(WIDTH, HEIGHT);

    for x in 0..WIDTH {
        utils::print_progress(x as f64 / WIDTH as f64);

        for y in 0.. HEIGHT {
            let color = raytracer.pixel_color(x, y);
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
