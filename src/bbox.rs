use json::JsonValue;

use crate::vector::Vec3;

const X_BIT: usize = 0b001usize;
const Y_BIT: usize = 0b010usize;
const Z_BIT: usize = 0b100usize;

pub struct BBox {
    left: f32,
    right: f32,
    front: f32,
    back: f32,
    bottom: f32,
    top: f32,
}

impl BBox {
    pub fn new(
            left: f32, 
            right: f32, 
            front: f32, 
            back: f32, 
            bottom: f32, 
            top: f32) -> Self {
        Self {
            left, right,
            front, back,
            bottom, top
        }
    }

    pub fn center(&self) -> Vec3 {
        Vec3::new(
            (self.left + self.right) / 2.0,
            (self.front + self.back) / 2.0,
            (self.bottom + self.top) / 2.0)
    }

    /// Compute the length of the diagonal of this bounding box. This is
    /// used for estimating geometric error
    pub fn diagonal_len(&self) -> f32 { 
        let dx = self.right - self.left;
        let dy = self.back - self.front;
        let dz = self.top - self.bottom;

        (dx * dx + dy * dy + dz * dz).sqrt()
    }

    /// Format this box in JSON format as used in the Cesium 3D Tiles Spec
    pub fn to_json(&self) -> JsonValue{
        let center = self.center();
        let dx = self.right - self.left;
        let dy = self.back - self.front;
        let dz = self.top - self.bottom;

        array![
            *center.x(), *center.y(), *center.z(),
            0.5 * dx, 0.0, 0.0,
            0.0, 0.5 * dy, 0.0,
            0.0, 0.0, 0.5 * dz 
        ]
    }

    pub fn subdivide(&self) -> Vec<Self> {
        let center = self.center();
        let cx = *center.x();
        let cy = *center.y();
        let cz = *center.z();

        let x1 = self.left;
        let x2 = self.right;
        let y1 = self.front;
        let y2 = self.back;
        let z1 = self.bottom;
        let z2 = self.top;

        vec![
            Self::new(x1, cx, y1, cy, z1, cz),
            Self::new(cx, x2, y1, cy, z1, cz),
            Self::new(x1, cx, cy, y2, z1, cz),
            Self::new(cx, x2, cy, y2, z1, cz),
            Self::new(x1, cx, y1, cy, cz, z2),
            Self::new(cx, x2, y1, cy, cz, z2),
            Self::new(x1, cx, cy, y2, cz, z2),
            Self::new(cx, x2, cy, y2, cz, z2),
        ]
    }

    pub fn contains(&self, vec: &Vec3) -> bool {
        let x = *vec.x();
        let y = *vec.y();
        let z = *vec.z();

        (
            self.left <= x && x < self.right && 
            self.front <= y && y < self.back &&
            self.bottom <= z && z < self.top
        )
    }

    pub fn find_quadrant(&self, vec: &Vec3) -> usize {
        let x_positive = (*vec.x() > 0.0) as usize;
        let y_positive = (*vec.y() > 0.0) as usize;
        let z_positive = (*vec.z() > 0.0) as usize;

        x_positive | (y_positive << 1) | (z_positive << 2)
    }

    pub fn make_quadrant(&self, quadrant: usize) -> Self {
        let x_positive = (quadrant & X_BIT) != 0;
        let y_positive = (quadrant & Y_BIT) != 0;
        let z_positive = (quadrant & Z_BIT) != 0;

        let center = self.center();
        let cx = *center.x();
        let cy = *center.y();
        let cz = *center.z();

        let x1 = self.left;
        let x2 = self.right;
        let y1 = self.front;
        let y2 = self.back;
        let z1 = self.bottom;
        let z2 = self.top;

        let left = if x_positive { cx } else { x1 };
        let right = if x_positive { x2 } else { cx };
        let front = if y_positive { cy } else { y1 };
        let back = if y_positive { y2 } else { cy };
        let bottom = if z_positive { cz } else { z1 };
        let top = if z_positive { z2 } else { cz };

        Self::new(left, right, front, back, top, bottom)
    }
}
