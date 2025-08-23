use rand::random;
use crate::geometry::vector::Vector;
use crate::geometry::ray::Ray;
use crate::config::*;

pub struct Camera {
    forward: Vector,
    right: Vector,
    up: Vector,
    focus_distance: f64,
    defocus_radius: f64
}

impl Camera {
    pub fn new() -> Self {
        let forward = (LOOK_AT - CAMERA_POSITION).normalize();
        let right = forward.cross(&VIEW_UP).normalize();
        let up = right.cross(&forward).normalize();
        let focus_distance = (LOOK_AT - CAMERA_POSITION).magnitude();
        let defocus_radius = focus_distance * DEFOCUS_ANGLE.sin();

        Self {
            forward,
            right,
            up,
            focus_distance,
            defocus_radius
        }
    }

    pub fn ray(&self, x: u32, y: u32) -> Ray {
        // Two distortions are applied to the ray:
        // - A distortion is applied to the ray origin within a given
        //   defocus_radius, which creates a focus/defocus effect the
        //   farther away from the LOOK_AT point
        // - A distortion is applied to the ray's position at the LOOK_AT
        //   point, which create an antialiasing effect

        // Calculating the starting position
        // https://stackoverflow.com/questions/5837572/generate-a-random-point-within-a-circle-uniformly
        let r = self.defocus_radius * random::<f64>();
        let theta = random::<f64>() * 2. * std::f64::consts::PI;
        let start = Vector::new(
            r * theta.cos(),
            r * theta.sin(),
            0.
        );

        // Calculating the end position. Top left corner is given by
        // (-tan(FOV_ANGLE / 2), tan(FOV_ANGLE / 2) / ASPECT_RATIO)
        let offset_x = random::<f64>() - 0.5;
        let offset_y = random::<f64>() - 0.5;
        let end = Vector::new(
            (FOV_ANGLE / 2.).tan() * (-1. + (x as f64 + 0.5 + offset_x) * 2. / WIDTH as f64),
            (FOV_ANGLE / 2.).tan() / ASPECT_RATIO * (1. - (y as f64 + 0.5 + offset_y) * 2. / HEIGHT as f64),
            - 1.
        ) * self.focus_distance;

        let direction = end - start;

        // Transform the ray's coordinates from camera space to world space
        let world_position = CAMERA_POSITION + start.x * self.right + start.y * self.up;
        let world_direction = direction.x * self.right + direction.y * self.up - direction.z * self.forward;
        Ray::new(world_position, world_direction.normalize())
    }
}
