use std::fmt::{Debug, Formatter, Result};

use json::JsonValue;

use crate::vector::Vec3;
use crate::quaternion;
use crate::quaternion::Quaternion;

/// A generic transformation from a 3D space to another 3D space.
pub trait Transform: Debug {
    /// Transform a point into another point in the same space.
    fn transform(&self, vector: &Vec3) -> Vec3;
}

/// Translate, rotate and scale transformation.
///
/// More specifically, the operation TRS(v), so scaling is applied first.
/// This handles most of the linear transformations you could ever want.
/// No shear mappings, but I don't use them much.
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

    /// Parse a linear transformation from JSON of the form
    ///
    /// ```text
    /// {
    ///     "type": "trs",
    ///     "translate": [dx, dy, dz],
    ///     "rotate": <Quaternion JSON>,
    ///     "scale": [sx, sy, sz]
    /// }
    /// ```
    /// When not specified, these transformations default to the identity.
    pub fn from_json(xform_desc: &JsonValue) -> Self {
        let translate = Vec3::from_json(&xform_desc["translate"], Vec3::zero());
        let rotate = quaternion::from_json(&xform_desc["rotate"]);
        let scale = Vec3::from_json(&xform_desc["scale"], Vec3::ones());

        Self::new(translate, rotate, scale)
    }

    to_box!(Transform);
}

impl Transform for TRS {
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

pub struct Inverse {}

impl Inverse {
    pub fn new() -> Self {
        Self {}
    }

    to_box!(Transform);
}

impl Transform for Inverse {
    fn transform(&self, vector: &Vec3) -> Vec3 {
        vector.inverse()
    }
}

impl Debug for Inverse {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "v^-1")
    }
}

/// Parse a transformation from JSON of the form
///
/// ```text
/// {
///     "type": "trs",
///     ...params
/// }
/// ```
pub fn from_json(xform_desc: &JsonValue) -> Box<dyn Transform> {
    let xform_type = xform_desc["type"]
        .as_str()
        .expect("type must be a string!");
    match &xform_type[..] {
        "trs" => TRS::from_json(&xform_desc).to_box(),
        "inverse" => Inverse::new().to_box(),
        _ => panic!("xform type must be 'trs' for now")
    }
}
