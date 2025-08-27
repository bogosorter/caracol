use std::mem::swap;
use std::cmp::Ordering;
use crate::geometry::vector::Vector;
use crate::geometry::ray::Ray;
use crate::utils::utils::Axis;
use crate::config::EPSILON;

#[derive(Clone, Copy)]
pub struct HitBox {
    start: Vector,
    end: Vector,
    center: Vector,
}

impl HitBox {
    pub fn new(start: Vector, end: Vector) -> Self {
        Self {
            start,
            end,
            center: (start + end) / 2.
        }
    }

    pub const fn area(&self) -> f64 {
        let width = self.end.x - self.start.x;
        let height = self.end.y - self.start.y;
        let depth = self.end.z - self.start.z;
        width * height * depth
    }

    pub fn merge(&mut self, other: &HitBox) {
        self.start.x = self.start.x.min(other.start.x);
        self.start.y = self.start.y.min(other.start.y);
        self.start.z = self.start.z.min(other.start.z);

        self.end.x = self.end.x.max(other.end.x);
        self.end.y = self.end.y.max(other.end.y);
        self.end.z = self.end.z.max(other.end.z);
    }
    
    // Compares the center of two hitboxes along a given axis
    pub fn compare(&self, other: &HitBox, axis: Axis) -> Ordering {
        match axis {
            Axis::X => {
                let scenter = self.start.x + self.end.x;
                let ocenter = other.start.x + other.end.x;
                scenter.partial_cmp(&ocenter).unwrap()
            },
            Axis::Y => {
                let scenter = self.start.y + self.end.y;
                let ocenter = other.start.y + other.end.y;
                scenter.partial_cmp(&ocenter).unwrap()
            },
            Axis::Z => {
                let scenter = self.start.z + self.end.z;
                let ocenter = other.start.z + other.end.z;
                scenter.partial_cmp(&ocenter).unwrap()
            }
        }
    }

    pub fn distance(&self, point: &Vector) -> f64 {
        (point - self.center).magnitude_sqr()
    }

    pub fn intersects(&self, ray: &Ray, max_distance: f64) -> bool {
        let mut tmin = f64::NEG_INFINITY;
        let mut tmax = max_distance;
    
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
