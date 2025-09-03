use std::sync::Arc;
use crate::geometry::vector::Vector;
use crate::scene::materials::{DiffuseMaterial};
use crate::scene::elements::{SceneElement, Triangle};
use crate::utils::reader::read_obj;

// Image settings
pub const ASPECT_RATIO: f64 = 16./9.;
pub const WIDTH: u32 = 1920;
pub const HEIGHT: u32 = (WIDTH as f64 / ASPECT_RATIO) as u32;

// Camera settings
pub const FOV_ANGLE: f64 = std::f64::consts::FRAC_PI_3;
pub const CAMERA_POSITION: Vector = Vector::new(0., 1.5, 6.);
pub const LOOK_AT: Vector = Vector::new(0., 1., 1.);
pub const VIEW_UP: Vector = Vector::new(0., 1., 0.);
pub const DEFOCUS_ANGLE: f64 = 0.02;
pub const VOID: Vector = Vector::new(0., 0., 0.0);

// Raytracing settings
pub const ITERATIONS: u32 = 200;
pub const BOUNCES: u8 = 4;
pub const EPSILON: f64 = 1e-6;

pub fn create_scene() -> Vec<Arc<dyn SceneElement>> {
    let mut elements = read_obj("src/assets/shell.obj", Arc::new(DiffuseMaterial::new(
        Vector::new(0.6, 0.4, 0.05), 0.
    )));

    let mut snail = read_obj("src/assets/snail.obj", Arc::new(DiffuseMaterial::new(
        Vector::new(1., 0.71, 0.), 0.
    )));

    elements.append(&mut snail);

    let mut rocks = read_obj("src/assets/rocks.obj", Arc::new(DiffuseMaterial::new(
        Vector::new(0.62, 0.62, 0.62), 0.
    )));

    elements.append(&mut rocks);

    let mut grass = read_obj("src/assets/grass.obj", Arc::new(DiffuseMaterial::new(
        Vector::new(0., 0.9, 0.06), 0.
    )));

    elements.append(&mut grass);

    // Cornell box
    elements.push(Arc::new(Triangle::new(
        Vector::new(-4., 0., -3.),
        Vector::new(4., 6., -3.),
        Vector::new(-4., 6., -3.),
        Arc::new(DiffuseMaterial::new(Vector::new(1., 1., 1.), 0.))
    )));
    elements.push(Arc::new(Triangle::new(
        Vector::new(4., 6., -3.),
        Vector::new(-4., 0., -3.),
        Vector::new(4., 0., -3.),
        Arc::new(DiffuseMaterial::new(Vector::new(1., 1., 1.), 0.))
    )));
    elements.push(Arc::new(Triangle::new(
        Vector::new(-4., 6., -3.),
        Vector::new(4., 6., -3.),
        Vector::new(-4., 6., 3.),
        Arc::new(DiffuseMaterial::new(Vector::new(1., 1., 1.), 0.))
    )));
    elements.push(Arc::new(Triangle::new(
        Vector::new(4., 6., -3.),
        Vector::new(4., 6., 3.),
        Vector::new(-4., 6., 3.),
        Arc::new(DiffuseMaterial::new(Vector::new(1., 1., 1.), 0.))
    )));
    elements.push(Arc::new(Triangle::new(
        Vector::new(-4., 0.05, -3.),
        Vector::new(-4., 0.05, 3.),
        Vector::new(4., 0.05, -3.),
        Arc::new(DiffuseMaterial::new(Vector::new(1., 1., 1.), 0.))
    )));
    elements.push(Arc::new(Triangle::new(
        Vector::new(4., 0.05, -3.),
        Vector::new(-4., 0.05, 3.),
        Vector::new(4., 0.05, 3.),
        Arc::new(DiffuseMaterial::new(Vector::new(1., 1., 1.), 0.))
    )));
    elements.push(Arc::new(Triangle::new(
        Vector::new(-4., 0., -3.),
        Vector::new(-4., 6., -3.),
        Vector::new(-4., 6., 3.),
        Arc::new(DiffuseMaterial::new(Vector::new(1., 1., 1.), 0.))
    )));
    elements.push(Arc::new(Triangle::new(
        Vector::new(-4., 6., 3.),
        Vector::new(-4., 0., 3.),
        Vector::new(-4., 0., -3.),
        Arc::new(DiffuseMaterial::new(Vector::new(1., 1., 1.), 0.))
    )));
    elements.push(Arc::new(Triangle::new(
        Vector::new(4., 0., -3.),
        Vector::new(4., 6., 3.),
        Vector::new(4., 6., -3.),
        Arc::new(DiffuseMaterial::new(Vector::new(1., 1., 1.), 0.))
    )));
    elements.push(Arc::new(Triangle::new(
        Vector::new(4., 6., 3.),
        Vector::new(4., 0., -3.),
        Vector::new(4., 0., 3.),
        Arc::new(DiffuseMaterial::new(Vector::new(1., 1., 1.), 0.))
    )));
    elements.push(Arc::new(Triangle::new(
        Vector::new(-4., 0., 3.),
        Vector::new(-4., 6., 3.),
        Vector::new(4., 6., 3.),
        Arc::new(DiffuseMaterial::new(Vector::new(1., 1., 1.), 0.))
    )));
    elements.push(Arc::new(Triangle::new(
        Vector::new(4., 6., 3.),
        Vector::new(4., 0., 3.),
        Vector::new(-4., 0., 3.),
        Arc::new(DiffuseMaterial::new(Vector::new(1., 1., 1.), 0.))
    )));

    // Top light
    elements.push(Arc::new(Triangle::new(
        Vector::new(-2., 5.99, -2.),
        Vector::new(2., 5.99, -2.),
        Vector::new(-2., 5.99, 2.),
        Arc::new(DiffuseMaterial::new(Vector::new(1., 1., 1.), 2.))
    )));
    elements.push(Arc::new(Triangle::new(
        Vector::new(2., 5.99, -2.),
        Vector::new(2., 5.99, 2.),
        Vector::new(-2., 5.99, 2.),
        Arc::new(DiffuseMaterial::new(Vector::new(1., 1., 1.), 2.))
    )));
    
    elements
}
