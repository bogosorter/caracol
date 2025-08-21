use crate::geometry::vector::Vector;
use crate::geometry::sphere::Sphere;
use crate::geometry::material::{Material, SurfaceType};

// Image settings
pub const ASPECT_RATIO: f32 = 16./9.;
pub const WIDTH: u32 = 400;
pub const HEIGHT: u32 = (WIDTH as f32 / ASPECT_RATIO) as u32;

// Camera settings
pub const FOV_ANGLE: f32 = std::f32::consts::FRAC_PI_3;
pub const CAMERA_POSITION: Vector = Vector::new(0., 0., -20.);
pub const FORWARD: Vector = Vector::new(0., 0., 1.);
pub const RIGHT: Vector = Vector::new(1., 0., 0.);
pub const UP: Vector = RIGHT.cross(&FORWARD);
pub const VOID: Vector = Vector::new(0.3, 0.3, 0.3);

// Raytracing settings
pub const ITERATIONS: u32 = 1000;
pub const BOUNCES: u8 = 5;

// Scene settings
pub const OBJECTS: [Sphere; 5] = [
    Sphere::new(Vector::new(0., 0.3, -13.), 1., Material::new(Vector::new(1., 0., 1.), 0., SurfaceType::Diffuse)),
    Sphere::new(Vector::new(3., 0.5, -5.), 0.5, Material::new(Vector::new(0., 1., 0.), 0., SurfaceType::Diffuse)),
    Sphere::new(Vector::new(-3., 1.8, -6.), 2., Material::new(Vector::new(1., 1., 1.), 0., SurfaceType::Reflective)),
    Sphere::new(Vector::new(0., -100., 0.), 100., Material::new(Vector::new(0.5, 0.35, 0.03), 0., SurfaceType::Diffuse)),
    Sphere::new(Vector::new(60., 120., -20.), 80., Material::new(Vector::new(1., 1., 1.), 10., SurfaceType::Diffuse))
];
