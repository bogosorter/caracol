use crate::geometry::vector::Vector;
use crate::geometry::material::Material;
use crate::geometry::ray::Ray;

pub struct CollisionInfo<'a> {
    pub distance: f64,
    pub normal: Vector,
    pub material: &'a dyn Material
}

impl<'a> CollisionInfo<'a> {
    pub fn new(distance: f64, normal: Vector, material: &'a dyn Material) -> Self {
        Self {
            distance,
            normal,
            material
        }
    }
}

pub trait SceneElement {
    fn collide(&self, ray: &Ray) -> Option<CollisionInfo>;
}
