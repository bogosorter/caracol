use crate::geometry::vector::Vector;
use crate::geometry::material::Material;

#[derive(Debug, Clone, Copy)]
pub struct Sphere {
    pub center: Vector,
    pub radius: f64,
    pub material: Material
}

impl Sphere {
    pub const fn new(center: Vector, radius: f64, material: Material) -> Sphere {
        Sphere { center, radius, material }
    }

    pub fn normal(self, point: &Vector) -> Vector {
        // Normalizing a vector is expensive, since it requires calculating a
        // square root. For a sphere, it is possible to just divide by its
        // radius
        (point - self.center) / self.radius
    }
}
