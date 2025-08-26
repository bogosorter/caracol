use std::rc::Rc;
use crate::geometry::vector::Vector;
use crate::scene::materials::{DiffuseMaterial, ReflectiveMaterial};
use crate::scene::elements::{SceneElement, Triangle};
use crate::utils::reader::read_obj;

// Image settings
pub const ASPECT_RATIO: f64 = 1./1.;
pub const WIDTH: u32 = 512;
pub const HEIGHT: u32 = (WIDTH as f64 / ASPECT_RATIO) as u32;

// Camera settings
pub const FOV_ANGLE: f64 = std::f64::consts::FRAC_PI_3;
pub const CAMERA_POSITION: Vector = Vector::new(0., 0., 9.);
pub const LOOK_AT: Vector = Vector::new(0., 0., 0.);
pub const VIEW_UP: Vector = Vector::new(0., 1., 0.);
pub const DEFOCUS_ANGLE: f64 = 0.;
pub const VOID: Vector = Vector::new(0., 0., 0.0);

// Raytracing settings
pub const ITERATIONS: u32 = 100;
pub const BOUNCES: u8 = 4;
pub const EPSILON: f64 = 1e-6;

pub fn create_scene() -> Vec<Rc<dyn SceneElement>> {
    let mut elements = read_obj("src/assets/taxi.obj", Rc::new(ReflectiveMaterial::new(
        Vector::new(1.0, 1.0, 0.8), 0., 0.8
    )));

    // Cornell box
    elements.push(Rc::new(Triangle::new(
        Vector::new(-3., -3., -3.),
        Vector::new(-3., 3., -3.),
        Vector::new(3., 3., -3.),
        Rc::new(DiffuseMaterial::new(Vector::new(1., 1., 1.), 0.))
    )));
    elements.push(Rc::new(Triangle::new(
        Vector::new(3., 3., -3.),
        Vector::new(-3., -3., -3.),
        Vector::new(3., -3., -3.),
        Rc::new(DiffuseMaterial::new(Vector::new(1., 1., 1.), 0.))
    )));
    elements.push(Rc::new(Triangle::new(
        Vector::new(-3., 3., -3.),
        Vector::new(-3., 3., 3.),
        Vector::new(3., 3., -3.),
        Rc::new(DiffuseMaterial::new(Vector::new(1., 1., 1.), 0.))
    )));
    elements.push(Rc::new(Triangle::new(
        Vector::new(3., 3., -3.),
        Vector::new(-3., 3., 3.),
        Vector::new(3., 3., 3.),
        Rc::new(DiffuseMaterial::new(Vector::new(1., 1., 1.), 0.))
    )));
    elements.push(Rc::new(Triangle::new(
        Vector::new(-3., -3., -3.),
        Vector::new(-3., -3., 3.),
        Vector::new(3., -3., -3.),
        Rc::new(DiffuseMaterial::new(Vector::new(1., 1., 1.), 0.))
    )));
    elements.push(Rc::new(Triangle::new(
        Vector::new(3., -3., -3.),
        Vector::new(-3., -3., 3.),
        Vector::new(3., -3., 3.),
        Rc::new(DiffuseMaterial::new(Vector::new(1., 1., 1.), 0.))
    )));
    elements.push(Rc::new(Triangle::new(
        Vector::new(-3., -3., -3.),
        Vector::new(-3., 3., -3.),
        Vector::new(-3., 3., 3.),
        Rc::new(DiffuseMaterial::new(Vector::new(1., 0., 0.), 0.))
    )));
    elements.push(Rc::new(Triangle::new(
        Vector::new(-3., 3., 3.),
        Vector::new(-3., -3., -3.),
        Vector::new(-3., -3., 3.),
        Rc::new(DiffuseMaterial::new(Vector::new(1., 0., 0.), 0.))
    )));
    elements.push(Rc::new(Triangle::new(
        Vector::new(3., -3., -3.),
        Vector::new(3., 3., -3.),
        Vector::new(3., 3., 3.),
        Rc::new(DiffuseMaterial::new(Vector::new(0., 1., 0.), 0.))
    )));
    elements.push(Rc::new(Triangle::new(
        Vector::new(3., 3., 3.),
        Vector::new(3., -3., -3.),
        Vector::new(3., -3., 3.),
        Rc::new(DiffuseMaterial::new(Vector::new(0., 1., 0.), 0.))
    )));

    // Top light
    elements.push(Rc::new(Triangle::new(
        Vector::new(-2., 2.9, -2.),
        Vector::new(-2., 2.9, 2.),
        Vector::new(2., 2.9, -2.),
        Rc::new(DiffuseMaterial::new(Vector::new(1., 1., 1.), 3.))
    )));
    elements.push(Rc::new(Triangle::new(
        Vector::new(2., 2.9, -2.),
        Vector::new(-2., 2.9, 2.),
        Vector::new(2., 2.9, 2.),
        Rc::new(DiffuseMaterial::new(Vector::new(1., 1., 1.), 3.))
    )));
    
    elements
}
