use std::fmt::{Display, Debug, Formatter, Result};
use std::ops::{Add, Mul, Sub};

use json::JsonValue;
use json::JsonValue::Array;
use rand::Rng;

/// A 3-component vector of any type
#[derive(Copy, Clone)]
pub struct Vector3<T> {
    components: [T; 3]
}

/// The most common vector of float values. I'm using single-precision
/// only to be compatible with Cesium 3D tiles. This may change in the future.
pub type Vec3 = Vector3<f32>;
/// Color values are often represneted as values in the range [0, 255],
/// which fit in a u8
pub type Color = Vector3<u8>;

impl<T> Vector3<T> {
    pub fn new(x: T, y: T, z: T) -> Vector3<T> {
        Vector3 { components: [x, y, z] }
    }

    /// x-component
    pub fn x(&self) -> &T {
        &self.components[0]
    }

    /// y-component
    pub fn y(&self) -> &T {
        &self.components[1]
    }

    /// z-component
    pub fn z(&self) -> &T {
        &self.components[2]
    }
}

/// Debug format: (x, y, z)
impl<T: Display> Debug for Vector3<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "({}, {}, {})", self.x(), self.y(), self.z())
    }
}

/// Display format is a vector in 3D-Clifford Algebra Cl(3):
/// `xe1 + ye2 + ze3`
impl<T: Display> Display for Vector3<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}e1 + {}e2 + {}e3", self.x(), self.y(), self.z())
    }
}

impl Vec3 {
    /// Length of vector, `|v| = sqrt(x^2 + y^2 + z^2)`
    pub fn length(&self) -> f32 {
        let x_sqr = self.x() * self.x();
        let y_sqr = self.y() * self.y();
        let z_sqr = self.z() * self.z();
        
        (x_sqr + y_sqr + z_sqr).sqrt()
    }

    /// Make the vector unit length.
    /// `normalize(v) = v / length(v)`
    /*
    pub fn normalize(&self) -> Vec3 {
        let len = self.length();
        let x = self.x() / len;
        let y = self.y() / len;
        let z = self.z() / len;

        Vec3::new(x, y, z)
    }
    */

    /// Scale the vector.
    /// `S_k v = k * v
    pub fn scale(&self, scalar: f32) -> Vec3 {
        Vec3 { 
            components: [
                scalar * self.x(),
                scalar * self.y(),
                scalar * self.z(),
            ] 
        }
    }

    /// Convert vectors in the range [0.0, 1.0] to Color instances
    /// of the range [0, 255]
    pub fn to_color(&self) -> Color {
        Color {
            components: [
                (self.x() * 255.0) as u8,
                (self.y() * 255.0) as u8,
                (self.z() * 255.0) as u8
            ]
        }
    }

    /// Pack the vector into an array of bytes in little-endian format
    /// ```text
    /// bytes  0-3: x
    ///        4-7: y
    ///       8-11: z
    /// ```
    pub fn pack(&self) -> [u8; 12] {
        let mut results = [0; 12];
        results[..4].copy_from_slice(&self.x().to_bits().to_le_bytes());
        results[4..8].copy_from_slice(&self.y().to_bits().to_le_bytes());
        results[8..].copy_from_slice(&self.z().to_bits().to_le_bytes());

        results
    }

    /// Create a random vec3 in the range [-1.0, 1.0]^3
    pub fn random() -> Vec3 { 
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(-1.0, 1.0);
        let y = rng.gen_range(-1.0, 1.0);
        let z = rng.gen_range(-1.0, 1.0);

        Vec3::new(x, y, z)
    }

    /// Create a random bright color in the range [0.5, 1.0]^3
    pub fn random_color() -> Vec3 { 
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(0.5, 1.0);
        let y = rng.gen_range(0.5, 1.0);
        let z = rng.gen_range(0.5, 1.0);

        Vec3::new(x, y, z)
    }

    /// Create the zero vector, (0, 0, 0)
    pub fn zero() -> Vec3 {
        Vec3::new(0.0, 0.0, 0.0)
    }
    
    /// Create the one vector, (1, 1, 1)
    pub fn ones() -> Vec3 {
        Vec3::new(1.0, 1.0, 1.0)
    }

    /// Parse a Vec3 from JSON of the form:
    ///
    /// ```text
    /// [x],
    ///
    /// OR
    ///
    /// [x, y, z]
    /// ```
    ///
    /// A default value can be specified if this key is not provided.
    pub fn from_json(json: &JsonValue, default_val: Vec3) -> Vec3 {
        match json {
            Array(components) => Vec3::parse_components(components),
            _ => default_val
        }
    }

    /// Parse an array of 
    fn parse_components(components: &Vec<JsonValue>) -> Vec3 {
        let components_float: Vec<f32> = components.into_iter().map(|x| {
            x.as_f32().expect("vector components must be floats")
        }).collect();

        match components_float.as_slice() {
            [x] => Vec3::new(*x, *x, *x),
            [x, y, z] => Vec3::new(*x, *y, *z),
            _ => panic!("vectors must have 1 or 3 components")
        }
    }
}


/// Add Vec3 values component-wise
impl Add for Vec3 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            components: [
                self.x() + other.x(),
                self.y() + other.y(),
                self.z() + other.z(),
            ]
        }
    }
}

/// Subtract Vec3 values component-wise
impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            components: [
                self.x() - other.x(),
                self.y() - other.y(),
                self.z() - other.z(),
            ]
        }
    }
}


/// Multiply Vec3 values *component-wise*
impl Mul for Vec3 {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Self {
            components: [
                self.x() * other.x(),
                self.y() * other.y(),
                self.z() * other.z()
            ]
        }
    }
}

impl Color {
    /// Since colors are already u8s, we can pack them as `[x, y, z]` in
    /// 3 bytes
    pub fn pack(&self) -> [u8; 3] {
        self.components.clone()
    }
}

