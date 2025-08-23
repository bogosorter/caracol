use crate::geometry::vector::Vector;
use crate::geometry::material::Material;

pub struct Sphere {
    pub center: Vector,
    pub radius: f64,
    pub material: Box<dyn Material>
}

impl Sphere {
    pub const fn new(center: Vector, radius: f64, material: Box<dyn Material>) -> Self {
        Self { center, radius, material }
    }

    pub fn normal(&self, point: &Vector) -> Vector {
        // Normalizing a vector is expensive, since it requires calculating a
        // square root. For a sphere, it is possible to just divide by its
        // radius
        (point - self.center) / self.radius
    }
}
