use std::mem::swap;
use std::ops::Add;
use crate::geometry::vector::Vector;
use crate::geometry::ray::Ray;
use crate::config::EPSILON;

#[derive(Clone, Copy)]
pub struct HitBox {
    start: Vector,
    end: Vector
}

impl HitBox {
    pub const fn new(start: Vector, end: Vector) -> Self {
        Self {
            start,
            end
        }
    }

    pub fn intersects(&self, ray: &Ray) -> bool {
        let mut tmin = f64::NEG_INFINITY;
        let mut tmax = f64::INFINITY;
    
        // X slab
        if ray.direction.x.abs() < EPSILON {
            if ray.origin.x < self.start.x || ray.origin.x > self.end.x {
                return false;
            }
        } else {
            let mut tx1 = (self.start.x - ray.origin.x) / ray.direction.x;
            let mut tx2 = (self.end.x - ray.origin.x) / ray.direction.x;
            if tx1 > tx2 { swap(&mut tx1, &mut tx2); }
            tmin = tmin.max(tx1);
            tmax = tmax.min(tx2);
        }
    
        // Y slab
        if ray.direction.y.abs() < EPSILON {
            if ray.origin.y < self.start.y || ray.origin.y > self.end.y {
                return false;
            }
        } else {
            let mut ty1 = (self.start.y - ray.origin.y) / ray.direction.y;
            let mut ty2 = (self.end.y - ray.origin.y) / ray.direction.y;
            if ty1 > ty2 { swap(&mut ty1, &mut ty2); }
            tmin = tmin.max(ty1);
            tmax = tmax.min(ty2);
        }
    
        // Z slab
        if ray.direction.z.abs() < EPSILON {
            if ray.origin.z < self.start.z || ray.origin.z > self.end.z {
                return false;
            }
        } else {
            let mut tz1 = (self.start.z - ray.origin.z) / ray.direction.z;
            let mut tz2 = (self.end.z - ray.origin.z) / ray.direction.z;
            if tz1 > tz2 { swap(&mut tz1, &mut tz2); }
            tmin = tmin.max(tz1);
            tmax = tmax.min(tz2);
        }
    
        tmin <= tmax && tmax >= 0.0
    }
}

impl Add<HitBox> for HitBox {
    type Output = HitBox;
    fn add(self, other: HitBox) -> HitBox {
        HitBox::new(
            Vector::new(self.start.x.min(other.start.x), self.start.y.min(other.start.y), self.start.z.min(other.start.z)),
            Vector::new(self.end.x.max(other.end.x), self.end.y.max(other.end.y), self.end.z.max(other.end.z))
        )
    }
}
