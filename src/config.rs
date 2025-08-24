use once_cell::sync::Lazy;
use rand::random;
use crate::geometry::vector::Vector;
use crate::geometry::sphere::Sphere;
use crate::geometry::material::{DiffuseMaterial, ReflectiveMaterial};
use crate::geometry::scene_element::SceneElement;

// Image settings
pub const ASPECT_RATIO: f64 = 16./9.;
pub const WIDTH: u32 = 1920;
pub const HEIGHT: u32 = (WIDTH as f64 / ASPECT_RATIO) as u32;

// Camera settings
pub const FOV_ANGLE: f64 = std::f64::consts::FRAC_PI_3;
pub const CAMERA_POSITION: Vector = Vector::new(0., 10., 30.);
pub const LOOK_AT: Vector = Vector::new(0., 0., 0.);
pub const VIEW_UP: Vector = Vector::new(0., 1., -0.);
pub const DEFOCUS_ANGLE: f64 = 0.01;
pub const VOID: Vector = Vector::new(0.3, 0.3, 0.3);

// Raytracing settings
pub const ITERATIONS: u32 = 10;
pub const BOUNCES: u8 = 5;
pub const EPSILON: f64 = 1e-6;
