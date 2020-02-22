use std::fmt::{Debug, Formatter, Result};
use crate::vector::Vec3;

/// A multivector in Clifford Algebra Cl(3)
///
/// m = s + xe1 + ye2 + ze2 + xye12 + xze13 + yze23 + te123
///   = s +        v        +          B            +  T
///    (scalar + vector + bivector + trivector)
#[derive(Clone)]
pub struct Multivector {
    components: [f64; 8],
    // Often only a portion of the array is used, so mark the bounds
    start_index: usize,
    end_index: usize,
}

/// The first element is the scalar part
const SCALAR_OFFSET: usize = 0;
/// The next 3 elements are the vector part
const VECTOR_OFFSET: usize = 1;
/// The next 3 elements are the bivector part
const BIVECTOR_OFFSET: usize = 4;
/// The last element is the trivector part
const TRIVECTOR_OFFSET: usize = 7;
/// The end fo the array
const END_OFFSET: usize = 8;

/// Multivector multiplication gets complicated since multiplication
/// can either increase or reduce the grade of the multiplicands. This
/// is a lookup table of just the component indices of the results.
/// See https://bivector.net/tools.html for the Cayley table I refered to
/// when making this table. Note that I'm using the 3D Vectorspace Geometric
/// Algebra and not the projective one
const MULT_COMPONENTS: [[usize; 8]; 8] = [
    [0, 1, 2, 3, 4, 5, 6, 7],
    [1, 0, 4, 5, 2, 3, 7, 6],
    [2, 4, 0, 6, 1, 7, 3, 5],
    [3, 5, 6, 0, 7, 1, 2, 4],
    [4, 2, 1, 7, 0, 6, 5, 3],
    [5, 3, 7, 1, 6, 0, 4, 2],
    [6, 7, 3, 2, 5, 4, 0, 1],
    [7, 6, 5, 4, 3, 2, 1, 0],
];

/// I pulled out the signs separately for convenience. This handles cases
/// where the basis vector squared results in -1 like e12 * e12 = -1
/// Again, this is based on the Cayley table from 
/// https://bivector.net/tools.html
const MULT_SIGNS: [[f64; 8]; 8] = [
    [1.0,  1.0,  1.0, 1.0,  1.0,  1.0,  1.0,  1.0],
    [1.0,  1.0,  1.0, 1.0,  1.0,  1.0,  1.0,  1.0],
    [1.0, -1.0,  1.0, 1.0, -1.0, -1.0,  1.0, -1.0],
    [1.0, -1.0, -1.0, 1.0,  1.0, -1.0, -1.0,  1.0],
    [1.0, -1.0,  1.0, 1.0, -1.0, -1.0,  1.0, -1.0],
    [1.0, -1.0, -1.0, 1.0,  1.0, -1.0, -1.0,  1.0],
    [1.0,  1.0, -1.0, 1.0, -1.0,  1.0, -1.0, -1.0],
    [1.0,  1.0, -1.0, 1.0, -1.0,  1.0, -1.0, -1.0],
];

/// When taking the reverse of a multivector, flip the signs of the bivector
/// and trivector part
const REVERSE_SIGNS: [f64; 8] = [1.0, 1.0, 1.0, 1.0, -1.0, -1.0, -1.0, -1.0];

impl Multivector {
    pub fn new(
            components: [f64; 8], start_index: usize, end_index: usize) 
            -> Self {
        Self {
            components,
            start_index,
            end_index
        }
    }

    /// Get the scalar 1
    pub fn one() -> Self {
        Self::scalar(1.0)
    }

    /// Get the scalar 0
    pub fn zero() -> Self {
        Self::scalar(0.0)
    }

    /// Get a scalar s
    pub fn scalar(s: f64) -> Self {
        Self {
            components: [
                s,
                0.0, 0.0, 0.0,
                0.0, 0.0, 0.0,
                0.0
            ],
            start_index: SCALAR_OFFSET,
            end_index: VECTOR_OFFSET
        }
    }

    /// Get a vector xe1 + ye2 + ze3
    pub fn vector(x: f64, y: f64, z: f64) -> Self {
        Self {
            components: [
                0.0,
                x, y, z,
                0.0, 0.0, 0.0,
                0.0
            ],
            start_index: VECTOR_OFFSET,
            end_index: BIVECTOR_OFFSET
        }
    }

    /// Get a vector xye12 + xze13 + yze23
    pub fn bivector(xy: f64, xz: f64, yz: f64) -> Self {
        Self {
            components: [
                0.0,
                0.0, 0.0, 0.0,
                xy, xz, yz,
                0.0
            ],
            start_index: BIVECTOR_OFFSET,
            end_index: TRIVECTOR_OFFSET
        }
    }

    /// Get a trivector te123
    pub fn trivector(t: f64) -> Self {
        Self {
            components: [
                0.0,
                0.0, 0.0, 0.0,
                0.0, 0.0, 0.0,
                t
            ],
            start_index: TRIVECTOR_OFFSET,
            end_index: END_OFFSET
        }
    }

    /// Create a vector from a Vec3
    pub fn from_vec3(vector: &Vec3) -> Self {
        Self::vector(*vector.x() as f64, *vector.y() as f64, *vector.z() as f64)
    }

    /// Convert the vector part to a Vec3, ignoring other components
    pub fn to_vec3(&self) -> Vec3 {
        let x = self.components[VECTOR_OFFSET];
        let y = self.components[VECTOR_OFFSET + 1];
        let z = self.components[VECTOR_OFFSET + 2];
        Vec3::new(x as f32, y as f32, z as f32)
    }

    /// Add two multivectors together, which is a componentwise sum
    pub fn add(&self, other: &Self) -> Self {
        let start = self.start_index.min(other.start_index);
        let end = self.end_index.max(other.end_index);
        let mut components = [0.0; 8];
        for i in start..end {
            components[i] = self.components[i] + other.components[i];
        }

        Self::new(components, start, end)
    }

    /// Multiply two multivectors together, following the multiplication
    /// rules for Cl(3). See https://bivector.net/tools.html for the Cayley 
    /// table.
    pub fn mul(&self, other: &Self) -> Self {
        let mut result = [0.0; 8];
        let mut start = END_OFFSET;
        let mut end = SCALAR_OFFSET;
        for i in self.start_index..self.end_index {
            let a = self.components[i];
            for j in other.start_index..other.end_index {
                let b = other.components[j];
                let index = MULT_COMPONENTS[i][j];
                let sign = MULT_SIGNS[i][j];
                result[index] += sign * a * b;

                start = start.min(index);
                end = end.max(index);
            }
        }

        Self::new(result, start, end + 1)
    }

    /// Scale a multivector component-wise. This isn't a usual operation,
    /// but it's useful to emulate non-uniform scaling of vectors which is
    /// the only place I intend to use it.
    ///
    /// This only scales components between the other's start and end, as
    /// that is the only valid data is providided. However, this returns
    /// a new multivector of the same size as this one. Caller beware :)
    pub fn mul_components(&self, other: &Self) -> Self {
        let mut result = [0.0; 8];

        for i in self.start_index..self.end_index {
            result[i] = self.components[i];
        }

        for i in other.start_index..other.end_index {
            result[i] *= other.components[i];
        }

        Self::new(result, self.start_index, self.end_index)
    }

    /// The product
    ///
    /// ```text
    /// m n m^(-1)
    /// ```
    ///
    /// can be used for rotations and reflections in Cl(3). Technically this
    /// should be called conjugate(other), but that might get confused for
    /// other uses of the word "conjugate" like complex conjugate. Also
    /// sandwich product sounds cooler :)
    pub fn sandwich_product(&self, other: &Self) -> Self {
        let inv = self.inverse();
        self.mul(&other).mul(&inv)
    }

    /// Scale a multivector by multiplying componentwise
    ///
    /// k(s + v + B + T) = ks + kv + kB + kT
    pub fn scale(&self, scale_factor: f64) -> Self {
        let mut result = [0.0; 8];
        for i in self.start_index..self.end_index {
            result[i] = self.components[i] * scale_factor;
        }

        Self::new(result, self.start_index, self.end_index)
    }

    /// Compute the dual of this multivector.
    /// This is done by mulliplying by -e123
    pub fn dual(&self) -> Self {
        let pseudoscalar = Self::trivector(-1.0);

        self.mul(&pseudoscalar)
    }
    
    /// Reverse of the multivector. This is somewhat like conjugation of
    /// complex numbers. The bivector and trivector components are negated
    /// while the scalar and vector parts remain unchanged.
    ///
    /// (s + v + B + T).reverse = s + v - B - T
    pub fn reverse(&self) -> Self {
        let mut result = [0.0; 8];
        for i in self.start_index..self.end_index {
            result[i] = REVERSE_SIGNS[i] * self.components[i];
        }

        Self::new(result, self.start_index, self.end_index)
    }

    /// Inverse of a vector
    ///
    /// v^-1 = v.reverse / (v * v.reverse) = v.reverse / |v|^2
    ///
    /// I only claim that this is well-defined for blades
    pub fn inverse(&self) -> Self {
        let scale_factor = 1.0 / self.norm();
        let rev = self.reverse(); 

        rev.scale(scale_factor)
    }

    /// Normalize a blade so it has unit magnitude
    ///
    /// normalize(v) = v / |v|
    ///
    /// I only claim that this is well-defined for blades
    pub fn normalize(&self) -> Self {
        let length = self.magnitude();

        self.scale(1.0 / length)
    }

    /// norm = length squared
    ///
    /// |v|^2 = (v * v.reverse)[0]   (scalar part of result)
    ///
    /// I only claim that this is well-defined for blades
    pub fn norm(&self) -> f64 {
        let rev = self.reverse();
        let product = self.mul(&rev);

        product.components[SCALAR_OFFSET]
    }

    /// magnitude of a blade
    ///
    /// |v| = sqrt((v * v.reverse)[0])
    ///
    /// I only claim that this is well-defined for blades
    pub fn magnitude(&self) -> f64 {
        let mag_squared = self.norm();

        mag_squared.sqrt()
    }
}

/// Debug format: Multivector[s, x, y, z, xy, xz, yz, t]
impl Debug for Multivector {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "Multivector{:?}({}-{})", 
           self.components, self.start_index, self.end_index)
    }
}
