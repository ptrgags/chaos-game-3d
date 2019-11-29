use std::ops::{Add, Mul};

#[derive(Copy, Clone, Debug)]
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
