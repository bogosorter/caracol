use image::Rgb;
use crate::geometry::vector::Vector;

pub fn to_rgb(vector: &Vector) -> Rgb<u8> {
    Rgb([(vector.x * 255.) as u8, (vector.y * 255.) as u8, (vector.z * 255.) as u8])
}

pub fn print_progress(progress: f64) {
    let percentage = progress * 100.;
    print!("\r{percentage:.1}%");
    std::io::Write::flush(&mut std::io::stdout()).unwrap();
}

#[derive(Clone, Copy)]
pub enum Axis {
    X,
    Y,
    Z
}
