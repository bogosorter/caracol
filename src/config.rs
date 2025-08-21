use crate::geometry::vector::Vector;
use crate::geometry::sphere::Sphere;

// Image settings
pub const WIDTH: u32 = 1920;
pub const HEIGHT: u32 = 1080;

// Camera settings
pub const FOV_ANGLE: f32 = std::f32::consts::FRAC_PI_3;
pub const CAMERA_POSITION: Vector = Vector::new(0., 0., -20.);
pub const FORWARD: Vector = Vector::new(0., 0., 1.);
pub const RIGHT: Vector = Vector::new(1., 0., 0.);
pub const UP: Vector = RIGHT.cross(&FORWARD);
pub const VOID: Vector = Vector::ZERO;

// Scene settings
pub const BOUNCES: u8 = 3;
pub const OBJECTS: [Sphere; 5] = [
    Sphere::new(Vector::new(0., 0.3, -13.), 1., Vector::new(1., 0., 0.), 0.),
    Sphere::new(Vector::new(3., 0.5, -5.), 0.5, Vector::new(0., 1., 0.), 0.),
    Sphere::new(Vector::new(-3., 1.8, -6.), 2., Vector::new(0., 0., 1.), 0.),
    Sphere::new(Vector::new(0., -100., 0.), 100., Vector::new(0.98, 0.73, 0.05), 0.),
    Sphere::new(Vector::new(60., 120., -20.), 80., Vector::new(1., 1., 1.), 10.)
];
