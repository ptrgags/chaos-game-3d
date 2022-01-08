use std::cmp::Eq;
use std::fmt::{Debug, Formatter, Result};

use crate::vector::Vec3;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Parity {
    Even,
    Odd
}

fn get_product_parity(left: &Parity, right: &Parity) -> Parity {
    if left == right {
        Parity::Even
    } else {
        Parity::Odd
    }
}

// Even multivectors:
// [s, yzpn, xzpn, xypn, xyzn, xyzp, xy, xz, xp, xn, yz, yp, yn, zp, zn, pn]
const SCALAR: usize = 0;
const YZPN: usize = 1;
const XZPN: usize = 2;
const XYPN: usize = 3;
const XYZN: usize = 4;
const XYZP: usize = 5;
const XY: usize = 6;
const XZ: usize = 7;
const XP: usize = 8;
const XN: usize = 9;
const YZ: usize = 10;
const YP: usize = 11;
const YN: usize = 12;
const ZP: usize = 13;
const ZN: usize = 14;
const PN: usize = 15;

const SCALAR_START: usize = 0;
const SCALAR_END: usize = 1;
#[allow(dead_code)]
const QUADVECTOR_START: usize = 1;
#[allow(dead_code)]
const QUADVECTOR_END: usize = 6;
const BIVECTOR_START: usize = 6;
const BIVECTOR_END: usize = 16;


// Odd multivectors:
// [xyzpn, x, y, z, p, n, zpn, ypn, yzn, yzp, xpn, xzn, xzp, xyn, xyp, xyz]
const XYZPN: usize = 0;
const X: usize = 1;
const Y: usize = 2;
const Z: usize = 3;
const P: usize = 4;
const N: usize = 5;
const ZPN: usize = 6;
const YPN: usize = 7;
const YZN: usize = 8;
const YZP: usize = 9;
const XPN: usize = 10;
const XZN: usize = 11;
const XZP: usize = 12;
const XYN: usize = 13;
const XYP: usize = 14;
const XYZ: usize = 15;

#[allow(dead_code)]
const PSEUDOSCALAR_START: usize = 0;
#[allow(dead_code)]
const PSEUDOSCALAR_END: usize = 1;
const VECTOR_START: usize = 1;
#[allow(dead_code)]
const VECTOR_END: usize = 6;
#[allow(dead_code)]
const TRIVECTOR_START: usize = 6;
#[allow(dead_code)]
const TRIVECTOR_END: usize = 16;

type ComponentLUT = [[usize; 16]; 16];

const COMPONENTS_EVEN_EVEN: ComponentLUT = [
    [SCALAR, YZPN, XZPN, XYPN, XYZN, XYZP, XY, XZ, XP, XN, YZ, YP, YN, ZP, ZN, PN],
    [YZPN, SCALAR, XY, XZ, XP, XN, XZPN, XYPN, XYZN, XYZP, PN, ZN, ZP, YN, YP, YZ],
    [XZPN, XY, SCALAR, YZ, YP, YN, YZPN, PN, ZN, ZP, XYPN, XYZN, XYZP, XN, XP, XZ],
    [XYPN, XZ, YZ, SCALAR, ZP, ZN, PN, YZPN, YN, YP, XZPN, XN, XP, XYZN, XYZP, XY],
    [XYZN, XP, YP, ZP, SCALAR, PN, ZN, YN, YZPN, YZ, XN, XZPN, XZ, XYPN, XY, XYZP],
    [XYZP, XN, YN, ZN, PN, SCALAR, ZP, YP, YZ, YZPN, XP, XZ, XZPN, XY, XYPN, XYZN],
    [XY, XZPN, YZPN, PN, ZN, ZP, SCALAR, YZ, YP, YN, XZ, XP, XN, XYZP, XYZN, XYPN],
    [XZ, XYPN, PN, YZPN, YN, YP, YZ, SCALAR, ZP, ZN, XY, XYZP, XYZN, XP, XN, XZPN],
    [XP, XYZN, ZN, YN, YZPN, YZ, YP, ZP, SCALAR, PN, XYZP, XY, XYPN, XZ, XZPN, XN],
    [XN, XYZP, ZP, YP, YZ, YZPN, YN, ZN, PN, SCALAR, XYZN, XYPN, XY, XZPN, XZ, XP],
    [YZ, PN, XYPN, XZPN, XN, XP, XZ, XY, XYZP, XYZN, SCALAR, ZP, ZN, YP, YN, YZPN],
    [YP, ZN, XYZN, XN, XZPN, XZ, XP, XYZP, XY, XYPN, ZP, SCALAR, PN, YZ, YZPN, YN],
    [YN, ZP, XYZP, XP, XZ, XZPN, XN, XYZN, XYPN, XY, ZN, PN, SCALAR, YZPN, YZ, YP],
    [ZP, YN, XN, XYZN, XYPN, XY, XYZP, XP, XZ, XZPN, YP, YZ, YZPN, SCALAR, PN, ZN],
    [ZN, YP, XP, XYZP, XY, XYPN, XYZN, XN, XZPN, XZ, YN, YZPN, YZ, PN, SCALAR, ZP],
    [PN, YZ, XZ, XY, XYZP, XYZN, XYPN, XZPN, XN, XP, YZPN, YN, YP, ZN, ZP, SCALAR],
];

const COMPONENTS_ODD_ODD: ComponentLUT = COMPONENTS_EVEN_EVEN;

const COMPONENTS_EVEN_ODD: ComponentLUT = [
    [XYZPN, X, Y, Z, P, N, ZPN, YPN, YZN, YZP, XPN, XZN, XZP, XYN, XYP, XYZ],
    [X, XYZPN, ZPN, YPN, YZN, YZP, Y, Z, P, N, XYZ, XYP, XYN, XZP, XZN, XPN],
    [Y, ZPN, XYZPN, XPN, XZN, XZP, X, XYZ, XYP, XYN, Z, P, N, YZP, YZN, YPN],
    [Z, YPN, XPN, XYZPN, XYN, XYP, XYZ, X, XZP, XZN, Y, YZP, YZN, P, N, ZPN],
    [P, YZN, XZN, XYN, XYZPN, XYZ, XYP, XZP, X, XPN, YZP, Y, YPN, Z, ZPN, N],
    [N, YZP, XZP, XYP, XYZ, XYZPN, XYN, XZN, XPN, X, YZN, YPN, Y, ZPN, Z, P],
    [ZPN, Y, X, XYZ, XYP, XYN, XYZPN, XPN, XZN, XZP, YPN, YZN, YZP, N, P, Z],
    [YPN, Z, XYZ, X, XZP, XZN, XPN, XYZPN, XYN, XYP, ZPN, N, P, YZN, YZP, Y],
    [YZN, P, XYP, XZP, X, XPN, XZN, XYN, XYZPN, XYZ, N, ZPN, Z, YPN, Y, YZP],
    [YZP, N, XYN, XZN, XPN, X, XZP, XYP, XYZ, XYZPN, P, Z, ZPN, Y, YPN, YZN],
    [XPN, XYZ, Z, Y, YZP, YZN, YPN, ZPN, N, P, XYZPN, XYN, XYP, XZN, XZP, X],
    [XZN, XYP, P, YZP, Y, YPN, YZN, N, ZPN, Z, XYN, XYZPN, XYZ, XPN, X, XZP],
    [XZP, XYN, N, YZN, YPN, Y, YZP, P, Z, ZPN, XYP, XYZ, XYZPN, X, XPN, XZN],
    [XYN, XZP, YZP, P, Z, ZPN, N, YZN, YPN, Y, XZN, XPN, X, XYZPN, XYZ, XYP],
    [XYP, XZN, YZN, N, ZPN, Z, P, YZP, Y, YPN, XZP, X, XPN, XYZ, XYZPN, XYN],
    [XYZ, XPN, YPN, ZPN, N, P, 1, Y, YZP, YZN, X, XZP, XZN, XYP, XYN, XYZPN],
];

const COMPONENTS_ODD_EVEN: ComponentLUT = COMPONENTS_EVEN_ODD;

fn get_component_table(left: &Parity, right: &Parity) -> ComponentLUT {
    match (left, right) {
        (Parity::Even, Parity::Even) => COMPONENTS_EVEN_EVEN,
        (Parity::Even, Parity::Odd) => COMPONENTS_EVEN_ODD,
        (Parity::Odd, Parity::Even) => COMPONENTS_ODD_EVEN,
        (Parity::Odd, Parity::Odd) => COMPONENTS_ODD_ODD,
    }
}

type SignLUT = [[i8; 16]; 16];

const SIGNS_EVEN_EVEN: SignLUT = [
    [1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1, 1],
    [1, -1,  1, -1,  1,  1, -1,  1, -1, -1, -1,  1,  1, -1, -1, 1],
    [1, -1, -1,  1, -1, -1,  1, -1,  1,  1, -1,  1,  1, -1, -1, 1],
    [1,  1, -1, -1,  1,  1, -1, -1,  1,  1,  1, -1, -1, -1, -1, 1],
    [1, -1,  1, -1, -1, -1, -1,  1,  1,  1, -1, -1, -1,  1,  1, 1],
    [1, -1,  1, -1,  1,  1, -1,  1, -1, -1, -1,  1,  1, -1, -1, 1],
    [1,  1, -1, -1, -1, -1, -1, -1, -1, -1,  1,  1,  1,  1,  1, 1],
    [1, -1, -1,  1,  1,  1,  1, -1, -1, -1, -1, -1, -1,  1,  1, 1],
    [1,  1,  1,  1, -1, -1,  1,  1, -1, -1,  1, -1, -1, -1, -1, 1],
    [1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1, 1],
    [1, -1,  1, -1, -1, -1, -1,  1,  1,  1, -1, -1, -1,  1,  1, 1],
    [1,  1, -1, -1,  1,  1, -1, -1,  1,  1,  1, -1, -1, -1, -1, 1],
    [1,  1, -1, -1, -1, -1, -1, -1, -1, -1,  1,  1,  1,  1,  1, 1],
    [1, -1, -1,  1, -1, -1,  1, -1,  1,  1, -1,  1,  1, -1, -1, 1],
    [1, -1, -1,  1,  1,  1,  1, -1, -1, -1, -1, -1, -1,  1,  1, 1],
    [1,  1,  1,  1, -1, -1,  1,  1, -1, -1,  1, -1, -1, -1, -1, 1],
];

const SIGNS_ODD_ODD: SignLUT = [
    [-1,  1, -1,  1, -1, -1,  1, -1,  1,  1,  1, -1, -1,  1,  1, -1],
    [ 1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1],
    [-1, -1,  1,  1,  1,  1,  1,  1,  1,  1, -1, -1, -1, -1, -1, -1],
    [ 1, -1, -1,  1,  1,  1,  1, -1, -1, -1, -1, -1, -1,  1,  1,  1],
    [-1, -1, -1, -1,  1,  1, -1, -1,  1,  1, -1,  1,  1,  1,  1, -1],
    [-1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
    [ 1, -1, -1,  1, -1, -1,  1, -1,  1,  1, -1,  1,  1, -1, -1,  1],
    [-1, -1,  1,  1, -1, -1,  1,  1, -1, -1, -1,  1,  1,  1,  1, -1],
    [ 1, -1,  1, -1, -1, -1, -1,  1,  1,  1, -1, -1, -1,  1,  1,  1],
    [ 1, -1,  1, -1,  1,  1, -1,  1, -1, -1, -1,  1,  1, -1, -1,  1],
    [ 1,  1,  1,  1, -1, -1,  1,  1, -1, -1,  1, -1, -1, -1, -1,  1],
    [-1,  1,  1, -1, -1, -1, -1,  1,  1,  1,  1,  1,  1, -1, -1, -1],
    [-1,  1,  1, -1,  1,  1, -1,  1, -1, -1,  1, -1, -1,  1,  1, -1],
    [ 1,  1, -1, -1, -1, -1, -1, -1, -1, -1,  1,  1,  1,  1,  1,  1],
    [ 1,  1, -1, -1,  1,  1, -1, -1,  1,  1,  1, -1, -1, -1, -1,  1],
    [-1,  1, -1,  1,  1,  1,  1, -1, -1, -1,  1,  1,  1, -1, -1, -1],
];

const SIGNS_EVEN_ODD: SignLUT = [
    [ 1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1],
    [-1,  1, -1,  1, -1, -1,  1, -1,  1,  1,  1, -1, -1,  1,  1, -1],
    [ 1, -1, -1,  1, -1, -1,  1, -1,  1,  1, -1,  1,  1, -1, -1,  1],
    [-1, -1,  1,  1, -1, -1,  1,  1, -1, -1, -1,  1,  1,  1,  1, -1],
    [ 1, -1,  1, -1, -1, -1, -1,  1,  1,  1, -1, -1, -1,  1,  1,  1],
    [ 1, -1,  1, -1,  1,  1, -1,  1, -1, -1, -1,  1,  1, -1, -1,  1],
    [-1, -1,  1,  1,  1,  1,  1,  1,  1,  1, -1, -1, -1, -1, -1, -1],
    [ 1, -1, -1,  1,  1,  1,  1, -1, -1, -1, -1, -1, -1,  1,  1,  1],
    [-1, -1, -1, -1,  1,  1, -1, -1,  1,  1, -1,  1,  1,  1,  1, -1],
    [-1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
    [-1,  1, -1,  1,  1,  1,  1, -1, -1, -1,  1,  1,  1, -1, -1, -1],
    [ 1,  1, -1, -1,  1,  1, -1, -1,  1,  1,  1, -1, -1, -1, -1,  1],
    [ 1,  1, -1, -1, -1, -1, -1, -1, -1, -1,  1,  1,  1,  1,  1,  1],
    [-1,  1,  1, -1,  1,  1, -1,  1, -1, -1,  1, -1, -1,  1,  1, -1],
    [-1,  1,  1, -1, -1, -1, -1,  1,  1,  1,  1,  1,  1, -1, -1, -1],
    [ 1,  1,  1,  1, -1, -1,  1,  1, -1, -1,  1, -1, -1, -1, -1,  1],
];

const SIGNS_ODD_EVEN: SignLUT = [
    [1, -1,  1, -1,  1,  1, -1,  1, -1, -1, -1,  1,  1, -1, -1, 1],
    [1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1, 1],
    [1,  1, -1, -1, -1, -1, -1, -1, -1, -1,  1,  1,  1,  1,  1, 1],
    [1, -1, -1,  1,  1,  1,  1, -1, -1, -1, -1, -1, -1,  1,  1, 1],
    [1,  1,  1,  1, -1, -1,  1,  1, -1, -1,  1, -1, -1, -1, -1, 1],
    [1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1, 1],
    [1, -1, -1,  1, -1, -1,  1, -1,  1,  1, -1,  1,  1, -1, -1, 1],
    [1,  1, -1, -1,  1,  1, -1, -1,  1,  1,  1, -1, -1, -1, -1, 1],
    [1, -1,  1, -1, -1, -1, -1,  1,  1,  1, -1, -1, -1,  1,  1, 1],
    [1, -1,  1, -1,  1,  1, -1,  1, -1, -1, -1,  1,  1, -1, -1, 1],
    [1,  1,  1,  1, -1, -1,  1,  1, -1, -1,  1, -1, -1, -1, -1, 1],
    [1, -1, -1,  1,  1,  1,  1, -1, -1, -1, -1, -1, -1,  1,  1, 1],
    [1, -1, -1,  1, -1, -1,  1, -1,  1,  1, -1,  1,  1, -1, -1, 1],
    [1,  1, -1, -1, -1, -1, -1, -1, -1, -1,  1,  1,  1,  1,  1, 1],
    [1,  1, -1, -1,  1,  1, -1, -1,  1,  1,  1, -1, -1, -1, -1, 1],
    [1, -1,  1, -1, -1, -1, -1,  1,  1,  1, -1, -1, -1,  1,  1, 1],
];

fn get_sign_table(left: &Parity, right: &Parity) -> SignLUT {
    match (left, right) {
        (Parity::Even, Parity::Even) => SIGNS_EVEN_EVEN,
        (Parity::Even, Parity::Odd) => SIGNS_EVEN_ODD,
        (Parity::Odd, Parity::Even) => SIGNS_ODD_EVEN,
        (Parity::Odd, Parity::Odd) => SIGNS_ODD_ODD,
    }
}

fn find_start(array: &[f64]) -> usize {
    match array.iter().position(|&x| x != 0.0) {
        Some(index) => index,
        None => array.len()
    }
}

fn find_end(array: &[f64]) -> usize {
    match array.iter().rposition(|&x| x != 0.0) {
        Some(index) => index + 1,
        None => 0
    }
}

#[derive(Clone, PartialEq)]
pub struct HalfMultivector {
    components: [f64; 16],
    parity: Parity,
    start_index: usize,
    end_index: usize
}

impl HalfMultivector {
    #[allow(dead_code)]
    pub fn even(
            components: [f64; 16],
            start_index: usize,
            end_index: usize) -> Self {
        Self {
            components,
            parity: Parity::Even,
            start_index,
            end_index
        }
    }

    /// Create an arbitrary odd vector. Used in unit tests
    #[allow(dead_code)]
    pub fn odd(
            components: [f64; 16],
            start_index: usize,
            end_index: usize) -> Self {
        Self {
            components,
            parity: Parity::Odd,
            start_index,
            end_index
        }
    }

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
        components[YZ] = -s * nx;
        components[XZ] = s * ny;
        components[XY] = -s * nz;
        Self {
            components,
            parity: Parity::Even,
            start_index: SCALAR_START,
            end_index: YZ + 1
        }
    }

    pub fn scale(scale_factor: f64) -> Self {
        let half_log_scale = scale_factor.ln() / 2.0;
        let c = half_log_scale.cosh();
        let s = half_log_scale.sinh();
        // cosh(ln(alpha) / 2) + sinh(ln(alpha) / 2) np
        let mut components = [0.0; 16];
        components[SCALAR] = c;
        components[PN] = -s;
        Self {
            components,
            parity: Parity::Even,
            start_index: SCALAR,
            end_index: BIVECTOR_END,
        }
    }

    pub fn translation(x: f64, y: f64, z: f64) -> Self {
        let hx = 0.5 * x;
        let hy = 0.5 * y;
        let hz = 0.5 * z;
        let mut components = [0.0; 16];
        // 1 + infx + infy + infz
        // = 1 + (n + p)x + (n + p)y + (n + p)z
        // = 1 - xn - xp - yn - yp - zn - zp
        components[SCALAR] = 1.0;
        components[XP] = -hx;
        components[XN] = -hx;
        components[YP] = -hy;
        components[YN] = -hy;
        components[ZP] = -hz;
        components[ZN] = -hz;
        
        Self {
            components,
            parity: Parity::Even,
            start_index: SCALAR_START,
            end_index: ZN + 1
        }
    }

    pub fn reflection(nx: f64, ny: f64, nz: f64) -> Self {
        let mut components = [0.0; 16];
        components[X] = nx;
        components[Y] = ny;
        components[Z] = nz;
        Self {
            components,
            parity: Parity::Odd,
            start_index: VECTOR_START,
            end_index: Z + 1
        }
    }

    pub fn inversion() -> Self {
        let mut components = [0.0; 16];
        components[P] = 1.0;
        Self {
            components,
            parity: Parity::Odd,
            start_index: P,
            end_index: P + 1
        }
    }

    pub fn point(x: f64, y: f64, z: f64) -> Self {
        let mag_sqr = x * x + y * y + z * z;
        
        // x + 1/2x^2 inf + origin
        // x + 1/2x^2 (n + p) + 1/2(n - p)
        // x + 1/2(x^2 + 1)n + 1/2(x^2 - 1)p
        let n = 0.5 * (mag_sqr + 1.0);
        let p = 0.5 * (mag_sqr - 1.0);

        let mut components = [0.0; 16];
        components[X] = x;
        components[Y] = y;
        components[Z] = z;
        components[P] = p;
        components[N] = n;
        Self {
            components,
            parity: Parity::Odd,
            start_index: find_start(&components),
            end_index: find_end(&components)
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

    pub fn geometric_product(&self, other: &Self) -> Self {
        let sign_table = get_sign_table(&self.parity, &other.parity);
        let component_table = get_component_table(&self.parity, &other.parity);
        let parity = get_product_parity(&self.parity, &other.parity);
        let mut result = [0.0; 16];
        for i in self.start_index..self.end_index {
            let a = self.components[i];
            for j in other.start_index..other.end_index {
                let b = other.components[j];
                let index = component_table[i][j];
                let sign = sign_table[i][j] as f64;
                /*
                // helpful for debugging
                if a != 0.0 && b != 0.0 {
                    println!("result[{}] = {}*{}*{} = {}, [{}][{}]", index, sign, a, b, sign * a * b, i, j);
                }
                */
                result[index] += sign * a * b;
            }
        }

        let start = find_start(&result);
        let end = find_end(&result);

        Self {
            components: result,
            parity,
            start_index: start,
            end_index: end
        }
    }

    pub fn tighten(&mut self) {
        self.start_index = find_start(&self.components);
        self.end_index = find_end(&self.components);
    }

    pub fn sandwich_product(&self, other: &Self) -> Self {
        let reverse = self.reverse();
        let mut result = self.geometric_product(&other).geometric_product(&reverse);
        result.tighten();
        result
    }

    pub fn from_vec3(position: &Vec3) -> Self {
        Self::point(
            *position.x() as f64,
            *position.y() as f64,
            *position.z() as f64
        )
    }

    // for points only
    pub fn to_vec3(&self) -> Vec3 {
        if self.parity != Parity::Odd {
            panic!("not a vector!");
        }

        let x = self.components[X];
        let y = self.components[Y];
        let z = self.components[Z];
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

/// Debug format: (Odd|Even)[component0, component1, ..., component15](start-end)
impl Debug for HalfMultivector {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{:?}{:?}({}-{})", 
            self.parity, self.components, self.start_index, self.end_index)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identity() {
        let expected = HalfMultivector::even(
            [
                1.0, 
                0.0, 0.0, 0.0, 0.0, 0.0, 
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            ],
            0,
            1
        );

        assert_eq!(HalfMultivector::identity(), expected);
    }

    #[test]
    fn test_translation() {
        let hx = 1.0;
        let hy = 2.0;
        let hz = 3.0;
        let expected = HalfMultivector::even(
            [
                1.0, 
                0.0, 0.0, 0.0, 0.0, 0.0, 
                0.0, 0.0, -hx, -hx, 0.0, -hy, -hy, -hz, -hz, 0.0,
            ],
            0,
            15
        );
        assert_eq!(HalfMultivector::translation(2.0, 4.0, 6.0), expected);
    }

    #[test]
    fn test_product_even_even() {
        let left = HalfMultivector::even(
            [
                1.0, 
                0.0, 0.0, 0.0, 0.0, 1.0, 
                0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            ],
            0,
            11
        );
        let right = HalfMultivector::even(
            [
                2.0,
                0.0, 0.0, 2.0, 0.0, 0.0,
                1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            ],
            0,
            7
        );
        let expected = HalfMultivector::even(
            [
                2.0, 
                0.0, -2.0, 2.0, 0.0, 2.0, 
                1.0, -1.0, 0.0, 0.0, 2.0, 0.0, 0.0, -1.0, -2.0, 0.0,
            ],
            0,
            15
        );
        assert_eq!(left.geometric_product(&right), expected);
    }

    #[test]
    fn test_point() {
        let expected = HalfMultivector::odd(
            [
                0.0,
                1.0, 2.0, 4.0, 10.0, 11.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            ],
            1,
            6
        );
        assert_eq!(HalfMultivector::point(1.0, 2.0, 4.0), expected);
    }

    #[test]
    fn test_reverse() {
        let original = HalfMultivector::odd(
            [
                1.0,
                2.0, 3.0, 4.0, 5.0, 6.0,
                7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0,
            ],
            0,
            16
        );
        let expected = HalfMultivector::odd(
            [
                1.0,
                2.0, 3.0, 4.0, 5.0, 6.0,
                -7.0, -8.0, -9.0, -10.0, -11.0, -12.0, -13.0, -14.0, -15.0, -16.0,
            ],
            0,
            16
        );
        assert_eq!(original.reverse(), expected);
    }

    #[test]
    fn test_identity_xform() {
        let xform = HalfMultivector::identity();
        let point = HalfMultivector::point(1.0, 2.0, 3.0);
        assert_eq!(xform.sandwich_product(&point), point);
    }

    #[test]
    fn test_translation_xform() {
        let xform = HalfMultivector::translation(1.0, 2.0, 3.0);
        let point = HalfMultivector::point(1.0, 1.0, 1.0);
        let expected = HalfMultivector::point(2.0, 3.0, 4.0);
        assert_eq!(xform.sandwich_product(&point), expected);
    }
}