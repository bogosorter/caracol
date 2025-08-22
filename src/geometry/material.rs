use crate::geometry::vector::Vector;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum SurfaceType {
    Diffuse,
    Reflective
}

#[derive(Debug, Clone, Copy)]
pub struct Material {
    pub color: Vector,
    pub intensity: f64,
    pub surface_type: SurfaceType
}

impl Material {
    pub const fn new(color: Vector, intensity: f64, surface_type: SurfaceType) -> Material {
        Material { color, intensity, surface_type }
    }
}
