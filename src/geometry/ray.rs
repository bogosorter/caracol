use crate::geometry::vector::Vector;
use crate::geometry::sphere::Sphere;

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Vector,
    pub direction: Vector
}

impl Ray {
    pub fn new(origin: Vector, direction: Vector) -> Ray {
        Ray { origin, direction }
    }

    // Returns the intersection point of a ray and a sphere, if any
    pub fn shoot(&self, sphere: &Sphere) -> Option<f32> {
        let oc = self.origin - sphere.center;
        let a = self.direction.dot(&self.direction);
        let b = 2. * oc.dot(&self.direction);
        let c = oc.dot(&oc) - sphere.radius * sphere.radius;

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
