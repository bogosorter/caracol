use std::rc::Rc;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use ordered_float::OrderedFloat;
use crate::geometry::ray::Ray;
use crate::geometry::hitbox::HitBox;
use crate::geometry::vector::Vector;
use crate::scene::elements::{SceneElement, CollisionInfo};

pub fn build_bvh(mut elements: Vec<Rc<dyn SceneElement>>) -> Rc<dyn SceneElement> {
    if elements.len() == 0 { return Rc::new(EmptyBVHNode::new()); }
    if elements.len() == 1 { return elements.remove(0); }

    // The BVH is built using greedy bottom-up agglomerative clustering. For all
    // pairs of elements, the hitbox of a possible node constructed using those
    // elements is calculated. The pairs are ordered in a min-heap and, at each
    // step, a pair is extracted. A node is created and placed in the BVH, and
    // used as one of the elements for further pairs, etc. The process ends once
    // n - 1 nodes have been created (the last node is the BVH's root node).

    let n = elements.len();

    // Create the heap (Reverse is used to induce min-heap behavior)
    let mut heap = BinaryHeap::new();
    for i in 0..n {
        for j in i + 1..n {
            let area = (elements[i].hitbox() + elements[j].hitbox()).area();
            heap.push((Reverse(OrderedFloat(area)), i, j));
        }
    }
    
    // All of the scene elements that are available to build the BVH. If not
    // available the value is None
    let mut elements: Vec<Option<Rc<dyn SceneElement>>> = elements.into_iter().map(Some).collect();
    
    let mut nodes_left = n - 1;
    while nodes_left > 0 {
        let min = heap.pop().unwrap();

        // The nodes may need elements that were already used by other nodes
        if elements[min.1].is_none() || elements[min.2].is_none() { continue; }

        // Create the new element
        let element = BVHNode::new(elements[min.1].take().unwrap(), elements[min.2].take().unwrap());
        
        // Compute the cost of possible new nodes and add them to the min-heap
        for i in 0..elements.len() {
            if let Some(element) = &elements[i] {
                let area = (element.hitbox() + element.hitbox()).area();
                heap.push((Reverse(OrderedFloat(area)), i, elements.len()));
            }
        }
        
        elements.push(Some(Rc::new(element)));
        nodes_left -= 1;
    }

    // The root node is the last to have been added to the candidates
    elements.pop().unwrap().unwrap()
}

struct BVHNode {
    left: Rc<dyn SceneElement>,
    right: Rc<dyn SceneElement>,
    hitbox: HitBox
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
