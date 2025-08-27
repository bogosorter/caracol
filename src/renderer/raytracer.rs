use std::sync::Arc;
use crate::geometry::ray::Ray;
use crate::geometry::vector::Vector;
use crate::scene::elements::SceneElement;
use crate::scene::bvh::build_bvh;
use crate::renderer::camera::Camera;
use crate::config::*;

pub struct Raytracer {
    camera: Camera,
    bvh: Arc<dyn SceneElement>

}

impl Raytracer {
    pub fn new(scene: Vec<Arc<dyn SceneElement>>) -> Self {
        Self {
            camera: Camera::new(),
            bvh: build_bvh(scene)
        }
    }

    pub fn pixel_color(&self, x: u32, y: u32) -> Vector {
        let mut result = Vector::ZERO;
    
        for _ in 0..ITERATIONS {
            result += self.raytrace(&self.camera.ray(x, y), BOUNCES);
        }
    
        result /= ITERATIONS as f64;
        result.clamp(0., 1.)
    }
    
    // Returns the color of the ray, using diffuse reflection
    fn raytrace(&self, ray: &Ray, bounces: u8) -> Vector {
    
        // Find the closest collision
        let collision = self.bvh.collide(ray, f64::INFINITY);
    
        if collision.is_none() { return VOID }
        let info = collision.unwrap();
    
        // Compute the pixel's color
    
        if bounces == 0 { return info.material.emission() };
    
        // Adding normal * EPSILON helps to prevent shadow acne
        let intersection = ray.at(info.distance) + info.normal * EPSILON;
    
        let emitted = info.material.emission();
        let reflected_ray = &info.material.reflect(&ray, &intersection, &info.normal);
        let reflected = self.raytrace(reflected_ray, bounces - 1).hadamard(&info.material.albedo());
    
        emitted + reflected
    }

}
