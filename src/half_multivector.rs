use crate::vector::Vec3;

#[derive(Clone, PartialEq)]
enum Parity {
    Even,
    Odd
}

// even:
// [s, yz, zx, xy, np, xn, xp, yn, yp, zn, zp, _, _, _, _, _]
const SCALAR: usize = 0;
const BIVECTOR_YZ: usize = 1;
const BIVECTOR_ZX: usize = 2;
const BIVECTOR_XY: usize = 3;
const BIVECTOR_NP: usize = 4;
const BIVECTOR_XN: usize = 5;
const BIVECTOR_XP: usize = 6;
const BIVECTOR_YN: usize = 7;
const BIVECTOR_YP: usize = 8;
const BIVECTOR_ZN: usize = 9;
const BIVECTOR_ZP: usize = 10;
// TODO: what order to list quadvectors?

const SCALAR_START: usize = 0;
const SCALAR_END: usize = 1;
const BIVECTOR_START: usize = 1;
const BIVECTOR_END: usize = 11;
const QUADVECTOR_START: usize = 11;
const QUADVECTOR_END: usize = 16;

// odd terms:
// [xyznp, _, _, _, _, _, _, _, _, _, _, x, y, z, n, p]
const PSEUDOSCALAR: usize = 0;
// TODO: What order to list trivectors?
const VECTOR_X: usize = 11;
const VECTOR_Y: usize = 12;
const VECTOR_Z: usize = 13;
const VECTOR_N: usize = 14;
const VECTOR_P: usize = 15;

const PSEUDOSCALAR_START: usize = 0;
const PSEUDOSCALAR_END: usize = 1;
const TRIVECTOR_START: usize = 1;
const TRIVECTOR_END: usize = 11;
const VECTOR_START: usize = 11;
const VECTOR_END: usize = 16;

#[derive(Clone)]
pub struct HalfMultivector {
    components: [f64; 16],
    parity: Parity,
    start_index: usize,
    end_index: usize
}

impl HalfMultivector {
    pub fn identity() -> Self {
        let mut components = [0.0; 16];
        components[SCALAR] = 1.0;
        Self {
            components,
            parity: Parity::Even,
            start_index: SCALAR_START,
            end_index: SCALAR_END
        }
    }

    pub fn rotation(nx: f64, ny: f64, nz: f64, angle_rad: f64) -> Self {
        let half_angle = angle_rad / 2.0;
        let c = half_angle.cos();
        let s = half_angle.sin();
        // cos(theta/2) + sin(theta/2)B
        let mut components = [0.0; 16];
        components[SCALAR] = c;
        components[BIVECTOR_YZ] = s * nx;
        components[BIVECTOR_ZX] = s * ny;
        components[BIVECTOR_YZ] = s * nz;
        Self {
            components,
            parity: Parity::Even,
            start_index: SCALAR_START,
            end_index: BIVECTOR_NP
        }
    }

    pub fn scale(scale_factor: f64) -> Self {
        let half_log_scale = scale_factor.ln() / 2.0;
        let c = half_log_scale.cosh();
        let s = half_log_scale.sinh();
        // cosh(ln(alpha) / 2) + sinh(ln(alpha) / 2) np
        let mut components = [0.0; 16];
        components[SCALAR] = c;
        components[BIVECTOR_NP] = s;
        Self {
            components,
            parity: Parity::Even,
            start_index: SCALAR,
            end_index: BIVECTOR_XN,
        }
    }

    pub fn translation(x: f64, y: f64, z: f64) -> Self {
        let hx = 0.5 * x;
        let hy = 0.5 * y;
        let hz = 0.5 * z;
        let mut components = [0.0; 16];
        components[SCALAR] = 1.0;
        components[BIVECTOR_XN] = -hx;
        components[BIVECTOR_XP] = -hx;
        components[BIVECTOR_YN] = -hy;
        components[BIVECTOR_YP] = -hy;
        components[BIVECTOR_ZN] = -hz;
        components[BIVECTOR_ZP] = -hz;
        Self {
            components,
            parity: Parity::Even,
            start_index: SCALAR_START,
            end_index: BIVECTOR_END
        }
    }

    pub fn reflection(nx: f64, ny: f64, nz: f64) -> Self {
        let mut components = [0.0; 16];
        components[VECTOR_X] = nx;
        components[VECTOR_Y] = ny;
        components[VECTOR_Z] = nz;
        Self {
            components,
            parity: Parity::Odd,
            start_index: VECTOR_X,
            end_index: VECTOR_N
        }
    }

    pub fn inversion() -> Self {
        let mut components = [0.0; 16];
        components[VECTOR_P] = 1.0;
        Self {
            components,
            parity: Parity::Odd,
            start_index: VECTOR_P,
            end_index: VECTOR_END
        }
    }

    pub fn point(x: f64, y: f64, z: f64) -> Self {
        let mag_sqr = x * x + y * y + z * z;
        
        // x + 1/2x^2 inf + origin
        // x + 1/2x^2 (n + p) + 1/2(n - p)
        // x + 1/2(x^2 + 1) n + 1/2(x^2 - 1)p
        let n = 0.5 * (mag_sqr + 1.0);
        let p = 0.5 * (mag_sqr - 1.0);

        let mut components = [0.0; 16];
        components[VECTOR_X] = x;
        components[VECTOR_Y] = y;
        components[VECTOR_Z] = z;
        components[VECTOR_N] = n;
        components[VECTOR_P] = p;
        Self {
            components,
            parity: Parity::Odd,
            start_index: VECTOR_X,
            end_index: VECTOR_END
        }
    }

    pub fn from_vec3(position: &Vec3) -> Self {
        Self::point(
            *position.x() as f64,
            *position.y() as f64,
            *position.z() as f64
        )
    }

    pub fn reverse(&self) -> Self {
        let mut components = self.components.clone();

        // Bivectors and trivectors are reversed, everything else stays
        // the same. Bivectors are even and trivectors are odd,
        // but they occupy the same 10 elements of the array
        for i in BIVECTOR_START..BIVECTOR_END {
            components[i] *= -1.0;
        }

        Self {
            components,
            parity: self.parity.clone(),
            start_index: self.start_index,
            end_index: self.end_index
        }
    }

    pub fn geometric_product(&self, other: &Self) -> Self {
        todo!();
    }

    pub fn sandwich_product(&self, other: &Self) -> Self {
        todo!();
    }

    // for points only
    pub fn to_vec3(&self) -> Vec3 {
        let x = self.components[VECTOR_X];
        let y = self.components[VECTOR_Y];
        let z = self.components[VECTOR_Z];
        Vec3::new(x as f32, y as f32, z as f32)
    }

    pub fn lerp(a: &Self, b: &Self, t: f64) -> Self {
        if a.parity != b.parity {
            panic!("can only lerp versors of the same parity");
        }

        let start_index = a.start_index.min(b.start_index);
        let end_index = a.end_index.max(b.end_index);

        let mut components = [0.0; 16];
        let p = 1.0 - t;
        let q = t;
        for i in start_index..end_index {
            components[i] = p * a.components[i] + q * b.components[i];
        }

        Self {
            components: components,
            parity: a.parity.clone(),
            start_index,
            end_index
        }
    }
}