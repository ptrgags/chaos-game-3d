use std::ops::Mul;

use crate::vector::Vec3;

#[derive(Copy, Clone, Debug)]
pub struct Quaternion {
    components: [f32; 4],
}

impl Quaternion {
    pub fn new(w: f32, x: f32, y: f32, z:f32) -> Self {
        Quaternion { components: [w, x, y, z] }
    }

    pub fn from_axis_angle(angle: f32, axis: Vec3) -> Self {
        let half_angle = angle / 2.0;
        let cos_ha = half_angle.cos();
        let sin_ha = half_angle.sin();
        let scaled_axis = axis.normalize().scale(sin_ha);
        Self { 
            components: [
                cos_ha, 
                *scaled_axis.x(),
                *scaled_axis.y(), 
                *scaled_axis.z()
            ]
        }
    }

    pub fn from_vector(vector: Vec3) -> Self {
        Self {
            components: [
                0.0f32,
                *vector.x(),
                *vector.y(),
                *vector.z(),
            ]
        }
    }

    pub fn identity() -> Self {
        Self {
            components: [1.0, 0.0, 0.0, 0.0]
        }
    }

    pub fn w(&self) -> f32 {
        self.components[0]
    }

    pub fn x(&self) -> f32 {
        self.components[1]
    }

    pub fn y(&self) -> f32 {
        self.components[2]
    }

    pub fn z(&self) -> f32 {
        self.components[3]
    }

    pub fn conj(&self) -> Self {
        Quaternion {
            components: [
                self.w(),
                -self.x(),
                -self.y(),
                -self.z(),
            ]
        }
    }
}

impl Mul for Quaternion {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        let w1 = self.w();
        let x1 = self.x();
        let y1 = self.y();
        let z1 = self.z();

        let w2 = other.w();
        let x2 = other.x();
        let y2 = other.y();
        let z2 = other.z();

        Self {
            components: [
                w1 * w2 - x1 * x2 - y1 * y2 - z1 * z2,
                w1 * x2 + x1 * w2 + y1 * z2 - z1 * y2,
                w1 * y2 - x1 * z2 + y1 * w2 + z1 * x2,
                w1 * z2 + x1 * y2 - y1 * x2 + z1 * w2,
            ]
        }
    }
}

impl Mul<Vec3> for Quaternion {
    type Output = Vec3;
    fn mul(self, other: Vec3) -> Vec3 {
        let other_quat = Self::from_vector(other);
        let conj = self.conj();
        let product = self * other_quat * conj; 

        Vec3::new(product.x(), product.y(), product.z())
    }
}
