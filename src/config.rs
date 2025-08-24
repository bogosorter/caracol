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
pub const ITERATIONS: u32 = 40;
pub const BOUNCES: u8 = 5;
pub const EPSILON: f64 = 1e-6;

// Scene settings
pub static ELEMENTS: Lazy<Vec<Box<dyn SceneElement + Send + Sync>>> = Lazy::new( || {
    let mut result: Vec<Box<dyn SceneElement + Send + Sync>> = Vec::new();

    for i in 0..10 {
        for j in 0..10 {
            let radius = random::<f64>();
            result.push(Box::new(Sphere::new(
                Vector::new((i * 2 - 5 * 2) as f64, radius, (j * 2 - 5 * 2) as f64), radius,
                Box::new(DiffuseMaterial::new(Vector::new(random::<f64>(), random::<f64>(), random::<f64>()), 0.)))
            ));
        }
    }

    result.push(Box::new(Sphere::new(
        Vector::new(60., 60., 0.), 40.,
        Box::new(DiffuseMaterial::new(Vector::new(1., 1., 1.), 3.)))
    ));

    result
});
