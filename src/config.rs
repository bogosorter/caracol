use once_cell::sync::Lazy;
use crate::geometry::vector::Vector;
use crate::geometry::sphere::Sphere;
use crate::geometry::material::{DiffuseMaterial, ReflectiveMaterial};

// Image settings
pub const ASPECT_RATIO: f64 = 16./9.;
pub const WIDTH: u32 = 1920;
pub const HEIGHT: u32 = (WIDTH as f64 / ASPECT_RATIO) as u32;

// Camera settings
pub const FOV_ANGLE: f64 = std::f64::consts::FRAC_PI_3;
pub const CAMERA_POSITION: Vector = Vector::new(0., 0., -20.);
pub const FORWARD: Vector = Vector::new(0., 0., 1.);
pub const RIGHT: Vector = Vector::new(1., 0., 0.);
pub const UP: Vector = RIGHT.cross(&FORWARD);
pub const VOID: Vector = Vector::new(0.3, 0.3, 0.3);

// Raytracing settings
pub const ITERATIONS: u32 = 40;
pub const BOUNCES: u8 = 5;
pub const EPSILON: f64 = 1e-6;

// Scene settings
pub static OBJECTS: Lazy<Vec<Sphere>> = Lazy::new( || {
    vec![
        Sphere::new(Vector::new(0., 0.3, -13.), 1., Box::new(DiffuseMaterial::new(Vector::new(1., 0., 1.), 0.))),
        Sphere::new(Vector::new(3., 0.5, -5.), 0.5, Box::new(DiffuseMaterial::new(Vector::new(0., 1., 0.), 0.))),
        Sphere::new(Vector::new(-3., 1.8, -6.), 2., Box::new(ReflectiveMaterial::new(Vector::new(0.9, 0.9, 0.9), 0., 1.))),
        Sphere::new(Vector::new(-3., 0., -13.), 0.8, Box::new(ReflectiveMaterial::new(Vector::new(0.9, 0.9, 0.9), 0., 0.8))),
        Sphere::new(Vector::new(0., -100., 0.), 100., Box::new(DiffuseMaterial::new(Vector::new(0.5, 0.35, 0.03), 0.))),
        Sphere::new(Vector::new(60., 120., -40.), 80., Box::new(DiffuseMaterial::new(Vector::new(1., 1., 1.), 3.)))
    ]
});
