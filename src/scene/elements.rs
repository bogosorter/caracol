use std::rc::Rc;
use crate::geometry::hitbox::HitBox;
use crate::geometry::vector::Vector;
use crate::geometry::ray::Ray;
use crate::scene::materials::Material;
use crate::config::*;

pub trait SceneElement {
    fn collide(&self, ray: &Ray) -> Option<CollisionInfo>;
    fn hitbox(&self) -> &HitBox;
}

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


pub struct Sphere {
    center: Vector,
    radius: f64,
    material: Rc<dyn Material>,
    hitbox: HitBox
}

impl Sphere {
    pub const fn new(center: Vector, radius: f64, material: Rc<dyn Material>) -> Self {
        let hitbox = HitBox::new(
            Vector::new(center.x - radius, center.y - radius, center.z - radius),
            Vector::new(center.x + radius, center.y + radius, center.z + radius)
        );
        Self {center, radius, material, hitbox}
    }

    // Returns the intersection distance of a ray and a sphere, if any
    fn distance(&self, ray: &Ray) -> Option<f64> {
        if !self.hitbox.intersects(ray) { return None }

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

        Some(CollisionInfo::new(d, normal, self.material.clone()))
    }

    fn hitbox(&self) -> &HitBox {
        &self.hitbox
    }
}

// Plane is not a SceneElement since that would require implementing an infinite
// HitBox
pub struct Plane {
    point: Vector,
    normal: Vector
}

impl Plane {
    fn new(point: Vector, normal: Vector) -> Self {
        Self {point, normal}
    }

    fn collide(&self, ray: &Ray) -> Option<f64> {
        let w = self.point - ray.origin;
        let a = w.dot(&self.normal);
        let b = ray.direction.dot(&self.normal);

        // Ray is parallel to the plane
        if b.abs() < EPSILON { return None }
        
        let distance = a / b;
        if distance < 0. { return None }
        Some(distance)
    }
}

pub struct Triangle {
    a: Vector,
    b: Vector,
    material: Rc<dyn Material>,

    normal: Vector,
    plane: Plane,
    hitbox: HitBox,

    // This cached data is used to improve efficiency in collision detection
    ac: Vector,
    bc: Vector,
    barycentric_a: Vector,
    barycentric_b: Vector
}

impl Triangle {
    pub fn new(a: Vector, b: Vector, c: Vector, material: Rc<dyn Material>) -> Self {
        let ac = c - a;
        let bc = c - b;
        let barycentric_a = ac - ac.project(&bc);
        let barycentric_b = bc - bc.project(&ac);

        let normal = ac.cross(&bc).normalize();
        let plane = Plane::new(a, normal);
        let hitbox = HitBox::new(
            Vector::new(a.x.min(b.x).min(c.x), a.y.min(b.y).min(c.y), a.z.min(b.z).min(c.z)),
            Vector::new(a.x.max(b.x).max(c.x), a.y.max(b.y).max(c.y), a.z.max(b.z).max(c.z))
        );

        Self {a, b, material, normal, plane, hitbox, ac, bc, barycentric_a, barycentric_b}
    }
}

impl SceneElement for Triangle {
    fn collide(&self, ray: &Ray) -> Option<CollisionInfo> {
        if !self.hitbox.intersects(ray) { return None }

        let distance = self.plane.collide(ray)?;
        let point = ray.at(distance);

        // Check if the point is inside the triangle using barycentric coordinates

        // First barycentric coordinate 
        let ah = point - self.a;
        let a = 1. - self.barycentric_a.dot(&ah) / self.barycentric_a.dot(&self.ac);
        if a < 0. { return None }

        // Second barycentric coordinate
        let bh = point - self.b;
        let b = 1. - self.barycentric_b.dot(&bh) / self.barycentric_b.dot(&self.bc);
        if b < 0. { return None }

        let c = 1. - a - b;
        if c < 0. { return None }

        // We consider the normal to always face the origin of the ray
        if self.normal.dot(&ray.direction) >= 0. {
            Some(CollisionInfo::new(distance, -self.normal, self.material.clone()))
        } else {
            Some(CollisionInfo::new(distance, self.normal, self.material.clone()))
        }
    }

    fn hitbox(&self) -> &HitBox {
        &self.hitbox
    }
}
