use crate::geometry::vector::Vector;
use crate::geometry::material::Material;

#[derive(Debug, Clone, Copy)]
pub struct Sphere {
    pub center: Vector,
    pub radius: f32,
    pub material: Material
}

impl Sphere {
    pub const fn new(center: Vector, radius: f32, material: Material) -> Sphere {
        Sphere { center, radius, material }
    }

    pub fn normal(self, point: &Vector) -> Vector {
        (point - self.center).normalize()
    }
}
