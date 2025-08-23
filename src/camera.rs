use rand::random;
use crate::geometry::vector::Vector;
use crate::geometry::ray::Ray;
use crate::config::*;

pub struct Camera {
    forward: Vector,
    right: Vector,
    up: Vector
}

impl Camera {
    pub fn new() -> Self {
        let forward = (LOOK_AT - CAMERA_POSITION).normalize();
        let right = LOOK_AT.cross(&VIEW_UP).normalize();
        let up = right.cross(&forward).normalize();

        Self {
            forward,
            right,
            up
        }
    }

    pub fn ray(&self, x: u32, y: u32) -> Ray {
        // To create an antialising effect, we add a little offset to the starting
        // ray position
        let offset_x = random::<f64>() - 0.5;
        let offset_y = random::<f64>() - 0.5;

        // Top left corner is given by
        // (-tan(FOV_ANGLE / 2), tan(FOV_ANGLE / 2) * HEIGHT / WIDTH)
        let ray_x = (FOV_ANGLE / 2.).tan() * (-1. + (x as f64 + 0.5 + offset_x) * 2. / WIDTH as f64);
        let ray_y = (FOV_ANGLE / 2.).tan() / ASPECT_RATIO * (1. - (y as f64 + 0.5 + offset_y) * 2. / HEIGHT as f64);

        // Transform the ray's direction coordinates from camera space to
        // world space
        let world_direction = ray_x * self.right + ray_y * self.up + self.forward;
        Ray::new(CAMERA_POSITION, world_direction)
    }
}
