use crate::geometry::vector::Vector;

#[derive(Debug, Clone, Copy)]
pub struct Sphere {
    pub center: Vector,
    pub radius: f32,
    pub color: Vector,
    pub intensity: f32
}

impl Sphere {
    pub const fn new(center: Vector, radius: f32, color: Vector, intensity: f32) -> Sphere {
        Sphere { center, radius, color, intensity }
    }

    pub fn normal(self, point: &Vector) -> Vector {
        (point - self.center).normalize()
    }
}
