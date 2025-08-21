use crate::geometry::ray::Ray;
use crate::geometry::vector::Vector;
use crate::geometry::sphere::Sphere;
use crate::config::*;

pub fn pixel_color(x: u32, y: u32) -> Vector {
    // Calculate ray direction relative to camera. Top left corner is
    // given by (-tan(FOV_ANGLE / 2), tan(FOV_ANGLE / 2) * HEIGHT / WIDTH)
    let ray_x = (FOV_ANGLE / 2.).tan() * (-1. + (x as f32 + 0.5) * 2. / WIDTH as f32);
    let ray_y = (FOV_ANGLE / 2.).tan() * HEIGHT as f32 / WIDTH as f32 * (1. - (y as f32 + 0.5) * 2. / HEIGHT as f32);

    // Transform the ray's direction coordinates from camera space to
    // world space
    let world_direction = (ray_x * RIGHT + ray_y * UP + FORWARD).normalize();
    let ray = Ray::new(CAMERA_POSITION, world_direction);

    raytrace(&ray, BOUNCES)
}

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
