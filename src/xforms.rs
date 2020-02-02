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

    pub fn from_json(chain_desc: &JsonValue) -> Self {
        let xforms_json = chain_desc[1];
        
        let mut transforms: Vec<Box<dyn Transform>> = Vec::new();
        let mut current_sandwich = TranslatedSandwich::identity();
        let mut sandwich_active = false;
        for xform_json in xforms_json.members() {
            let xform_type = xform_json[0]
                .as_str()
                .expect("transformation type must be a string");

            match xform_type {
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
                    let old_sandwich = current_sandwich;
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

impl TranslatedSandwich {
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
        let rotor = cos_part.add(&sin_part);

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

    pub fn from_json(xform_desc: &JsonValue) -> Self {
        let xform_type = xform_desc[0]
            .as_str()
            .expect("transformation type must be a string.");
        
        let parameters: Vec<f64> = match xform_desc {
            JsonValue::Array(components) => 
                components[1..].iter().map(|x| {
                    x.as_f64()
                    .expect("transformation parameters must be floats")
                }).collect(),
            _ => Vec::new()
        };

        let valid_names: Vec<&str> = vec![
            "translate",
            "rotate",
            "scale",
            "invert",
            "reflect_vec",
            "reflect_thru_vec",
        ];

        match xform_type {
            "translate" => {
                let [x, y, z] = &parameters[..];
                let displacement = Multivector::vector(*x, *y, *z);
                Self::translation(displacement)
            }
            "rotate" => {
                let [nx, ny, nz, theta] = &parameters[..];
                let axis = Multivector::vector(*nx, *ny, *nz);
                Self::rotation(axis, *theta)
            }
            "scale" => {
                let [k] = &parameters[..];
                Self::scale(*k)
            }
            "reflect_vec" => {
                let [nx, ny, nz] = &parameters[..];
                let direction = Multivector::vector(*nx, *ny, *nz).normalize();
                let negate = Multivector::scalar(-1.0);
                let no_translation = Multivector::zero();
                Self::new(negate, direction, no_translation)
            }
            "reflect_thru_vec" => {
                let [nx, ny, nz] = &parameters[..];
                let direction = Multivector::vector(*nx, *ny, *nz).normalize();
                let stay_positive = Multivector::one();
                let no_translation = Multivector::zero();
                Self::new(stay_positive, direction, no_translation)
            }
            _ => panic!("transformation type must be one of {:?}", valid_names)
        }
    }
}

impl Transform for TranslatedSandwich {
    /// Compute f(v) = s a v a^(-1) + d
    fn transform(&self, point: &Multivector) -> Multivector {
        let sandwiched = self.sandwich.sandwich_product(&point);
        let scaled = self.scalar.mul(&sandwiched);
        let translated = scaled.add(&self.translation);

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

/// Parse a transformation from JSON of the form
///
/// ```text
/// [type, params...]
/// ```
pub fn from_json(xform_desc: &JsonValue) -> Box<dyn Transform> {
    let xform_type = xform_desc[0]
        .as_str()
        .expect("transformation type must be a string");

    let valid_names: Vec<&str> = vec![
        "chain",
        "invert",
        "translate",
        "rotate",
        "scale",
        "reflect_vec",
        "reflect_thru_vec",
    ];

    match &xform_type[..] {
        "chain" => Box::new(TransformChain::from_json(&xform_desc)),
        "invert" => Box::new(Inverse::new()),
        "translate" | 
        "rotate" | 
        "scale" | 
        "reflect_vec" | 
        "reflect_thru_vec" => 
            Box::new(TranslatedSandwich::from_json(&xform_desc)),
        _ => panic!("xform type must be one of {:?} for now", valid_names)
    }
}
