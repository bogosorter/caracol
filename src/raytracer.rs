use rand::random;
use crate::geometry::ray::Ray;
use crate::geometry::vector::Vector;
use crate::geometry::sphere::Sphere;
use crate::geometry::material::SurfaceType;
use crate::config::*;

pub fn pixel_color(x: u32, y: u32) -> Vector {
    let mut result = Vector::ZERO;
    for _ in 0..ITERATIONS {
        let ray = ray_for_pixel(x, y);
        result += raytrace(&ray, BOUNCES);
    }

    result / ITERATIONS as f64
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

    let emitted = object.material.color * object.material.intensity;

    if bounces == 0 { return emitted };

    let mut intersection = ray.origin + distance * ray.direction;
    let normal = object.normal(&intersection);
    intersection += normal * EPSILON;

    let reflected = if object.material.surface_type == SurfaceType::Diffuse {
        // Calculate the direction of the reflection. Must be in the half-space
        // defined by the intersection normal
        let random = Vector::random();
        let random_dot = normal.dot(&random);
        let new_direction = if random_dot >= 0. { random } else { -random };
        let new_ray = Ray::new(intersection, new_direction);
        let bounce_strength = random_dot.abs();
        
        raytrace(&new_ray, bounces - 1).hadamard(&object.material.color) * bounce_strength
    } else {
        let new_direction = ray.direction - 2. * ray.direction.dot(&normal) * normal;
        let new_ray = Ray::new(intersection, new_direction);

        raytrace(&new_ray, bounces - 1).hadamard(&object.material.color)
    };

    emitted + reflected
}

fn ray_for_pixel(x: u32, y: u32) -> Ray {
    // To create an antialising effect, we add a little offset to the starting
    // ray position
    let offset_x = random::<f64>() - 0.5;
    let offset_y = random::<f64>() - 0.5;

    // Calculate ray direction relative to camera. Top left corner is
    // given by (-tan(FOV_ANGLE / 2), tan(FOV_ANGLE / 2) * HEIGHT / WIDTH)
    let ray_x = (FOV_ANGLE / 2.).tan() * (-1. + (x as f64 + 0.5 + offset_x) * 2. / WIDTH as f64);
    let ray_y = (FOV_ANGLE / 2.).tan() / ASPECT_RATIO * (1. - (y as f64 + 0.5 + offset_y) * 2. / HEIGHT as f64);

    // Transform the ray's direction coordinates from camera space to
    // world space
    let world_direction = ray_x * RIGHT + ray_y * UP + FORWARD;
    Ray::new(CAMERA_POSITION, world_direction)
}
