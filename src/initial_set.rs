use crate::buffers::InternalBuffer;
use crate::vector::Vec3;
use crate::multivector::Multivector;

use rand::Rng;
use rand::prelude::ThreadRng;
use json::JsonValue;

/// This trait is used to arrange a set of points to represent an initial
/// set that will be sent through a Chaos Game algorithm. Typically, this is
/// done by randomly generating a set of points in some arrangement like a
/// box or line.
pub trait InitialSet {
    /// Generate a set of points. This may be called several times, and each
    /// time it must produce a new buffer. This uses a mutable reference since
    /// it must be 
    fn generate(&mut self) -> InternalBuffer; 
    /// Get the number of points in the initial set for measuring complexity.
    fn len(&self) -> usize;
}

/// Randomly generate N points constrained to a cuboid. The box is a solid
/// color.
pub struct RandomBox {
    /// Center of the box
    center: Vec3,
    /// width in the x, y, and z directions.
    dimensions: Vec3,
    /// The box starts off with a solid color (RGB from 0 to 1)
    color: Vec3,
    /// Number of points per box
    num_points: usize,
    /// Random number generator for generating points
    rng: ThreadRng,
}

impl RandomBox {
    pub fn new(
            center: Vec3, 
            dimensions: Vec3, 
            color: Vec3, 
            num_points: usize) -> Self {
        Self {
            center,
            dimensions,
            color,
            num_points,
            rng: rand::thread_rng(), 
        }
    }

    to_box!(InitialSet);

    /// Parse a RandomBox generator from JSON of the form:
    /// ```text
    /// {
    ///     "type": "box",
    ///     "center": [x, y, z],
    ///     "dimensions": [x, y, z],
    ///     "color": [r, g, b] // 0.0 to 1.0
    ///     "num_points": N
    /// }
    /// ```
    pub fn from_json(json: &JsonValue) -> Self {
        let center = Vec3::from_json(&json["center"], Vec3::zero());
        let dimensions = Vec3::from_json(&json["dims"], Vec3::ones());
        let color = Vec3::from_json(&json["color"], Vec3::ones());
        let num_points = &json["num_points"]
            .as_usize()
            .expect("num_points must be a positive integer");

        Self::new(center, dimensions, color, *num_points)
    }
}

impl InitialSet for RandomBox {
    fn generate(&mut self) -> InternalBuffer {
        let mut buf = InternalBuffer::new();

        // Find the bounding box for generating points
        let half_dims = self.dimensions.scale(0.5);
        let min = self.center - half_dims;
        let max = self.center + half_dims;
        let color = Multivector::from_vec3(&self.color);

        // Generate N random points, uniformly distributed over the box.
        for _ in 0..self.num_points {
            let x = self.rng.gen_range(min.x(), max.x());
            let y = self.rng.gen_range(min.y(), max.y());
            let z = self.rng.gen_range(min.z(), max.z());
        
            let point = Multivector::vector(x as f64, y as f64, z as f64);
            buf.add(point, color.clone());
        }

        buf
    }

    fn len(&self) -> usize {
        self.num_points
    }
}

/// Random points arranged in a line segment from start to end
pub struct RandomLine {
    /// Start point
    start: Vec3,
    /// End point
    end: Vec3,
    /// The line is a solid color
    color: Vec3,
    /// Generate N points
    num_points: usize,
    /// Random number generator for arranging points
    rng: ThreadRng,
}

impl RandomLine {
    pub fn new(
            start: Vec3, 
            end: Vec3, 
            color: Vec3, 
            num_points: usize) -> Self {
        Self {
            start, 
            end,
            color,
            num_points,
            rng: rand::thread_rng(), 
        }
    }

    /// Parse a RandomLine generator from JSON of the form:
    /// ```text
    /// {
    ///     "type": "line",
    ///     "start": [x, y, z],
    ///     "end": [x, y, z],
    ///     "color": [r, g, b] // 0.0 to 1.0
    ///     "num_points": N
    /// }
    /// ```
    pub fn from_json(json: &JsonValue) -> Self {
        let start = Vec3::from_json(&json["start"], Vec3::zero());
        let end = Vec3::from_json(&json["end"], Vec3::new(1.0, 0.0, 0.0));
        let color = Vec3::from_json(&json["color"], Vec3::ones());
        let num_points = &json["num_points"]
            .as_usize()
            .expect("num_points must be an integer");

        Self::new(start, end, color, *num_points)
    }

    to_box!(InitialSet);
}

impl InitialSet for RandomLine {
    fn generate(&mut self) -> InternalBuffer {
        let mut buf = InternalBuffer::new();
        let color = Multivector::from_vec3(&self.color);
        let start = Multivector::from_vec3(&self.start);
        let end = Multivector::from_vec3(&self.end);

        // Generate N random points, uniformly distributed over the 
        // line segment
        for _ in 0..self.num_points {
            let t = self.rng.gen_range(0.0, 1.0);
            let weighted_start = start.scale(1.0 - t);
            let weighted_end = end.scale(t);
            let point = weighted_start.add(&weighted_end);

            buf.add(point, color.clone());
        }

        buf
    }

    fn len(&self) -> usize {
        self.num_points
    }
}

/// Parse one of the initial set types from a JSON value of the form:
/// ```text
/// {
///     "type": "box" | "line",
///     ...params
/// }
/// ```
pub fn from_json(json: &JsonValue) -> Box<dyn InitialSet> {
    let valid_types: Vec<&str> = vec!["box", "line"];
    let type_id = &json["type"]
        .as_str()
        .expect("type must be a string");

    match &type_id[..] {
        "box" => RandomBox::from_json(&json).to_box(),
        "line" => RandomLine::from_json(&json).to_box(),
        _ => panic!(
            "Initial set type {} must be one of {:?}", type_id, valid_types)
    }
}
