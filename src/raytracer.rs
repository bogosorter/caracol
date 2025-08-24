use crate::geometry::ray::Ray;
use crate::geometry::vector::Vector;
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

    // Find the closest collision
    let collision = ELEMENTS
        .iter()
        .filter_map(|obj| obj.collide(ray))
        .min_by(|a, b| a.distance.total_cmp(&b.distance));

    if collision.is_none() { return VOID }
    let info = collision.unwrap();

    // Compute the pixel's color

    if bounces == 0 { return info.material.emission() };

    // Adding normal * EPSILON helps to prevent shadow acne
    let intersection = ray.at(info.distance) + info.normal * EPSILON;

    let emitted = info.material.emission();
    let reflected_ray = &info.material.reflect(&ray, &intersection, &info.normal);
    let reflected = raytrace(reflected_ray, bounces - 1).hadamard(&info.material.albedo());

    emitted + reflected
}
