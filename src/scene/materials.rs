use crate::geometry::vector::Vector;
use crate::geometry::ray::Ray;

pub trait Material: Send + Sync {
    fn albedo(&self) -> Vector;
    fn emission(&self) -> Vector;
    fn reflect(&self, ray: &Ray, point: &Vector, normal: &Vector) -> Ray;
}

pub struct DiffuseMaterial {
    pub albedo: Vector,
    pub intensity: f64
}

impl DiffuseMaterial {
    pub fn new(albedo: Vector, intensity: f64) -> Self {
        Self { albedo, intensity }
    }
}

impl Material for DiffuseMaterial {
    fn albedo(&self) -> Vector {
        return self.albedo;
    }

    fn emission(&self) -> Vector {
        self.albedo * self.intensity
    }

    fn reflect(&self, _: &Ray, point: &Vector, normal: &Vector) -> Ray {
        // Diffuse materials use Lambertian distribution
        let mut direction = normal + Vector::random();
        // Prevent the unlikely event that the result of the above operation is 0
        if direction.is_zero() { direction = normal.clone(); }

        Ray::new(point.clone(), direction)
    }
}

pub struct ReflectiveMaterial {
    pub albedo: Vector,
    pub glossiness: f64,
    pub intensity: f64
}

impl ReflectiveMaterial {
    pub fn new(albedo: Vector, intensity: f64, glossiness: f64) -> Self {
        Self { albedo, intensity, glossiness }
    }
}

impl Material for ReflectiveMaterial {
    fn albedo(&self) -> Vector {
        return self.albedo;
    }
    
    fn emission(&self) -> Vector {
        self.albedo * self.intensity
    }

    fn reflect(&self, ray: &Ray, point: &Vector, normal: &Vector) -> Ray {
        let mut direction = ray.direction - 2. * ray.direction.dot(&normal) * normal;
        direction += (1. - self.glossiness) * Vector::random();

        // Prevent the unlikely event that the result of the above operation is 0
        if direction.is_zero() { direction = normal.clone(); }

        Ray::new(point.clone(), direction)
    }
}
