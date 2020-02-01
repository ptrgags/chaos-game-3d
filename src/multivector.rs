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

    pub fn one() -> Self {
        Self::scalar(1.0)
    }

    pub fn zero() -> Self {
        Self::scalar(0.0)
    }

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

    pub fn trivector(t: f64) -> Self {
        Self {
            components: [
                0.0,
                0.0, 0.0, 0.0,
                0.0, 0.0, 0.0,
                t
            ],
            start_index: BIVECTOR_OFFSET,
            end_index: TRIVECTOR_OFFSET
        }
    }

    pub fn add(&self, other: &Self) -> Self {
        let start = self.start_index.min(other.start_index);
        let end = self.end_index.max(other.end_index);
        let mut components = [0.0; 8];
        for i in start..end {
            components[i] = self.components[i] + other.components[i];
        }

        Self::new(components, start, end)
    }

    pub fn mul(&self, other: &Self) -> Self {
        let mut result = [0.0; 8];
        let mut start = END_OFFSET;
        let mut end = SCALAR_OFFSET;
        for i in self.start_index..self.end_index {
            let a = self.components[i];
            for j in other.start_index..self.end_index {
                let b = other.components[j];
                let index = MULT_COMPONENTS[i][j];
                let sign = MULT_SIGNS[i][j];
                result[index] = sign * a * b;

                start = start.min(index);
                end = end.max(index);
            }
        }

        Self::new(result, start, end)
    }
    
    pub fn reverse(&self, other: &Self) -> Self {
        let mut result = [0.0; 8];
        for i in self.start_index..self.end_index {
            result[i] = REVERSE_SIGNS[i] * self.components[i];
        }

        Self::new(result, self.start_index, self.end_index);
    }
}
