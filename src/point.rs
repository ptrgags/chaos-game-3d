use crate::vector::Vec3;
use crate::half_multivector::HalfMultivector;

/// A single point in the fractal point cloud. It has a position, color,
/// and other metadata for styling the fractal
#[derive(Clone)]
pub struct Point<T> {
    /// The position of the point in 3D space
    pub position: T,
    /// The color of the point in RGB space
    pub color: T,
    /// The u, (maybe) v, and (maybe) w coordinates within the cluster shape.
    /// The nature of this depends on the cluster type.
    pub cluster_coordinates: Vec3,
    /// The iteration number when this point was plotted
    pub iteration: u64,
    /// Which copy of the cluster
    pub cluster_copy: u16,
    /// Which sub-cluster (in the case of ManyCluster)
    pub cluster_id: u16,
    /// The ID of the point within the cluster
    pub point_id: u16,
    /// The index of the last transformation that was applied
    pub last_xform: u8,
    /// The index of the last color transformation that was applied
    pub last_color_xform: u8
}

/// Internally the point is represented as a multivector in geometric algebra
/// as this makes it easier to apply transformations
pub type InternalPoint = Point<HalfMultivector>;

/// When writing to disk, the point is converted to a vec3 for storage and
/// rendering. since OpenGL uses single precision float vectors, this is
/// a vec3 rather than a dvec3
pub type OutputPoint = Point<Vec3>;

impl From<InternalPoint> for OutputPoint {
    fn from(point: InternalPoint) -> Self {
        Self {
            position: point.position.to_vec3(),
            color: point.color.to_vec3(),
            cluster_coordinates: point.cluster_coordinates,
            iteration: point.iteration,
            cluster_copy: point.cluster_copy,
            cluster_id: point.cluster_id,
            point_id: point.point_id,
            last_xform: point.last_xform,
            last_color_xform: point.last_color_xform
        }
    }
}