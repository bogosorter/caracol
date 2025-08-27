use std::rc::Rc;
use crate::geometry::ray::Ray;
use crate::geometry::hitbox::HitBox;
use crate::geometry::vector::Vector;
use crate::scene::elements::{SceneElement, CollisionInfo};

pub fn build_bvh(mut elements: Vec<Rc<dyn SceneElement>>) -> Rc<dyn SceneElement> {
    if elements.len() == 0 { return Rc::new(EmptyBVHNode::new()); }
    if elements.len() == 1 { return elements.remove(0); }

    let n = elements.len();

    let mut hitbox: HitBox = *elements[0].hitbox();
    for i in 1..n {
        hitbox.merge(elements[i].hitbox());
    }

    elements.sort_by(|a, b| a.hitbox().compare(b.hitbox(), hitbox.longest_axis()));

    let mid = n / 2;
    let left = build_bvh(elements.drain(0..mid).collect());
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
    fn collide(&self, ray: &Ray) -> Option<CollisionInfo> {
        if !self.hitbox.intersects(ray) { return None }

        let l = self.left.collide(ray);
        let r = self.right.collide(ray);

        if l.is_none() { return r }
        if r.is_none() { return l }

        let left = l.unwrap();
        let right = r.unwrap();

        if left.distance < right.distance { return Some(left) }
        Some(right)
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
    fn collide(&self, _: &Ray) -> Option<CollisionInfo> { None }
    fn hitbox(&self) -> &HitBox {
        &self.hitbox
    }
}
