use crate::geometry::vector::Vector;

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Vector,
    pub direction: Vector
}

impl Ray {
    pub fn new(origin: Vector, direction: Vector) -> Self {
        Self { origin, direction }
    }

    pub fn at(&self, distance: f64) -> Vector {
        self.origin + self.direction * distance
    }
}
