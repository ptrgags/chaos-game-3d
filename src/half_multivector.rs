use crate::vector::Vec3;

#[derive(Clone, PartialEq)]
enum Parity {
    Even,
    Odd
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
const QUADVECTOR_START: usize = 1;
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

const PSEUDOSCALAR_START: usize = 0;
const PSEUDOSCALAR_END: usize = 1;
const VECTOR_START: usize = 1;
const VECTOR_END: usize = 6;
const TRIVECTOR_START: usize = 6;
const TRIVECTOR_END: usize = 16;

const COMPONENTS_EVEN_EVEN: [[usize; 16]; 16] = [
    [1, YZPN, XZPN, XYPN, XYZN, XYZP, XY, XZ, XP, XN, YZ, YP, YN, ZP, ZN, PN],
    [YZPN, 1, XY, XZ, XP, XN, XZPN, XYPN, XYZN, XYZP, PN, ZN, ZP, YN, YP, YZ],
    [XZPN, XY, 1, YZ, YP, YN, YZPN, PN, ZN, ZP, XYPN, XYZN, XYZP, XN, XP, XZ],
    [XYPN, XZ, YZ, 1, ZP, ZN, PN, YZPN, YN, YP, XZPN, XN, XP, XYZN, XYZP, XY],
    [XYZN, XP, YP, ZP, 1, PN, ZN, YN, YZPN, YZ, XN, XZPN, XZ, XYPN, XY, XYZP],
    [XYZP, XN, YN, ZN, PN, 1, ZP, YP, YZ, YZPN, XP, XZ, XZPN, XY, XYPN, XYZN],
    [XY, XZPN, YZPN, PN, ZN, ZP, 1, YZ, YP, YN, XZ, XP, XN, XYZP, XYZN, XYPN],
    [XZ, XYPN, PN, YZPN, YN, YP, YZ, 1, ZP, ZN, XY, XYZP, XYZN, XP, XN, XZPN],
    [XP, XYZN, ZN, YN, YZPN, YZ, YP, ZP, 1, PN, XYZP, XY, XYPN, XZ, XZPN, XN],
    [XN, XYZP, ZP, YP, YZ, YZPN, YN, ZN, PN, 1, XYZN, XYPN, XY, XZPN, XZ, XP],
    [YZ, PN, XYPN, XZPN, XN, XP, XZ, XY, XYZP, XYZN, 1, ZP, ZN, YP, YN, YZPN],
    [YP, ZN, XYZN, XN, XZPN, XZ, XP, XYZP, XY, XYPN, ZP, 1, PN, YZ, YZPN, YN],
    [YN, ZP, XYZP, XP, XZ, XZPN, XN, XYZN, XYPN, XY, ZN, PN, 1, YZPN, YZ, YP],
    [ZP, YN, XN, XYZN, XYPN, XY, XYZP, XP, XZ, XZPN, YP, YZ, YZPN, 1, PN, ZN],
    [ZN, YP, XP, XYZP, XY, XYPN, XYZN, XN, XZPN, XZ, YN, YZPN, YZ, PN, 1, ZP],
    [PN, YZ, XZ, XY, XYZP, XYZN, XYPN, XZPN, XN, XP, YZPN, YN, YP, ZN, ZP, 1],
];

const COMPONENTS_ODD_ODD: [[usize; 16]; 16] = COMPONENTS_EVEN_EVEN;

const SIGNS_EVEN_EVEN: [[i8; 16]; 16] = [
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
        // x + 1/2(x^2 + 1) n + 1/2(x^2 - 1)p
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
            start_index: VECTOR_START,
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