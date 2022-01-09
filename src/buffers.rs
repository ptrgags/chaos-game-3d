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
    feature_ids: Vec<u16>,
    iterations: Vec<u64>,
    point_ids: Vec<u16>,
    last_xforms: Vec<u8>
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
            feature_ids: Vec::new(),
            iterations: Vec::new(),
            point_ids: Vec::new(),
            last_xforms: Vec::new(),
        }
    }

    /// Construct from parallel vectors of points and a vector of colors
    pub fn from_vectors(points: Vec<T>, colors: Vec<T>) -> Self {
        assert!(
            points.len() == colors.len(), 
            "points and colors must have the same length");

        Self {
            points,
            colors,
            feature_ids: Vec::new(),
            iterations: Vec::new(),
            point_ids: Vec::new(),
            last_xforms: Vec::new(),
        }
    }

    pub fn clear(&mut self) {
        self.points.clear();
        self.colors.clear();
    }

    /// Return the list of points without colors
    pub fn get_points(&self) -> &Vec<T> {
        return &self.points;
    }

    /// Return the list of colors without points
    pub fn get_colors(&self) -> &Vec<T> {
        return &self.colors;
    }

    /// Return the list of feature IDs
    pub fn get_feature_ids(&self) -> &Vec<u16> {
        return &self.feature_ids;
    }

    /// Return the list of iteration IDs
    pub fn get_iterations(&self) -> &Vec<u64> {
        return &self.iterations;
    }

    /// Return the list of last transformation indexes
    pub fn get_last_xforms(&self) -> &Vec<u8> {
        return &self.last_xforms;
    }

    /// Add a new point to the buffer
    pub fn add(&mut self, point: T, color: T) {
        //feature_id: u16, iteration: u64, point_id: u16, last_xform: u8) {
        self.points.push(point);
        self.colors.push(color);
        // TODO: this is getting ridiculous...
        /*
        self.feature_ids.push(feature_id);
        self.iterations.push(iteration);
        self.point_ids.push(point_id);
        self.last_xforms.push(last_xform);
        */
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
            feature_ids: &self.feature_ids,
            iterations: &self.iterations,
            point_ids: &self.point_ids,
            last_xforms: &self.last_xforms,
            index: 0
        }
    }
}

/// Iterate over a buffer's (point, color) pairs in read-only fashion
pub struct BufferIterator<'a, T: Clone> {
    points: &'a Vec<T>,
    colors: &'a Vec<T>,
    feature_ids: &'a Vec<u16>,
    iterations: &'a Vec<u64>,
    point_ids: &'a Vec<u16>,
    last_xforms: &'a Vec<u8>,
    index: usize
}

impl<'a, T: Clone> Iterator for BufferIterator<'a, T> {
    type Item = (&'a T, &'a T, &'a u16, &'a u64, &'a u16, &'a u8);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.points.len() {
            return None
        }

        let pos = &self.points[self.index];
        let color = &self.colors[self.index];
        let feature_id = &self.feature_ids[self.index];
        let iterations = &self.iterations[self.index];
        let point_id = &self.point_ids[self.index];
        let last_xform = &self.last_xforms[self.index];
        self.index += 1;
        
        Some((pos, color, feature_id, iterations, point_id, last_xform))
    }
}
