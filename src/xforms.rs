use std::fmt::Debug;

use crate::vector::Vec3;
use crate::quaternion::Quaternion;

pub trait Transform<T>: Debug {
    fn transform(&self, vector: &Vec3) -> Vec3;
}

// Translate, Rotate, Scale
#[derive(Debug)]
pub struct TRS {
    translate: Vec3,
    rotate: Quaternion,
    scale: Vec3,
}

impl TRS {
    pub fn new(translate: Vec3, rotate: Quaternion, scale: Vec3) -> TRS {
        TRS {
            translate,
            rotate,
            scale
        }
    }
}

impl Transform<f32> for TRS {
    fn transform(&self, vector: &Vec3) -> Vec3 {
        let scaled = self.scale * *vector;
        let rotated = self.rotate * scaled;
        let translated = self.translate + rotated;

        translated
    }
}
