use std::f64::consts::PI;

use json::JsonValue;

use crate::multivector::Multivector;

/// Any transformation from Cl(3) -> Cl(3) (3D Clifford Algebra)
pub trait Transform {
    /// Transform a point into another point in the same space.
    fn transform(&self, point: &Multivector) -> Multivector;

    /// Compute the inverse of this transformation if it is well-defined
    /// or None if not possible.
    fn inverse(&self) -> Option<Box<dyn Transform>>;
}

/// Chain of transformations applied in the order specified.
/// for example, if the transformations were specified
/// ```text
/// ["chain", [
///     ["scale", k],
///     ["rotate", nx, ny, nz, theta],
///     ["translate", x, y, z]
/// ]]
/// ```
///
/// This would produce the transformation TRS(v) (scaling applied first)
pub struct TransformChain {
    transforms: Vec<Box<dyn Transform>>,
}

impl TransformChain {
    pub fn new(transforms: Vec<Box<dyn Transform>>) -> Self {
        Self {
            transforms
        }
    }

    /// Create a transformation chain from JSON of the form:
    /// ```text
    /// ["chain", [<xform JSON 1>, <xform JSON 2>, ...]]
    /// ```
    ///
    /// Note that any adjacent TranslatedSandwich transformations are
    /// composed together to reduce the total number of transformations applied
    /// at each iteration.
    pub fn from_json(chain_desc: &JsonValue) -> Self {
        let xforms_json = &chain_desc[1];
        
        let mut transforms: Vec<Box<dyn Transform>> = Vec::new();
        let mut current_sandwich = TranslatedSandwich::identity();
        let mut sandwich_active = false;
        for xform_json in xforms_json.members() {
            let xform_type = xform_json[0]
                .as_str()
                .expect("TransformChain: transformation type must be a string");

            match xform_type {
                "identity" |
                "translate" |
                "rotate" |
                "scale" |
                "reflect_vec" |
                "reflect_thru_vec" =>  {
                    // Try to compose sandwich products to reduce computation
                    // steps when iterating
                    let sandwich = TranslatedSandwich::from_json(&xform_json);
                    current_sandwich = sandwich.compose(&current_sandwich);
                    sandwich_active = true;
                },
                _ => {
                    // We've encountered a non-sandwich transform, so
                    // add the old sandwich to the list of transformations
                    // and start a new transform with identity
                    transforms.push(Box::new(current_sandwich));
                    current_sandwich = TranslatedSandwich::identity();
                    sandwich_active = false;

                    // Now construct the current transform and add it
                    // to the list
                    let boxed_xform = from_json(&xform_json);
                    transforms.push(boxed_xform);
                },
            }
        }

        if sandwich_active {
            transforms.push(Box::new(current_sandwich));
        }

        Self::new(transforms)
    }

    to_box!(Transform);
}

impl Transform for TransformChain {
    fn transform(&self, point: &Multivector) -> Multivector {
        let mut result = point.clone();
        for xform in self.transforms.iter() {
            result = xform.transform(&result);
        }

        result
    }

    /// Taking the inverse of a transformation chain is the same as inverting
    /// each transform and reversing the order:
    ///
    /// ```text
    /// (f1f2f3...fn)^(-1) = fn^(-1)...f3^(-1)f2^(-1)f1^(-1)
    /// ```
    fn inverse(&self) -> Option<Box<dyn Transform>> {
        let xforms = self.transforms.iter().rev().map(|xform| {
            match xform.inverse() {
                Some(inv) => inv,
                None => panic!(concat!(
                    "Transformation chain is not invertible because it ",
                    "contains a non-invertible transformation"))
            }
        }).collect();

        let chain = Self::new(xforms);
        let boxed = Box::new(chain);
        Some(boxed)
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

impl TranslatedSandwich {
    /// General sandwich transformation
    ///
    /// s n v n^(-1) + d
    ///
    /// where: 
    /// s is a scalar (for scaling coordinates)
    /// n is a multivector, one of:
    /// - a unit length vector (for reflections)
    /// - a rotor (for rotations)
    /// - a trivector (for space inversions)
    /// d is a vector representing a vector (for translation)
    pub fn new(
            scalar: Multivector,
            sandwich: Multivector, 
            translation: Multivector) 
            -> Self {
        Self {
            scalar,
            sandwich,
            translation
        }
    }

    /// Identity. Stay in place
    /// The map I(v) = v = 1 1 v 1^(-1) + 0
    pub fn identity() -> Self {
        Self { 
            scalar: Multivector::one(), 
            sandwich: Multivector::one(), 
            translation: Multivector::zero(), 
        }
    }

    /// Translation: shift points in space
    /// The map T(v) = v + d = 1 1 v 1^(-1) + d
    pub fn translation(d: Multivector) -> Self {
        Self {
            scalar: Multivector::one(),
            sandwich: Multivector::one(),
            translation: d
        }
    }

    /// Rotation
    /// 
    /// Given n = axis of rotation (vector)
    ///       theta = angle to rotate around the axis (scalar)
    ///
    /// Let r = cos(theta/2) + sin(theta/2) * B
    ///     B = n* = n * -e123
    ///
    /// Then R(v) = r v r^(-1) = 1 r v r^(-1) + 0
    pub fn rotation(axis: Multivector, angle: f64) -> Self {
        let unit_bivector = axis.dual().normalize();
        let half_angle = angle / 2.0;

        let cos_part = Multivector::scalar(half_angle.cos());
        let sin_part = unit_bivector.scale(half_angle.sin());
        let rotor = cos_part.add(&sin_part);

        Self {
            scalar: Multivector::one(),
            sandwich: rotor,
            translation: Multivector::zero()
        }
    }

    /// Scale
    /// S(v) = kv = k 1 v 1^(-1) + 0
    pub fn scale(s: f64) -> Self {
        Self {
            scalar: Multivector::scalar(s),
            sandwich: Multivector::one(),
            translation: Multivector::zero(),
        }
    }

    /// Compose two translated sandwiches to reduce chain lengths.
    /// f2(f1(v)) = s2 a2(s1 a1 v a1^(-1) + d1)a2^(-1) + d2
    ///           = (s1s2) (a2a1) v (a2a1)^(-1) + (s2 a2 (d1) a2^(-1) + d2)
    ///           = (s1s2) (a2a1) v (a2a1)^(-1) + f2(d1)
    pub fn compose(&self, other: &Self) -> Self {
        let scalar = self.scalar.mul(&other.scalar);
        let sandwich = self.sandwich.mul(&other.sandwich);
        let translation = self.transform(&other.translation);

        Self {
            scalar,
            sandwich,
            translation
        }
    }

    /// Create a Translated Sandwich from JSON of one of the forms:
    /// ```text
    /// ["identity"]
    /// ["translate", x, y, z]        // translation factor
    /// ["rotate", nx, ny, nz, theta] // axis and angle. Theta is a fraction of 
    ///                               // 2 PI 
    /// ["scale", k]                  // Scale factor. Uniform only for now.
    /// ["invert"]                    // Sphere inversion
    /// ["reflect_vec", nx, ny, nz]   // Reflect in plane normal to this vector.
    ///                               // In other words, flip the projection of
    ///                               // a coordinate onto this normal vector
    /// ["reflect_thru_vec", x, y, z] // Fix the direction specified by this
    ///                               // direction and flip the other two
    ///                               // orthogonal directions. This is like
    ///                               // a 180 degree rotation around the
    ///                               // vector.
    /// ```
    pub fn from_json(xform_desc: &JsonValue) -> Self {
        let xform_type = xform_desc[0]
            .as_str()
            .expect("sandwich: transformation type must be a string.");
        
        let parameters: Vec<f64> = match xform_desc {
            JsonValue::Array(components) => 
                components[1..].iter().map(|x| {
                    x.as_f64()
                    .expect("sandwich:transformation parameters must be floats")
                }).collect(),
            _ => Vec::new()
        };

        let valid_names: Vec<&str> = vec![
            "identity",
            "translate",
            "rotate",
            "scale",
            "invert",
            "reflect_vec",
            "reflect_thru_vec",
        ];

        match xform_type {
            "identity" => {
                Self::identity()
            },
            "translate" => {
                if let [x, y, z] = &parameters[..] {
                    let displacement = Multivector::vector(*x, *y, *z);
                    Self::translation(displacement)
                } else {
                    panic!("should be [\"translate\", x, y, z]")
                }
            },
            "rotate" => {
                if let [nx, ny, nz, theta] = &parameters[..] {
                    let axis = Multivector::vector(*nx, *ny, *nz);
                    let angle = *theta * 2.0 * PI;
                    Self::rotation(axis, angle)
                } else {
                    panic!("should be [\"rotate\", nx, ny, nz, theta]")
                }
            },
            "scale" => {
                if let [k] = &parameters[..] {
                    Self::scale(*k)
                } else {
                    panic!("should be [\"scale\", k]")
                }
            },
            "reflect_vec" => {
                if let [nx, ny, nz] = &parameters[..] {
                    let direction = 
                        Multivector::vector(*nx, *ny, *nz).normalize();
                    let negate = Multivector::scalar(-1.0);
                    let no_translation = Multivector::zero();
                    Self::new(negate, direction, no_translation)
                } else {
                    panic!("should be [\"reflect_vec\", nx, ny, nz]")
                }
            },
            "reflect_thru_vec" => {
                if let [nx, ny, nz] = &parameters[..] {
                    let direction = 
                        Multivector::vector(*nx, *ny, *nz).normalize();
                    let stay_positive = Multivector::one();
                    let no_translation = Multivector::zero();
                    Self::new(stay_positive, direction, no_translation)
                } else {
                    panic!("should be [\"reflect_thru_vec\", nx, ny, nz]")
                }
            },
            _ => panic!("transformation type must be one of {:?}", valid_names)
        }
    }

    to_box!(Transform);
}

impl Transform for TranslatedSandwich {
    /// Compute f(v) = s a v a^(-1) + d
    fn transform(&self, point: &Multivector) -> Multivector {
        let sandwiched = self.sandwich.sandwich_product(&point);
        let scaled = self.scalar.mul(&sandwiched);
        let translated = scaled.add(&self.translation);

        translated
    }

    /// The inverse of a sandwich transform is derived as follows:
    ///
    /// ```text
    ///     v' = s a v a^(-1) + d
    /// v' - d = s a v a^(-1)
    /// s^(-1) a^(-1) (v' - d) a = v
    /// s^(-1) a^(-1) v' a - s^(-1) a^(-1) d a = v
    ///
    /// so
    ///
    /// s' = s^(-1)
    /// a' = a^(-1)
    /// d' = s' a' d a^(-1)
    /// ```
    fn inverse(&self) -> Option<Box<dyn Transform>> {
        let inv_scalar = self.scalar.inverse();
        let inv_sandwich = self.sandwich.inverse();
        let inv_translation = inv_scalar
            .mul(&inv_sandwich)
            .mul(&self.translation)
            .mul(&self.sandwich);

        let xform = Self::new(inv_scalar, inv_sandwich, inv_translation);
        let boxed = Box::new(xform);
        Some(boxed)
    }
}

/// Sphere inversion in 3D
/// V(v) = v^(-1) = v / |v|^2
pub struct Inverse {}

impl Inverse {
    pub fn new() -> Self {
        Self {}
    }

    to_box!(Transform);
}

impl Transform for Inverse {
    fn transform(&self, point: &Multivector) -> Multivector {
        point.inverse()
    }

    /// Inverse transformations are self-inverses!
    fn inverse(&self) -> Option<Box<dyn Transform>> {
        let inv = Self::new();
        let boxed = Box::new(inv);
        Some(boxed)
    }
}

/// non-uniform scaling can't be expressed as a sandwich product, so
/// represent this separately
pub struct NonUniformScale {
    factors: Multivector
}

impl NonUniformScale {
    pub fn new(factors: Multivector) -> Self {
        Self { factors }
    }

    pub fn from_json(xform_desc: &JsonValue) -> Self {
        let parameters: Vec<f64> = match xform_desc {
            JsonValue::Array(components) => 
                components[1..].iter().map(|x| {
                    x.as_f64()
                    .expect("nonuniform: parameters must be floats")
                }).collect(),
            _ => Vec::new()
        };

        if parameters.len() != 3 {
            panic!(
                "{:?}: nonuniform scale must have 3 components.", 
                parameters);
        }
        
        let factors = Multivector::vector(
            parameters[0], parameters[1], parameters[2]);

        Self {
            factors 
        }
    }

    to_box!(Transform);
}

impl Transform for NonUniformScale {
    fn transform(&self, point: &Multivector) -> Multivector {
        point.mul_components(&self.factors)
    }

    /// For non-uniform scales, the inverse is the reciprocal scale in each
    /// dimension.
    ///
    /// (s_x, s_y, s_z) -> (1/s_x, 1/s_y, 1/s_z);
    fn inverse(&self) -> Option<Box<dyn Transform>> {
        // TODO: I should allow subscripting multivectors. This could be
        // written much more simply then.
        let old_vec = self.factors.to_vec3();
        let sx = (1.0 / *old_vec.x()) as f64;
        let sy = (1.0 / *old_vec.y()) as f64;
        let sz = (1.0 / *old_vec.z()) as f64;
        let new_factors = Multivector::vector(sx, sy, sz);

        let result = Self::new(new_factors);
        Some(result.to_box())
    }
}

/// Parse a transformation from JSON of the form
///
/// ```text
/// [type, params...]
/// ```
///
/// See the other transformation types for more information.
pub fn from_json(xform_desc: &JsonValue) -> Box<dyn Transform> {
    let xform_type = xform_desc[0]
        .as_str()
        .expect("xforms: transformation type must be a string");

    let valid_names: Vec<&str> = vec![
        "chain",
        "invert",
        "scale3",
        "identity",
        "translate",
        "rotate",
        "scale",
        "reflect_vec",
        "reflect_thru_vec",
    ];

    match &xform_type[..] {
        "chain" => TransformChain::from_json(&xform_desc).to_box(),
        "invert" => Inverse::new().to_box(),
        "scale3" => NonUniformScale::from_json(&xform_desc).to_box(),
        "identity" |
        "translate" | 
        "rotate" | 
        "scale" | 
        "reflect_vec" | 
        "reflect_thru_vec" => 
            TranslatedSandwich::from_json(&xform_desc).to_box(),
        _ => panic!("xforms: xform type must be one of {:?}", valid_names)
    }
}
