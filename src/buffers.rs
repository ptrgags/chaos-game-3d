use crate::vector::Vec3;
use crate::multivector::Multivector;

/// A buffer is a container of colored points.
/// It is stored as a pair of parallel vectors of points and colors.
/// The type is generic because this is used both for containing multivectors
/// when computing points and the more compact Vec3 when writing points to a
/// file
#[derive(Clone)]
pub struct Buffer<T: Clone> {
    points: Vec<T>,
    colors: Vec<T>,
}

/// Since transformations (see xforms.rs) are maps of 
/// `Multivector -> Multivector`, algorithms should store lists of multivectors
/// not Vec3 to avoid excessive packing/unpacking.
pub type InternalBuffer = Buffer<Multivector>;

/// When outputing a point cloud, use the more compact vector of point
pub type OutputBuffer = Buffer<Vec3>;

impl<T: Clone> Buffer<T> {
    pub fn new() -> Self {
        Self {
            points: Vec::new(),
            colors: Vec::new(),
        }
    }

    /// Construct from parallel vectors of points and a vector of colors
    pub fn from_vectors(points: Vec<T>, colors: Vec<T>) -> Self {
        assert!(
            points.len() == colors.len(), 
            "points and colors must have the same length");

        Self {
            points,
            colors
        }
    }

    /// Return the list of points without colors
    pub fn get_points(&self) -> &Vec<T> {
        return &self.points;
    }

    /// Return the list of colors without points
    pub fn get_colors(&self) -> &Vec<T> {
        return &self.colors;
    }

    /// Move points from other to the end of self, leaving other empty.
    pub fn move_from(&mut self, other: &mut Self) {
        self.points.append(&mut other.points);
        self.colors.append(&mut other.colors);
    }

    /// Copy points from other to the end of self.
    pub fn copy_from(&mut self, other: &Self) {
        self.points.extend(other.points.iter().cloned());
        self.colors.extend(other.colors.iter().cloned());
    }

    /// Add a new point to the buffer
    pub fn add(&mut self, point: T, color: T) {
        self.points.push(point);
        self.colors.push(color);
    }

    /// How many points are in this buffer?
    pub fn len(&self) -> usize {
        return self.points.len();
    }

    /// Get an iterator over this buffer. This iterator clones values
    pub fn points_iter(&self) -> BufferIterator<T> {
        BufferIterator {
            points: &self.points,
            colors: &self.colors,
            index: 0
        }
    }
}

/// Iterate over a buffer's (point, color) pairs in read-only fashion
pub struct BufferIterator<'a, T: Clone> {
    points: &'a Vec<T>,
    colors: &'a Vec<T>,
    index: usize
}

impl<'a, T: Clone> Iterator for BufferIterator<'a, T> {
    type Item = (&'a T, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.points.len() {
            return None
        }

        let pos = &self.points[self.index];
        let color = &self.colors[self.index];
        self.index += 1;
        
        Some((pos, color))
    }
}
