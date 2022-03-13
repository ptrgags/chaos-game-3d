use std::fmt::{Debug, Formatter, Result};

use json::JsonValue;

use crate::vector::Vec3;

/**
 * Bounding box structure
 *
 * This contains two vectors, one for the minimum extent (left, front, bottom)
 * and the maximum extent (right, back, top)
 */
pub struct BBox {
    /// Minimum coordinate (left, front, bottom)
    min: Vec3,
    /// Maximum coordinate (right, back, top)
    max: Vec3,
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
            min: Vec3::new(left, front, bottom),
            max: Vec3::new(right, back, top),
        }
    }

    /// The center of the box is the midpoint in each direction, that is
    /// (min + max) / 2
    pub fn center(&self) -> Vec3 {
        (self.min + self.max).scale(0.5)
    }

    /// Compute the length of the diagonal of this bounding box. This is
    /// used for estimating geometric error
    pub fn diagonal_len(&self) -> f32 { 
        let diagonal = self.max - self.min;
        diagonal.length()
    }

    /// Format this box in JSON format as used in the Cesium 3D Tiles Spec
    pub fn to_json(&self) -> JsonValue{
        let center = self.center();

        let diagonal = self.max - self.min;
        let bounds = array![
            *center.x(), *center.y(), *center.z(),
            0.5 * *diagonal.x(), 0.0, 0.0,
            0.0, 0.5 * *diagonal.y(), 0.0,
            0.0, 0.0, 0.5 * *diagonal.z()
        ];

        object!{
            "box" => bounds,
        }
    }

    /// Subdivide this bounding box into 8 octants, evenly divided along
    /// each axis. The children bounding boxes are returned in a vector
    /// ordered by quadrant number from 0b000 to 0b111
    pub fn subdivide(&self) -> Vec<Self> {
        let center = self.center();
        let cx = *center.x();
        let cy = *center.y();
        let cz = *center.z();

        let x1 = *self.min.x();
        let y1 = *self.min.y();
        let z1 = *self.min.z();

        let x2 = *self.max.x();
        let y2 = *self.max.y();
        let z2 = *self.max.z();

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

    /// Check if a point is contained inside this box. The minimum bounds
    /// are inclusive and the maximum bounds are exclusive, much like
    /// typical range checks when indexing.
    pub fn contains(&self, vec: &Vec3) -> bool {
        let x = *vec.x();
        let y = *vec.y();
        let z = *vec.z();

        let left = *self.min.x();
        let front = *self.min.y();
        let bottom = *self.min.z();

        let right = *self.max.x();
        let back = *self.max.y();
        let top = *self.max.z();

        left <= x && x < right && 
        front <= y && y < back &&
        bottom <= z && z < top
    }

    /// Determine which octant a point is in. There are 8 octants, numbered
    /// from 0 to 7, but it's better to think about them in binary.
    ///
    /// 0bZYX
    ///
    /// where Z = 1 if the z coordinate is greater than center z
    ///       Y = 1 if the y coordinate is greater than center y
    ///       X = 1 if the x coordinate is greater than center x
    pub fn find_octant(&self, vec: &Vec3) -> usize {
        let from_center = *vec - self.center();

        let x_positive = (*from_center.x() > 0.0) as usize;
        let y_positive = (*from_center.y() > 0.0) as usize;
        let z_positive = (*from_center.z() > 0.0) as usize;

        (z_positive << 2) | (y_positive << 1) | x_positive
    }
}

/// Debug format: (min, max)
impl Debug for BBox {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "(min: {:?}, max: {:?})", self.min, self.max)
    }
}
