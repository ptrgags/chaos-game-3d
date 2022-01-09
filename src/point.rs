use crate::vector::Vec3;
use crate::multivector::Multivector;

#[derive(Clone)]
pub struct Point<T> {
    pub position: T,
    pub color: T,
    pub feature_id: u16,
    pub iteration: u64,
    pub point_id: u16,
    pub last_xform: u8,
    pub last_color_xform: u8
}

pub type InternalPoint = Point<Multivector>;

pub type OutputPoint = Point<Vec3>;

impl From<InternalPoint> for OutputPoint {
    fn from(point: InternalPoint) -> Self {
        Self {
            position: point.position.to_vec3(),
            color: point.color.to_vec3(),
            feature_id: point.feature_id,
            iteration: point.iteration,
            point_id: point.point_id,
            last_xform: point.last_xform,
            last_color_xform: point.last_color_xform
        }
    }
}