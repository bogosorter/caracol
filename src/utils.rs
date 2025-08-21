use image::Rgb;
use crate::geometry::vector::Vector;

pub fn to_rgb(vector: &Vector) -> Rgb<u8> {
    Rgb([(vector.x * 225.) as u8, (vector.y * 255.) as u8, (vector.z * 255.) as u8])
}
