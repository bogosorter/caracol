use image::Rgb;
use crate::geometry::vector::Vector;

#[derive(Debug, Clone, Copy)]
pub struct Sphere {
    pub center: Vector,
    pub radius: f32,
    pub color: Rgb<u8>
}

impl Sphere {
    pub const fn new(center: Vector, radius: f32, color: Rgb<u8>) -> Sphere {
        Sphere { center, radius, color }
    }
}
