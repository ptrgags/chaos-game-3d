use std::f64::consts::PI;
use rand::Rng;
use rand::prelude::ThreadRng;
use json::JsonValue;

use crate::vector::Vec3;
use crate::half_multivector::HalfMultivector;
use crate::point::InternalPoint;

/// This trait is used to arrange a set of points to represent an initial
/// set that will be sent through a Chaos Game algorithm. Typically, this is
/// done by randomly generating a set of points in some arrangement like a
/// box or line.
pub trait InitialSet {
    /// Generate a set of points. This may be called several times, and each
    /// time it must produce a new set of points.
    fn generate(&mut self, set_id: u16) -> Vec<InternalPoint>; 
    /// Get the number of points in the initial set for measuring complexity.
    fn len(&self) -> usize;
}

pub struct Points {
    positions: Vec<Vec3>,
    color: Vec3
}

impl Points {
    /// Parse a list of points from JSON of the form
    /// ```text
    /// {
    ///     "type": "points",
    ///     "positions": [
    ///         [x1, y1, z1],
    ///         [x2, y2, z2],
    ///         ...
    ///     ],
    ///     "color": [r, g, b] // 0.0 to 1.0
    /// }
    /// ```
    pub fn from_json(json: &JsonValue) -> Self {
        let mut positions = Vec::new();
        for position_json in json["positions"].members() {
            let position = Vec3::from_json(position_json, Vec3::zero());
            positions.push(position);
        }
        let color = Vec3::from_json(&json["color"], Vec3::ones());

        Self {
            positions,
            color: color
        }
    }

    to_box!(InitialSet);
}

impl InitialSet for Points {
    fn generate(&mut self, set_id: u16) -> Vec<InternalPoint> {
        let mut points = Vec::new();
        let color = HalfMultivector::from_vec3(&self.color);
        for (i, position) in self.positions.iter().enumerate() {
            let point = InternalPoint {
                position: HalfMultivector::from_vec3(&position),
                color: color.clone(),
                feature_id: set_id,
                iteration: 0,
                point_id: i as u16,
                last_xform: 0,
                last_color_xform: 0
            };
            points.push(point);
        }

        points
    }

    fn len(&self) -> usize {
        self.positions.len()
    }
}

/// Evenly spaced points along a line
pub struct Line {
    /// Start point
    start: Vec3,
    /// End point
    end: Vec3,
    /// The line is a solid color
    color: Vec3,
    /// Generate N points
    num_points: usize,
}

impl Line {
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

        Self {
            start,
            end,
            color,
            num_points: *num_points
        }
    }

    to_box!(InitialSet);
}

impl InitialSet for Line {
    fn generate(&mut self, set_id: u16) -> Vec<InternalPoint> {
        let mut points = Vec::new();
        let color = HalfMultivector::from_vec3(&self.color);

        let n = self.num_points as f32;

        // Generate N random points, uniformly distributed over the 
        // line segment
        for i in 0..self.num_points {
            let index = i as f32;
            let t = index / (n - 1.0);
            let position_vec3 = Vec3::lerp(&self.start, &self.end, t);
            let position = HalfMultivector::from_vec3(&position_vec3);

            let point = InternalPoint {
                position,
                color: color.clone(),
                feature_id: set_id,
                iteration: 0,
                point_id: i as u16,
                last_xform: 0,
                last_color_xform: 0
            };

            points.push(point);
        }

        points
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
    fn generate(&mut self, set_id: u16) -> Vec<InternalPoint> {
        let mut points = Vec::new();
        let color = HalfMultivector::from_vec3(&self.color);

        // Generate N random points, uniformly distributed over the 
        // line segment
        for i in 0..self.num_points {
            let t = self.rng.gen_range(0.0, 1.0);
            let position_vec3 = Vec3::lerp(&self.start, &self.end, t);
            let position = HalfMultivector::from_vec3(&position_vec3);

            let point = InternalPoint {
                position,
                color: color.clone(),
                feature_id: set_id,
                iteration: 0,
                point_id: i as u16,
                last_xform: 0,
                last_color_xform: 0
            };

            points.push(point);
        }

        points
    }

    fn len(&self) -> usize {
        self.num_points
    }
}

/// Evenly spaced points along a circle
pub struct Circle {
    /// center point
    center: Vec3,
    /// Radius of the circle
    radius: f64,
    /// x-axis of the circle. You're going to make this a unit vector, right? ;)
    x_dir: Vec3,
    /// y-axis of the circle. You're going to make this a unit vector orthogonal to x_dir, right? ;)
    y_dir: Vec3,
    /// The circle starts with a solid color
    color: Vec3,
    /// Generate N points along the circle
    num_points: usize,
}

impl Circle {
    /// Parse a RandomLine generator from JSON of the form:
    /// ```text
    /// {
    ///     "type": "circle",
    ///     "center": [x, y, z],
    ///     "x_dir": [xx, xy, xz],
    ///     "y_dir": [yx, yy, yz],
    ///     "color": [r, g, b] // 0.0 to 1.0
    ///     "num_points": N
    /// }
    /// ```
    pub fn from_json(json: &JsonValue) -> Self {
        let center = Vec3::from_json(&json["center"], Vec3::zero());
        let radius = &json["radius"].as_f64().unwrap_or(1.0);
        let x_dir = Vec3::from_json(&json["x_dir"], Vec3::new(1.0, 0.0, 0.0));
        let y_dir = Vec3::from_json(&json["y_dir"], Vec3::new(0.0, 1.0, 0.0));
        let color = Vec3::from_json(&json["color"], Vec3::ones());
        let num_points = &json["num_points"]
            .as_usize()
            .expect("num_points must be an integer");

        Self {
            center,
            radius: *radius,
            x_dir,
            y_dir,
            color,
            num_points: *num_points
        }
    }

    to_box!(InitialSet);
}

impl InitialSet for Circle {
    fn generate(&mut self, set_id: u16) -> Vec<InternalPoint> {
        let mut points = Vec::new();
        let color = HalfMultivector::from_vec3(&self.color);

        let n = self.num_points as f64;
        let r = self.radius;

        // Generate N random points, uniformly distributed over the 
        // line segment
        for i in 0..self.num_points {
            let index = i as f64;
            let t = 2.0 * PI * index / n;
            let x = (r * t.cos()) as f32;
            let y = (r * t.sin()) as f32;

            let position_vec3 = self.center + self.x_dir * x + self.y_dir * y;
            let position = HalfMultivector::from_vec3(&position_vec3);

            let point = InternalPoint {
                position,
                color: color.clone(),
                feature_id: set_id,
                iteration: 0,
                point_id: i as u16,
                last_xform: 0,
                last_color_xform: 0
            };

            points.push(point);
        }

        points
    }

    fn len(&self) -> usize {
        self.num_points
    }
}

/// A 2D quad of evenly-spaced points.
pub struct GridQuad {
    /// Center of the quad
    center: Vec3,
    /// x-axis of the quad. You're going to make this a unit vector, right? ;)
    x_dir: Vec3,
    /// y-axis of the quad. You're going to make this a unit vector orthogonal to x_dir, right? ;)
    y_dir: Vec3,
    /// Width of the quad in the x direction
    width: f64,
    /// height of the quad in the y-direction
    height: f64,
    /// The quad starts with a solid color
    color: Vec3,
    /// Generate up to N points (possibly less) in the grid
    num_points: usize
}

/// Parse a GridQuad generator from JSON of the form:
/// ```text
/// {
///     "type": "quad",
///     "center": [x, y, z],
///     "dims": [width, height],
///     "x_dir": [xx, xy, xz],
///     "y_dir": [yx, yy, yz],
///     "color": [r, g, b] // 0.0 to 1.0
///     "num_points": N
/// }
/// ```
impl GridQuad {
    pub fn from_json(json: &JsonValue) -> Self {
        let center = Vec3::from_json(&json["center"], Vec3::zero());
        let dims = &json["dims"];
        let width = dims[0].as_f64().unwrap_or(1.0);
        let height = dims[1].as_f64().unwrap_or(1.0);
        let x_dir = Vec3::from_json(&json["x_dir"], Vec3::new(1.0, 0.0, 0.0));
        let y_dir = Vec3::from_json(&json["y_dir"], Vec3::new(0.0, 1.0, 0.0));
        let color = Vec3::from_json(&json["color"], Vec3::ones());
        let num_points = &json["num_points"]
            .as_usize()
            .expect("num_points must be a positive integer");

        Self {
            center,
            x_dir,
            y_dir,
            width,
            height,
            num_points: *num_points,
            color
        }
    }

    to_box!(InitialSet);
}

impl InitialSet for GridQuad {
    fn generate(&mut self, set_id: u16) -> Vec<InternalPoint> {
        let mut grid = Vec::new();

        // Compute the effective grid size. If n points cannot be evenly
        // divided into the same ratio as width/height, the grid may be
        // slightly smaller
        // See https://www.desmos.com/calculator/ltvzx6ezec
        let n = self.num_points as f64;
        let area = self.width * self.height;
        let density = n / area;
        let sqrt_density = density.sqrt();
        let x_count = (self.width * sqrt_density).floor() as usize;
        let y_count = (self.height * sqrt_density).floor() as usize;
        let m = x_count * y_count;

        let color = HalfMultivector::from_vec3(&self.color);

        for i in 0..m {
            let row = i / x_count;
            let col = i % x_count;
            let u = (col as f64) / ((x_count - 1) as f64);
            let v = (row as f64) / ((y_count - 1) as f64);

            let x = (self.width * u) as f32;
            let y = (self.height * v) as f32;

            let position_vec3 = self.center + self.x_dir * x + self.y_dir * y;
            let position = HalfMultivector::from_vec3(&position_vec3);

            let point = InternalPoint {
                position,
                color: color.clone(),
                feature_id: set_id,
                iteration: 0,
                point_id: i as u16,
                last_xform: 0,
                last_color_xform: 0
            };

            grid.push(point);
        }

        grid
    }

    fn len(&self) -> usize {
        // TODO: I need to store the effective size
        1
    }
}

pub struct FibonacciDisk {
    /// center point
    center: Vec3,
    /// Radius of the circle
    radius: f64,
    /// x-axis of the circle. You're going to make this a unit vector, right? ;)
    x_dir: Vec3,
    /// y-axis of the circle. You're going to make this a unit vector orthogonal to x_dir, right? ;)
    y_dir: Vec3,
    /// The disk starts with a solid color
    color: Vec3,
    /// Generate N points along the circle
    num_points: usize,
}

impl FibonacciDisk {
    pub fn from_json(json: &JsonValue) -> Self {
        let center = Vec3::from_json(&json["center"], Vec3::zero());
        let radius = &json["radius"].as_f64().unwrap_or(1.0);
        let x_dir = Vec3::from_json(&json["x_dir"], Vec3::new(1.0, 0.0, 0.0));
        let y_dir = Vec3::from_json(&json["y_dir"], Vec3::new(0.0, 1.0, 0.0));
        let color = Vec3::from_json(&json["color"], Vec3::ones());
        let num_points = &json["num_points"]
            .as_usize()
            .expect("num_points must be a positive integer");

        Self {
            center,
            radius: *radius,
            x_dir,
            y_dir,
            num_points: *num_points,
            color
        }
    }

    to_box!(InitialSet);
}

impl InitialSet for FibonacciDisk {
    fn generate(&mut self, set_id: u16) -> Vec<InternalPoint> {
        // Golden ratio
        let phi = (1.0 + (5.0f64).sqrt()) / 2.0;
        let n = self.num_points as f64;
        let color = HalfMultivector::from_vec3(&self.color);
        let r = self.radius;
        let mut lattice = Vec::new();

        // Generate points based on the Fibonacci lattice.
        // See here:
        // http://extremelearning.com.au/how-to-evenly-distribute-points-on-a-sphere-more-effectively-than-the-canonical-fibonacci-lattice/
        for i in 0..self.num_points {
            let index = i as f64;
            let u = (index / phi) % 1.0;
            let v = index / n;

            // theta in the article
            let azimuth = 2.0 * PI * u;
            // r in the article
            let radius = v.sqrt();

            // Convert from polar to cartesian coordinates
            let x = (r * radius * azimuth.cos()) as f32;
            let y = (r * radius * azimuth.sin()) as f32;
            let position_vec3 = self.center + self.x_dir * x + self.y_dir * y;

            let position = HalfMultivector::from_vec3(&position_vec3);
            let point = InternalPoint {
                position,
                color: color.clone(),
                feature_id: set_id,
                iteration: 0,
                point_id: i as u16,
                last_xform: 0,
                last_color_xform: 0
            };
            lattice.push(point);
        }

        lattice
    }

    fn len(&self) -> usize {
        self.num_points
    }
}

struct FibonacciSphere {
    /// Center of the sphere
    center: Vec3,
    /// Radius of the sphere
    radius: f64,
    /// number of points to put on the surface of the sphere
    num_points: usize,
    /// The sphere starts off with a solid color
    color: Vec3,
}

impl FibonacciSphere {
    /// Parse a FibonacciSphere cluster from JSON of the form:
    /// ```text
    /// {
    ///     "type": "sphere",
    ///     "center": [x, y, z],
    ///     "radius": r,
    ///     "color": [r, g, b] // 0.0 to 1.0
    ///     "num_points": N
    /// }
    /// ```
    pub fn from_json(json: &JsonValue) -> Self {
        let center = Vec3::from_json(&json["center"], Vec3::zero());
        let radius = &json["radius"].as_f64().unwrap_or(1.0);
        let color = Vec3::from_json(&json["color"], Vec3::ones());
        let num_points = &json["num_points"]
            .as_usize()
            .expect("num_points must be a positive integer");

        Self {
            center,
            radius: *radius,
            num_points: *num_points,
            color
        }
    }

    to_box!(InitialSet);
}

impl InitialSet for FibonacciSphere {
    fn generate(&mut self, set_id: u16) -> Vec<InternalPoint> {
        // Golden ratio
        let phi = (1.0 + (5.0f64).sqrt()) / 2.0;
        let n = self.num_points as f64;
        let r = self.radius;
        let cx = *self.center.x() as f64;
        let cy = *self.center.y() as f64;
        let cz = *self.center.z() as f64;
        let color = HalfMultivector::from_vec3(&self.color);

        let mut lattice = Vec::new();

        // Generate points based on the Fibonacci lattice.
        // See here:
        // http://extremelearning.com.au/how-to-evenly-distribute-points-on-a-sphere-more-effectively-than-the-canonical-fibonacci-lattice/
        for i in 0..self.num_points {
            let index = i as f64;
            let u = (index / phi) % 1.0;
            let v = index / n;

            // theta in the article
            let azimuth = 2.0 * PI * u;
            // phi in the article
            let zenith = (1.0 - 2.0 * v).acos();
            let cos_aziumuth = azimuth.cos();
            let sin_aziumuth = azimuth.sin();
            let cos_zenith = zenith.cos();
            let sin_zenith = zenith.sin();

            // Convert from spherical to rectangular coordinates
            let x = r * cos_aziumuth * sin_zenith + cx;
            let y = r * sin_aziumuth * sin_zenith + cy;
            let z = r * cos_zenith + cz;

            let position = HalfMultivector::point(x, y, z);
            let point = InternalPoint {
                position,
                color: color.clone(),
                feature_id: set_id,
                iteration: 0,
                point_id: i as u16,
                last_xform: 0,
                last_color_xform: 0
            };
            lattice.push(point);
        }

        lattice
    }

    fn len(&self) -> usize {
        self.num_points
    }
}

/// Generate a box of evenly-spaced points
pub struct GridBox {
    /// Center of the box
    center: Vec3,
    /// width in the x, y, and z directions.
    dimensions: Vec3,
    /// x-axis of the box. You're going to make this a unit vector, right? ;)
    x_dir: Vec3,
    /// y-axis of the box. You're going to make this a unit vector orthogonal to x_dir, right? ;)
    y_dir: Vec3,
    /// z-axis of the box. You're going to make this a unit vector orthogonal to x_dir and y_dir, right? ;)
    z_dir: Vec3,
    /// The box starts off with a solid color (RGB from 0 to 1)
    color: Vec3,
    /// Number of points per box. If this is not evenly divisible
    /// by the product of all three dimensions, the actual number of points
    /// may be smaller than this number
    num_points: usize,
}

impl GridBox {
    /// Parse a GridBox generator from JSON of the form:
    /// ```text
    /// {
    ///     "type": "box",
    ///     "center": [x, y, z],
    ///     "x_dir": [xx, xy, xz],
    ///     "y_dir": [yx, yy, yz],
    ///     "z_dir": [zx, zy, zz],
    ///     "dims": [x, y, z],
    ///     "color": [r, g, b] // 0.0 to 1.0
    ///     "num_points": N
    /// }
    /// ```
    pub fn from_json(json: &JsonValue) -> Self {
        let center = Vec3::from_json(&json["center"], Vec3::zero());
        let dimensions = Vec3::from_json(&json["dims"], Vec3::ones());
        let x_dir = Vec3::from_json(&json["x_dir"], Vec3::new(1.0, 0.0, 0.0));
        let y_dir = Vec3::from_json(&json["y_dir"], Vec3::new(0.0, 1.0, 0.0));
        let z_dir = Vec3::from_json(&json["z_dir"], Vec3::new(0.0, 0.0, 1.0));
        let color = Vec3::from_json(&json["color"], Vec3::ones());
        let num_points = &json["num_points"]
            .as_usize()
            .expect("num_points must be a positive integer");

        Self {
            center,
            dimensions,
            x_dir,
            y_dir,
            z_dir,
            num_points: *num_points,
            color
        }
    }

    to_box!(InitialSet);
}

impl InitialSet for GridBox {
    fn generate(&mut self, set_id: u16) -> Vec<InternalPoint> {
        let mut grid = Vec::new();

        let n = self.num_points as f64;

        let dims_x = *self.dimensions.x() as f64;
        let dims_y = *self.dimensions.y() as f64;
        let dims_z = *self.dimensions.z() as f64;

        let volume = dims_x * dims_y * dims_z;
        let density = n / volume;
        let cbrt_density = density.powf(1.0 / 3.0);

        let x_count = (dims_x * cbrt_density).floor() as usize;
        let y_count = (dims_y * cbrt_density).floor() as usize;
        let z_count = (dims_z * cbrt_density).floor() as usize;
        let m = x_count * y_count * z_count;

        let color = HalfMultivector::from_vec3(&self.color);

        for i in 0..m {
            let layer = i / (x_count * y_count);
            let row = i / x_count;
            let col = i % x_count;
            let u = (col as f64) / ((x_count - 1) as f64);
            let v = (row as f64) / ((y_count - 1) as f64);
            let w = (layer as f64) / ((z_count - 1) as f64);

            let x = (dims_x * u) as f32;
            let y = (dims_y * v) as f32;
            let z = (dims_z * w) as f32;

            let position_vec3 = 
                self.center + 
                self.x_dir * x +
                self.y_dir * y + 
                self.z_dir * z;
            let position = HalfMultivector::from_vec3(&position_vec3);

            let point = InternalPoint {
                position,
                color: color.clone(),
                feature_id: set_id,
                iteration: 0,
                point_id: i as u16,
                last_xform: 0,
                last_color_xform: 0
            };

            grid.push(point);
        }

        grid
    }

    fn len(&self) -> usize {
        // TODO: I need to store the effective size
        self.num_points
    }
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
    fn generate(&mut self, set_id: u16) -> Vec<InternalPoint> {
        let mut points = Vec::new();

        // Find the bounding box for generating points
        let half_dims = self.dimensions.scale(0.5);
        let min = self.center - half_dims;
        let max = self.center + half_dims;
        let color = HalfMultivector::from_vec3(&self.color);

        // Generate N random points, uniformly distributed over the box.
        for i in 0..self.num_points {
            let x = self.rng.gen_range(min.x(), max.x());
            let y = self.rng.gen_range(min.y(), max.y());
            let z = self.rng.gen_range(min.z(), max.z());
            
            let position = HalfMultivector::point(x as f64, y as f64, z as f64);

            let point = InternalPoint {
                position,
                color: color.clone(),
                feature_id: set_id,
                iteration: 0,
                point_id: i as u16,
                last_xform: 0,
                last_color_xform: 0
            };

            points.push(point);
        }

        points
    }

    fn len(&self) -> usize {
        self.num_points
    }
}

/// Parse one of the initial set types from a JSON value of the form:
/// ```text
/// {
///     "type": 
///         "points" | 
///         "line" | 
///         "rand_line" | 
///         "circle" | 
///         "quad" |
///         "disk" |
///         "sphere" | 
///         "box" |
///         "rand_box"
///     ...params
/// }
/// ```
pub fn from_json(json: &JsonValue) -> Box<dyn InitialSet> {
    let valid_types: Vec<&str> = vec![
        "points",
        "line",
        "rand_line",
        "circle",
        "quad",
        "disk",
        "sphere",
        "box",
        "rand_box"
    ];
    let type_id = &json["type"]
        .as_str()
        .expect("type must be a string");

    match &type_id[..] {
        // 0-dimensional
        "points" => Points::from_json(&json).to_box(),
        // 1-dimensional
        "line" => Line::from_json(&json).to_box(),
        "rand_line" => RandomLine::from_json(&json).to_box(),
        "circle" => Circle::from_json(&json).to_box(),
        // 2-dimensional
        "quad" => GridQuad::from_json(&json).to_box(),
        "disk" => FibonacciDisk::from_json(&json).to_box(),
        "sphere" => FibonacciSphere::from_json(&json).to_box(),
        // 3-dimensional
        "box" => GridBox::from_json(&json).to_box(),
        "rand_box" => RandomBox::from_json(&json).to_box(),
        _ => panic!(
            "Initial set type {} must be one of {:?}", type_id, valid_types)
    }
}
