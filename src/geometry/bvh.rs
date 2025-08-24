use std::rc::Rc;
use crate::geometry::ray::Ray;
use crate::geometry::hitbox::HitBox;
use crate::geometry::scene_element::{SceneElement, CollisionInfo};

struct BVHNode {
    left: Rc<dyn SceneElement>,
    right: Rc<dyn SceneElement>,
    pub hitbox: HitBox
}

impl BVHNode {
    pub fn new(left: Rc<dyn SceneElement>, right: Rc<dyn SceneElement>) -> Self {
        let hitbox = left.hitbox() + right.hitbox();
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

    fn hitbox(&self) -> HitBox {
        self.hitbox
    }
}

pub fn build_bvh(mut elements: Vec<Rc<dyn SceneElement>>) -> Option<Rc<dyn SceneElement>> {
    if elements.len() == 0 { return None }
    if elements.len() == 1 { return Some(elements.remove(0)) }

    let mut result = elements.remove(0);
    for element in elements {
        result = Rc::new(BVHNode::new(result, element));
    }

    Some(result)
}
