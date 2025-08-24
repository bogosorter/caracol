use crate::geometry::vector::Vector;
use crate::geometry::material::Material;
use crate::geometry::ray::Ray;
use crate::geometry::scene_element::{SceneElement, CollisionInfo};

pub struct Sphere {
    pub center: Vector,
    pub radius: f64,
    pub material: Box<dyn Material>
}

impl Sphere {
    pub const fn new(center: Vector, radius: f64, material: Box<dyn Material>) -> Self {
        Self { center, radius, material }
    }

    // Returns the intersection distance of a ray and a sphere, if any
    fn distance(&self, ray: &Ray) -> Option<f64> {
        let oc = ray.origin - self.center;
        let a = ray.direction.dot(&ray.direction);
        let b = 2. * oc.dot(&ray.direction);
        let c = oc.dot(&oc) - self.radius * self.radius;

        let discriminant = b * b - 4. * a * c;
        if discriminant < 0. {
            return None;
        }

        let t1 = (-b - discriminant.sqrt()) / (2. * a);
        if t1 > 0. {
            return Some(t1);
        }

        let t2 = (-b + discriminant.sqrt()) / (2. * a);
        if t2 > 0. {
            return Some(t2);
        }

        None
    }
}

impl SceneElement for Sphere {
    fn collide(&self, ray: &Ray) -> Option<CollisionInfo> {
        let d = self.distance(ray)?;
        let point = ray.at(d);

        // Normalizing a vector is expensive, since it requires calculating a
        // square root. For a sphere, it is possible to just divide by its
        // radius
        let normal = (point - self.center) / self.radius;

        Some(CollisionInfo::new(d, normal, &*self.material))
    }
}
