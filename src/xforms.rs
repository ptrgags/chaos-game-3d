use std::fmt::{Debug, Formatter, Result};

use json::JsonValue;

use crate::vector;
use crate::vector::{Vector3, Vec3};
use crate::quaternion;
use crate::quaternion::Quaternion;

pub trait Transform<T>: Debug {
    fn transform(&self, vector: &Vector3<T>) -> Vector3<T>;
}

// Translate, Rotate, Scale
pub struct TRS {
    translate: Vec3,
    rotate: Quaternion,
    scale: Vec3,
}

impl TRS {
    pub fn new(translate: Vec3, rotate: Quaternion, scale: Vec3) -> Self {
        Self {
            translate,
            rotate,
            scale
        }
    }

    pub fn from_json(xform_desc: &JsonValue) -> Self {
        let translate = vector::from_json(&xform_desc["translate"], 0.0);
        let rotate = quaternion::from_json(&xform_desc["rotate"]);
        let scale = vector::from_json(&xform_desc["scale"], 1.0);

        Self::new(translate, rotate, scale)
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

impl Debug for TRS {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f, 
            "TRS(T={:?}, R={:?}, S={:?})", 
            self.translate, 
            self.rotate, 
            self.scale)
    }
}

pub fn from_json(xform_desc: &JsonValue) -> Box<dyn Transform<f32>> {
    let xform_type = xform_desc["type"]
        .as_str()
        .expect("type must be a string!");
    match &xform_type[..] {
        "trs" => Box::new(TRS::from_json(&xform_desc)),
        _ => panic!("Invalid xform type!")
    }
}
