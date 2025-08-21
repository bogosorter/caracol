use crate::geometry::ray::Ray;
use crate::geometry::vector::Vector;
use crate::geometry::sphere::Sphere;
use crate::config::*;

// Returns the color of the ray, using diffuse reflection
pub fn raytrace(ray: &Ray, bounces: u8) -> Vector {

    // Find the closest intersection

    let mut d: Option<f32> = None;
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

    let emitted = object.color * object.intensity;

    if bounces == 0 { return emitted };

    let intersection = ray.origin + distance * ray.direction;
    let normal = object.normal(&intersection);

    // Calculate the direction of the reflection. Must be in the half-space
    // defined by the intersection normal
    let random = Vector::random();
    let random_dot = normal.dot(&random);
    let new_direction = if random_dot >= 0. { random } else { -random };
    let new_ray = Ray::new(intersection, new_direction);
    let bounce_strength = random_dot.abs();

    
    let reflected = raytrace(&new_ray, bounces - 1).hadamard(&object.color) * bounce_strength;

    emitted + reflected
}
