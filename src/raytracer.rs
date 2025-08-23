use crate::geometry::ray::Ray;
use crate::geometry::vector::Vector;
use crate::geometry::sphere::Sphere;
use crate::camera::Camera;
use crate::config::*;

pub fn pixel_color(camera: &Camera, x: u32, y: u32) -> Vector {
    let mut result = Vector::ZERO;

    for _ in 0..ITERATIONS {
        result += raytrace(&camera.ray(x, y), BOUNCES);
    }

    result /= ITERATIONS as f64;
    result.clamp(0., 1.)
}

// Returns the color of the ray, using diffuse reflection
fn raytrace(ray: &Ray, bounces: u8) -> Vector {

    // Find the closest intersection

    let mut d: Option<f64> = None;
    let mut obj: Option<&Sphere> = None;

    for object in OBJECTS.iter() {
        let Some(distance) = ray.shoot(object) else { continue };
        if d.is_none() || distance < d.unwrap() {
            d = Some(distance);
            obj = Some(object);
        };
    }

    if d.is_none() { return VOID };

    // Compute the pixel's color given the intersected object and the number of
    // bounces left

    let distance = d.unwrap();
    let object = obj.unwrap();

    if bounces == 0 { return object.material.emission() };

    let mut intersection = ray.origin + distance * ray.direction;
    let normal = object.normal(&intersection);
    // Ading normal * EPSILON helps to prevent shadow acne
    intersection += normal * EPSILON;

    let emitted = object.material.emission();
    let reflected = raytrace(&object.material.reflect(&ray, &intersection, &normal), bounces - 1).hadamard(&object.material.albedo());

    emitted + reflected
}
