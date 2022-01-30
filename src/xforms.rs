use std::f64::consts::PI;

use json::JsonValue;

use crate::half_multivector::HalfMultivector;

/// Any transformation from Cl(3) -> Cl(3) (3D Clifford Algebra)
pub trait Transform {
    /// Transform a point into another point in the same space.
    fn transform(&self, point: &HalfMultivector) -> HalfMultivector;

    /// Compute the inverse of this transformation if it is well-defined
    /// or None if not possible.
    fn inverse(&self) -> Option<Box<dyn Transform>>;
}

pub struct Xform {
    versor: HalfMultivector
}

impl Xform {
    pub fn new(versor: HalfMultivector) -> Self {
        Self {
            versor
        }
    }

    pub fn identity() -> Self {
        Self {
            versor: HalfMultivector::identity()
        }
    }

    pub fn followed_by(&self, other: &Self) -> Self {
        Self {
            versor: other.versor.geometric_product(&self.versor)
        }
    }

    pub fn transform(&self, point: &HalfMultivector) -> HalfMultivector {
        let mut product = self.versor.sandwich_product(point);
        // often multiplication produces almost-zero components, but the
        // result will always be a vector since I'm only ever transforming
        // points.
        product.expect_vector();
        // Some transformations introduce a scaling factor, divide it out
        // (much like the w component of homongeneous coordinates in traditional
        // computer graphics)
        product.homogenize();
        product
    }

    pub fn inverse(&self) -> Self {
        Self {
            versor: self.versor.reverse()
        }
    }
}

fn get_versor(versor_desc: &JsonValue) -> HalfMultivector {
    // We've already validated the string at this point
    let xform_type = versor_desc[0].as_str().unwrap();
        
    let parameters: Vec<f64> = match versor_desc {
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
        "poloidal",
        "scale",
        "invert",
        "reflect",
    ];

    match xform_type {
        "identity" => HalfMultivector::identity(),
        "invert" => HalfMultivector::inversion(),
        "translate" => {
            if let [x, y, z] = &parameters[..] {
                HalfMultivector::translation(*x, *y, *z)
            } else {
                panic!("should be [\"translate\", x, y, z]")
            }
        },
        "rotate" => {
            if let [nx, ny, nz, theta_deg] = &parameters[..] {
                let angle = *theta_deg * PI / 180.0;
                HalfMultivector::rotation(*nx, *ny, *nz, angle)
            } else {
                panic!("should be [\"rotate\", axis_x, axis_y, axis_z, theta_deg]")
            }
        },
        "poloidal" => {
            if let [x, y, z, theta_deg] = &parameters[..] {
                let angle = *theta_deg * PI / 180.0;
                HalfMultivector::poloidal(*x, *y, *z, angle)
            } else {
                panic!("should be [\"poloidal\", axis_x, axis_y, axis_z, theta_deg]")
            }
        },
        "scale" => {
            if let [k] = &parameters[..] {
                HalfMultivector::scale(*k)
            } else {
                panic!("should be [\"scale\", scale_factor]")
            }
        },
        "reflect" => {
            if let [nx, ny, nz] = &parameters[..] {
                HalfMultivector::reflection(*nx, *ny, *nz)
            } else {
                panic!("should be [\"reflect_vec\", nx, ny, nz]")
            }
        },
        _ => panic!("transformation type must be one of {:?}", valid_names)
    }
}

fn from_chain(xform_chain: &JsonValue) -> Xform {
    let mut chain = Xform::identity();
    for xform_json in xform_chain[1].members() {
        let xform = from_json(&xform_json);
        chain = chain.followed_by(&xform);
    }
    chain
}

pub fn from_json(xform_desc: &JsonValue) -> Xform {
    let xform_type = xform_desc[0]
        .as_str()
        .expect("xforms: transformation type must be a string");

    let valid_names: Vec<&str> = vec![
        "chain",
        "invert",
        "identity",
        "translate",
        "rotate",
        "poloidal",
        "scale",
        "reflect",
    ];

    match &xform_type[..] {
        "chain" => from_chain(xform_desc),
        "invert" | 
        "identity" | 
        "translate" | 
        "rotate" | 
        "poloidal" |
        "reflect" |
        "scale" => Xform::new(get_versor(xform_desc)),
        _ => panic!("xforms: xform type must be one of {:?}", valid_names)
    }
}