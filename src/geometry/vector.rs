use std::ops::{Neg, Add, Sub, Mul, Div, AddAssign};
use std::fmt;
use rand::random;

#[derive(Debug, Clone, Copy)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector {
    pub const fn new(x: f32, y: f32, z: f32) -> Vector {
        Vector { x, y, z }
    }

    pub fn normalize(&self) -> Vector {
        let magnitude = (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
        Vector {
            x: self.x / magnitude,
            y: self.y / magnitude,
            z: self.z / magnitude
        }
    }

    pub const fn dot(&self, other: &Vector) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub const fn cross(&self, other: &Vector) -> Vector {
        Vector {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x
        }
    }

    pub const fn hadamard(&self, other: &Vector) -> Vector {
        Vector {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z
        }
    }

    // Returns a random unit vector
    // https://math.stackexchange.com/questions/44689/how-to-find-a-random-axis-or-unit-vector-in-3d
    pub fn random() -> Vector {
        let theta = random::<f32>() * std::f32::consts::PI * 2.;
        let z = random::<f32>() * 2. - 1.;
        let x = (1. - z * z).sqrt() * theta.cos();
        let y = (1. - z * z).sqrt() * theta.sin();
        
        Vector {x, y, z}
    }

    pub const ZERO: Vector = Vector { x: 0., y: 0., z: 0. };
}

impl Neg for Vector {
    type Output = Vector;

    fn neg(self) -> Vector {
        Vector {
            x: -self.x,
            y: -self.y,
            z: -self.z
        }
    }
}

impl Add<Vector> for Vector {
    type Output = Vector;

    fn add(self, other: Vector) -> Vector {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
}

impl Add<Vector> for &Vector {
    type Output = Vector;
    fn add(self, other: Vector) -> Vector {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Add<&Vector> for Vector {
    type Output = Vector;
    fn add(self, other: &Vector) -> Vector {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Add<&Vector> for &Vector {
    type Output = Vector;
    fn add(self, other: &Vector) -> Vector {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl AddAssign<Vector> for Vector {
    fn add_assign(&mut self, other: Vector) {
        *self = *self + other;
    }
}

impl AddAssign<&Vector> for Vector {
    fn add_assign(&mut self, other: &Vector) {
        *self = *self + other;
    }
}

impl Sub<Vector> for Vector {
    type Output = Vector;

    fn sub(self, other: Vector) -> Vector {
        self + (-other)
    }
}

impl Sub<Vector> for &Vector {
    type Output = Vector;
    fn sub(self, other: Vector) -> Vector {
        Vector {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

// Vector - &Vector  
impl Sub<&Vector> for Vector {
    type Output = Vector;
    fn sub(self, other: &Vector) -> Vector {
        Vector {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

// &Vector - &Vector
impl Sub<&Vector> for &Vector {
    type Output = Vector;
    fn sub(self, other: &Vector) -> Vector {
        Vector {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul<f32> for Vector {
    type Output = Vector;

    fn mul(self, scalar: f32) -> Vector {
        Vector {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar
        }
    }
}

impl Mul<Vector> for f32 {
    type Output = Vector;

    fn mul(self, vector: Vector) -> Vector {
        Vector {
            x: vector.x * self,
            y: vector.y * self,
            z: vector.z * self
        }
    }
}

impl Mul<f32> for &Vector {
    type Output = Vector;
    fn mul(self, scalar: f32) -> Vector {
        Vector {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}
 
impl Mul<&Vector> for f32 {
    type Output = Vector;
    fn mul(self, vector: &Vector) -> Vector {
        Vector {
            x: vector.x * self,
            y: vector.y * self,
            z: vector.z * self,
        }
    }
}

impl Div<f32> for Vector {
    type Output = Vector;

    fn div(self, scalar: f32) -> Vector {
        Vector {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar
        }
    }
}

impl Div<Vector> for f32 {
    type Output = Vector;

    fn div(self, vector: Vector) -> Vector {
        Vector {
            x: vector.x / self,
            y: vector.y / self,
            z: vector.z / self
        }
    }
}

impl Div<f32> for &Vector {
    type Output = Vector;
    fn div(self, scalar: f32) -> Vector {
        Vector {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
        }
    }
}
 
impl Div<&Vector> for f32 {
    type Output = Vector;
    fn div(self, vector: &Vector) -> Vector {
        Vector {
            x: vector.x / self,
            y: vector.y / self,
            z: vector.z / self,
        }
    }
}

impl fmt::Display for Vector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}
