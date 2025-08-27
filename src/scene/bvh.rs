use std::rc::Rc;
use crate::geometry::ray::Ray;
use crate::geometry::hitbox::HitBox;
use crate::geometry::vector::Vector;
use crate::scene::elements::{SceneElement, CollisionInfo};
use crate::utils::utils::Axis;

pub fn build_bvh(mut elements: Vec<Rc<dyn SceneElement>>) -> Rc<dyn SceneElement> {
    if elements.len() == 0 { return Rc::new(EmptyBVHNode::new()); }
    if elements.len() == 1 { return elements.remove(0); }

    let n = elements.len();
    let mut best_axis = Axis::X;
    let mut best_index = 0;
    let mut best_cost = f64::INFINITY;

    for axis in [Axis::X, Axis::Y, Axis::Z] {
        let mut lareas = vec![0.; n - 1];
        let mut rareas = vec![0.; n - 1];
        elements.sort_by(|a, b| a.hitbox().compare(b.hitbox(), axis));

        let mut lhitbox: HitBox = *elements[0].hitbox();
        lareas[0] = lhitbox.area();
        for i in 1..n - 1 {
            lhitbox.merge(elements[i].hitbox());
            lareas[i] = lhitbox.area();
        }

        let mut rhitbox: HitBox = *elements[n - 1].hitbox();
        rareas[n - 2] = rhitbox.area();
        for i in (0..n - 1).rev() {
            rhitbox.merge(elements[i].hitbox());
            rareas[i] = rhitbox.area();
        }

        for i in 0..n - 1 {
            let cost = lareas[i] * (i + 1) as f64 + rareas[i] * (n - i - 1) as f64;
            if cost < best_cost {
                best_axis = axis;
                best_index = i;
                best_cost = cost;
            }
        }
    }
    
    let mut hitbox = *elements[0].hitbox();
    for element in &elements {
        hitbox.merge(element.hitbox());
    }

    elements.sort_by(|a, b| a.hitbox().compare(b.hitbox(), best_axis));
    let left = build_bvh(elements.drain(0..=best_index).collect());
    let right = build_bvh(elements);

    Rc::new(BVHNode::new(left, right, hitbox))
}

struct BVHNode {
    left: Rc<dyn SceneElement>,
    right: Rc<dyn SceneElement>,
    hitbox: HitBox
}

impl BVHNode {
    pub fn new(left: Rc<dyn SceneElement>, right: Rc<dyn SceneElement>, hitbox: HitBox) -> Self {
        Self {left, right, hitbox}
    }
}

impl SceneElement for BVHNode {
    fn collide(&self, ray: &Ray, max_distance: f64) -> Option<CollisionInfo> {
        if !self.hitbox.intersects(ray, max_distance) { return None }

        if self.left.hitbox().distance(&ray.origin) < self.right.hitbox().distance(&ray.origin) {
            let l = self.left.collide(ray, max_distance);
            if l.is_none() { return self.right.collide(ray, max_distance) }
            let left = l.unwrap();

            let r = self.right.collide(ray, left.distance);
            if r.is_none() { return Some(left) }
            r
        } else {
            let r = self.right.collide(ray, max_distance);
            if r.is_none() { return self.left.collide(ray, max_distance) }
            let right = r.unwrap();

            let l = self.left.collide(ray, right.distance);
            if l.is_none() { return Some(right) }
            l
        }
    }

    fn hitbox(&self) -> &HitBox {
        &self.hitbox
    }
}

struct EmptyBVHNode {
    hitbox: HitBox
}

impl EmptyBVHNode {
    pub fn new() -> Self {
        let hitbox = HitBox::new(Vector::ZERO, Vector::ZERO);
        Self {hitbox}
    }
}

impl SceneElement for EmptyBVHNode {
    fn collide(&self, _: &Ray, _: f64) -> Option<CollisionInfo> { None }
    fn hitbox(&self) -> &HitBox {
        &self.hitbox
    }
}
