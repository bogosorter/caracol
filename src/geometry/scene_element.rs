use std::rc::Rc;
use crate::geometry::hitbox::HitBox;
use crate::geometry::vector::Vector;
use crate::geometry::material::Material;
use crate::geometry::ray::Ray;

pub struct CollisionInfo {
    pub distance: f64,
    pub normal: Vector,
    pub material: Rc<dyn Material>
}

impl CollisionInfo {
    pub fn new(distance: f64, normal: Vector, material: Rc<dyn Material>) -> Self {
        Self {
            distance,
            normal,
            material
        }
    }
}

pub trait SceneElement {
    fn collide(&self, ray: &Ray) -> Option<CollisionInfo>;
    fn hitbox(&self) -> HitBox;
}
