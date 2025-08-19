use image::{RgbImage, Rgb};

fn main() {
    let width = 1920;
    let height = 1080;
    let mut image = RgbImage::new(width, height);

    for x in 0..width {
        for y in 0.. height {
            image.put_pixel(x, y, Rgb([(x * 255 / width) as u8, (y * 255 / height) as u8, 100]));
        }
    }

    match image.save("output.png") {
        Ok(()) => println!("Image saved"),
        Err(e) => eprintln!("Couldn't save image: {}", e)
    }
}
