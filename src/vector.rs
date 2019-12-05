use std::fmt::{Display, Debug, Formatter, Result};
use std::ops::{Add, Mul};

use json::JsonValue;
use json::JsonValue::Array;
use rand::Rng;

#[derive(Copy, Clone)]
pub struct Vector3<T> {
    components: [T; 3]
}

pub type Vec3 = Vector3<f32>;
pub type Color = Vector3<u8>;

impl<T> Vector3<T> {
    pub fn new(x: T, y: T, z: T) -> Vector3<T> {
        Vector3 { components: [x, y, z] }
    }

    pub fn x(&self) -> &T {
        &self.components[0]
    }

    pub fn y(&self) -> &T {
        &self.components[1]
    }

    pub fn z(&self) -> &T {
        &self.components[2]
    }
}

impl<T: Display> Debug for Vector3<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "({}, {}, {})", self.x(), self.y(), self.z())
    }
}

impl<T: Display> Display for Vector3<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}i + {}j + {}k", self.x(), self.y(), self.z())
    }
}

impl Vec3 {
    pub fn length(&self) -> f32 {
        let x_sqr = self.x() * self.x();
        let y_sqr = self.y() * self.y();
        let z_sqr = self.z() * self.z();
        
        (x_sqr + y_sqr + z_sqr).sqrt()
    }

    pub fn normalize(&self) -> Vec3 {
        let len = self.length();
        let x = self.x() / len;
        let y = self.y() / len;
        let z = self.z() / len;

        Vec3::new(x, y, z)
    }

    pub fn scale(&self, scalar: f32) -> Vec3 {
        Vec3 { 
            components: [
                scalar * self.x(),
                scalar * self.y(),
                scalar * self.z(),
            ] 
        }
    }

    pub fn to_color(&self) -> Color {
        Color {
            components: [
                (self.x() * 255.0) as u8,
                (self.y() * 255.0) as u8,
                (self.z() * 255.0) as u8
            ]
        }
    }

    pub fn pack(&self) -> [u8; 12] {
        let mut results = [0; 12];
        results[..4].copy_from_slice(&self.x().to_bits().to_le_bytes());
        results[4..8].copy_from_slice(&self.y().to_bits().to_le_bytes());
        results[8..].copy_from_slice(&self.z().to_bits().to_le_bytes());

        results
    }

    pub fn random() -> Vec3 { 
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(-1.0, 1.0);
        let y = rng.gen_range(-1.0, 1.0);
        let z = rng.gen_range(-1.0, 1.0);

        Vec3::new(x, y, z)
    }

    pub fn random_color() -> Vec3 { 
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(0.5, 1.0);
        let y = rng.gen_range(0.5, 1.0);
        let z = rng.gen_range(0.5, 1.0);

        Vec3::new(x, y, z)
    }
}


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
    pub fn pack(&self) -> [u8; 3] {
        self.components.clone()
    }
}

pub fn from_json(json: &JsonValue, default_val: f32) -> Vec3 {
    match json {
        Array(components) => parse_components(components),
        _ => Vec3::new(default_val, default_val, default_val)
    }
}

fn parse_components(components: &Vec<JsonValue>) -> Vec3 {
    let components_float: Vec<f32> = components.into_iter().map(|x| {
        x.as_f32().expect("invalid vector component")
    }).collect();

    match components_float.as_slice() {
        [x] => Vec3::new(*x, *x, *x),
        [x, y, z] => Vec3::new(*x, *y, *z),
        _ => panic!("vectors must have 1 or 3 components")
    }
}
