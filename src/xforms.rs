use json::JsonValue;

use crate::multivector::Multivector;

/// Any transformation from Cl(3) -> Cl(3) (3D Clifford Algebra)
pub trait Transform {
    /// Transform a point into another point in the same space.
    fn transform(&self, point: &Multivector) -> Multivector;
}

pub struct TransformChain {
    transforms: Vec<Box<dyn Transform>>,
}

impl TransformChain {
    pub fn new(transforms: Vec<Box<dyn Transform>>) -> Self {
        Self {
            transforms
        }
    }
}

impl Transform for TransformChain {
    fn transform(&self, point: &Multivector) -> Multivector {
        let mut result = point.clone();
        for xform in self.transforms {
            result = xform.transform(&result);
        }

        result
    }
}

/// A fancy composition of sandwich product, scalar product and addition of
/// multivectors
///
/// f(v) = s a v a^(-1) + d
///
/// where s is a scalar,
/// a is a unit blade or rotor*
/// d is a vector
///
///
/// *note: For now, I only plan to use the sandwich part with the following:
/// - unit vectors: n, |n| = 1
/// - unit bivectors: B, |B| = 1
/// - unit trivectors: T, |T| = 1
/// - rotors: (cos(theta/2) + sin(theta/2)B), |B| = 1
/// - dual rotors: (cos(theta/2)T + sin(theta/2)n), |T| = 1, |n| = 1
///
/// Does this work for other multivector sandwich products? Probably. As long
/// as ava^(-1) produces a vector, that should be fine. When does this happen?
/// I don't currently know.
pub struct TranslatedSandwich {
    scalar: Multivector,
    sandwich: Multivector,
    translation: Multivector,
}

impl TranslatedSandwitch {
    pub fn new(
            scalar: Multivector, sandwich: Multivector, translation: Multivector) 
            -> Self {
        Self {
            scalar,
            sandwich,
            translation
        }
    }

    pub fn identity() -> Self {
        Self { 
            scalar: Multivector::one(), 
            sandwich: Multivector::one(), 
            translation: Multivector::zero(), 
        }
    }

    pub fn translation(d: Multivector) -> Self {
        Self {
            scalar: Multivector::one(),
            sandwich: Multivector::one(),
            translation: d
        }
    }

    pub fn rotation(axis: Multivector, angle: f64) -> Self {
        let unit_bivector = axis.dual().normalize();
        let half_angle = angle / 2.0;

        let cos_part = Multivector::scalar(half_angle.cos());
        let sin_part = unit_bivector.scale(half_angle.sin());
        let rotor = cos_part.add(sin_part);

        Self {
            scalar: Multivector::one(),
            sandwich: rotor,
            translation: Multivector::zero()
        }
    }

    pub fn scale(s: f64) -> Self {
        Self {
            scalar: Multivector::scalar(s),
            sandwich: Multivector::one(),
            translation: Multivector:zero(),
        }
    }

    /// Compose two translated sandwiches to reduce chain lengths.
    /// f2(f1(v)) = s2 a2(s1 a1 v a1^(-1) + d1)a2^(-1) + d2
    ///           = (s1s2) (a2a1) v (a2a1)^(-1) + (s2 a2 (d1) a2^(-1) + d2)
    ///           = (s1s2) (a2a1) v (a2a1)^(-1) + f2(d1)
    pub fn compose(&self, other: &Self) -> Self {
        let scalar = self.scalar.mul(other.scalar);
        let sandwich = self.sandwich.mul(other.sandwich);
        let translation = self.transform(other.translation);

        Self {
            scalar,
            sandwich,
            translation
        }
    }
}

impl Transform for TranslatedSandwitch {
    /// Compute f(v) = s a v a^(-1) + d
    fn transform(&self, point: &Multivector) -> Multivector {
        let sandwiched = self.sandwich.sandwich_product(point);
        let scaled = self.scalar.mul(sandwitched);
        let translated = self.scaled.add(self.translation);

        translated
    }
}

pub struct Inverse {}

impl Inverse {
    pub fn new() -> Self {
        Self {}
    }
}

impl Transform for Inverse {
    fn transform(&self, point: &Multivector) -> Multivector {
        point.inverse()
    }
}

/*
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
*/

/*
/// Parse a transformation from JSON of the form
///
/// ```text
/// {
///     "type": "chain" | "sandwich" | "inverse",
///     ...params
/// }
/// ```
pub fn from_json(xform_desc: &JsonValue) -> Box<dyn Transform> {
    let xform_type = xform_desc["type"]
        .as_str()
        .expect("type must be a string!");

    match &xform_type[..] {
        "chain" => Box::new(TransformChain::from_json(&xform_desc)),
        "trs" => Box::new(TRS::from_json(&xform_desc)),
        _ => panic!("xform type must be 'trs' for now")
    }
}
*/
