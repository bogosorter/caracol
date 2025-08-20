use image::Rgb;
use crate::geometry::vector::Vector;
use crate::geometry::sphere::Sphere;

// Image settings
pub const WIDTH: u32 = 1920;
pub const HEIGHT: u32 = 1080;

// Camera settings
pub const FOV_ANGLE: f32 = std::f32::consts::FRAC_PI_3;
pub const CAMERA_POSITION: Vector = Vector::new(0., 0., -10.);
pub const FORWARD: Vector = Vector::new(0., 0., 1.);
pub const RIGHT: Vector = Vector::new(1., 0., 0.);
pub const UP: Vector = RIGHT.cross(&FORWARD);

// Scene settings
pub const OBJECTS: [Sphere; 3] = [
    Sphere::new(Vector::new(0., 0., 0.), 1., Rgb([255, 0, 0])),
    Sphere::new(Vector::new(3., 0., -2.), 0.5, Rgb([0, 255, 0])),
    Sphere::new(Vector::new(-3., 0., 3.), 2., Rgb([0, 0, 255]))
];

