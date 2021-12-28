#[derive(Clone)]
enum VersorParity {
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
pub struct Versor {
    components: [f64; 16],
    parity: VersorParity,
    start_index: usize,
    end_index: usize
}

impl Versor {
    pub fn rotation(axis: [f64; 3], angle: f64) -> Self {
        let [x, y, z] = axis;
        let half_angle = angle / 2.0;
        let c = half_angle.cos();
        let s = half_angle.sin();
        // cos(theta/2) + sin(theta/2)B
        let mut components = [0.0; 16];
        components[SCALAR] = c;
        components[BIVECTOR_YZ] = s * x;
        components[BIVECTOR_ZX] = s * y;
        components[BIVECTOR_YZ] = s * z;
        Self {
            components,
            parity: VersorParity::Even,
            start_index: SCALAR,
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
            parity: VersorParity::Even,
            start_index: SCALAR,
            end_index: BIVECTOR_XN,
        }
    }

    pub fn translation(offset: [f64; 3]) -> Self {
        let [x, y, z] = offset;
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
            parity: VersorParity::Even,
            start_index: SCALAR_START,
            end_index: BIVECTOR_END
        }
    }

    pub fn reflection(normal: [f64; 3]) -> Self {
        let [nx, ny, nz] = normal;
        let mut components = [0.0; 16];
        components[VECTOR_X] = nx;
        components[VECTOR_Y] = ny;
        components[VECTOR_Z] = nz;
        Self {
            components,
            parity: VersorParity::Odd,
            start_index: VECTOR_X,
            end_index: VECTOR_N
        }
    }

    pub fn inversion() -> Self {
        let mut components = [0.0; 16];
        components[VECTOR_P] = 1.0;
        Self {
            components,
            parity: VersorParity::Odd,
            start_index: VECTOR_P,
            end_index: VECTOR_END
        }
    }

    pub fn point(position: [f64; 3]) -> Self {
        let [x, y, z] = position;
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
            parity: VersorParity::Odd,
            start_index: VECTOR_X,
            end_index: VECTOR_END
        }
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
}