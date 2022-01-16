use json::JsonValue;

use crate::ifs::{self, IFS};
use crate::initial_set::{self, InitialSet};
use crate::plotters::{self, Plotter};
use crate::vector::Vec3;
use crate::multivector::Multivector;
use crate::point::{InternalPoint, OutputPoint};
use crate::fractal_metadata::FractalMetadata;

/// A generic IFS-based rendering algorithm like the Chaos Game and other
/// related algorithms
pub trait Algorithm {
    /// Perform the main iterations of the algorithm
    fn iterate(&mut self);
    /// Save the file to disk
    fn save(&mut self, fname: &str);
    /// Get the complexity of the algorithm measured by number of points in
    /// the output tileset.
    fn complexity(&self) -> usize;
}

const STARTUP_ITERS: usize = 10;

/// The basic Chaos Game algorithm (see Fractals Everywhere by Michael F. 
/// Barnsley)
pub struct ChaosGame {
    /// Metadata about the fractal. Used for 3D Tiles Next output
    metadata: FractalMetadata,
    /// IFS for transforming the points
    position_ifs: IFS,
    /// IFS for transforming the colors
    color_ifs: IFS,
    /// Octree-based plotter to store the resulting fractal/tiling
    output: Box<dyn Plotter>,
    /// Number of iterations to perform
    num_iters: usize,
}

impl ChaosGame {
    /// Parse from a JSON object of the form
    ///
    /// ```text
    /// {
    ///     "algorithm": "chaos",
    ///     "ifs": <IFS JSON>
    ///     "color_ifs": <IFS JSON>,
    ///     "iters": N,
    ///     "plotter": <Plotter JSON>
    /// }
    /// ```
    pub fn from_json(json: &JsonValue) -> Self {
        let position_ifs = ifs::from_json(&json["ifs"]);
        let color_ifs = ifs::from_json(&json["color_ifs"]);
        let plotter = plotters::from_json(&json["plotter"]);
        let num_iters = json["iters"]
            .as_usize()
            .expect("iters must be a positive integer");
        let metadata = FractalMetadata::from_json(&json);

        Self {
            metadata,
            position_ifs,
            color_ifs,
            output: plotter,
            num_iters,
        }
    }

    to_box!(Algorithm);
}

impl Algorithm for ChaosGame {
    fn iterate(&mut self) {
        // Start with a random position and color
        let mut pos = Multivector::from_vec3(&Vec3::random());
        let mut color_vec = Multivector::from_vec3(&Vec3::random_color());
        const UPDATE_FREQ: usize = 100000;
        let complexity = self.complexity() / UPDATE_FREQ;

        // For the basic chaos game, everything is the same feature
        const FEATURE_ID: u16 = 0;
        const POINT_ID: u16 = 0;

        for i in 0..(STARTUP_ITERS + self.num_iters) {
            // Skip the first few iterations as they are often not on 
            // the fractal.
            if i >= STARTUP_ITERS {
                let point = OutputPoint {
                    position: pos.to_vec3(),
                    color: color_vec.to_vec3(),
                    feature_id: FEATURE_ID,
                    iteration: i as u64,
                    point_id: POINT_ID,
                    last_xform: self.position_ifs.get_last_xform(),
                    last_color_xform: self.color_ifs.get_last_xform()
                };

                self.output.plot_point(point);
            }

            pos = self.position_ifs.transform(&pos);
            color_vec = self.color_ifs.transform(&color_vec);

            // Show progress every UPDATE_FREQ iterations
            if i > STARTUP_ITERS && i % UPDATE_FREQ == STARTUP_ITERS {
                println!(
                    "Completed ~{}/{} chunks of 100K iterations", 
                    (i - STARTUP_ITERS) / UPDATE_FREQ, 
                    complexity);
            }
        }
    }

    fn save(&mut self, fname: &str) {
        self.output.save(fname, &self.metadata);
    }

    /// The complexity of the basic chaos game is O(n) where n is the number
    /// of iterations
    fn complexity(&self) -> usize {
        self.num_iters
    }
}

/// Similar to ChaosGame, but instead of operating on a single input point,
/// this allows "condensation sets" (using Michael F. Barnsley's terminology),
/// which is a set of input points that gets transformed as a single unit
/// at each iteration.
pub struct ChaosSets {
    /// Metadata about the fractal. Used for 3D Tiles Next output
    metadata: FractalMetadata, 
    /// IFS for transforming the points
    position_ifs: IFS,
    /// IFS for transforming colors
    color_ifs: IFS,
    /// Pattern for the initial sets
    initial_set: Box<dyn InitialSet>,
    /// How many initial sets to create. Each one is transformed independently
    /// from the others.
    initial_copies: usize,
    /// Octree-based plotter for storing the output
    output: Box<dyn Plotter>,
    /// Number of iterations to perform.
    num_iters: usize,
}

impl ChaosSets {

    /// Apply the position/color IFS to a buffer, and produce a new buffer
    pub fn transform_buffer(
            &mut self, points: Vec<InternalPoint>, iteration: u64
            ) -> Vec<InternalPoint> {

        let old_positions = points.iter().map(|x| x.position.clone()).collect();
        let old_colors = points.iter().map(|x| x.color.clone()).collect();
        let new_positions = 
            self.position_ifs.transform_points(&old_positions);
        let new_colors = self.color_ifs.transform_points(&old_colors);

        let last_xform = self.position_ifs.get_last_xform();
        let last_color_xform = self.color_ifs.get_last_xform();

        points.iter().enumerate().map(|(i, point)| InternalPoint {
            position: new_positions[i].clone(),
            color: new_colors[i].clone(),
            feature_id: point.feature_id,
            iteration,
            point_id: point.point_id,
            last_xform,
            last_color_xform
        }).collect()
    }

    /// Parse a Chaos Sets instance from JSON of the form:
    ///
    /// ```text
    /// {
    ///     "algorithm": "chaos_sets",
    ///     "initial_set": <InitialSet JSON>,
    ///     "initial_set_copies": N,
    ///     "ifs": <IFS JSON>,
    ///     "color_ifs": <IFS JSON>,
    ///     "plotter": <Plotter JSON>,
    ///     "iters": M
    /// }
    /// ```
    pub fn from_json(json: &JsonValue) -> Self {
        let metadata = FractalMetadata::from_json(json);
        let position_ifs = ifs::from_json(&json["ifs"]);
        let color_ifs = ifs::from_json(&json["color_ifs"]);
        let arranger = initial_set::from_json(&json["initial_set"]);
        let plotter = plotters::from_json(&json["plotter"]);
        let initial_copies: usize = json["initial_set_copies"]
            .as_usize()
            .expect("initial_copies must be a positive integer");
        let num_iters = json["iters"]
            .as_usize()
            .expect("iters must be a positive integer");

        Self {
            metadata,
            position_ifs,
            color_ifs,
            initial_set: arranger,
            initial_copies,
            output: plotter,
            num_iters,
        }
    }

    to_box!(Algorithm);
}

impl Algorithm for ChaosSets {
    fn iterate(&mut self) {
        // Generate a number of initial sets. They will be transformed
        // independently. This helps to view more of the search space
        let mut buffers: Vec<Vec<InternalPoint>> = 
            (0..self.initial_copies)
                .map(|i| { self.initial_set.generate(i as u16) })
                .collect();

        // Only write the first copy to the output, since they are all in
        // the same location
        self.output.plot_points(&buffers[0]);

        // Every iteration, transform each buffer using the IFS, 
        // and plot the results in the output buffer.
        for i in 0..self.num_iters {
            let mut new_buffers: Vec<Vec<InternalPoint>> = Vec::new();
            for point_buffer in buffers.into_iter() {
                let new_buffer = self.transform_buffer(point_buffer, i as u64);
                self.output.plot_points(&new_buffer);
                new_buffers.push(new_buffer);
            }
            buffers = new_buffers;
        }
    }

    fn save(&mut self, fname: &str) {
        self.output.save(fname, &self.metadata);
    }

    /// Complexity in this case is O(m * n * p) where m is the points each 
    /// initial set, n is the number of copies of the initial set, and p is
    /// the number of iterations.
    fn complexity(&self) -> usize {
        let points_per_buf = self.initial_set.len();
        let points_per_iter = points_per_buf * self.initial_copies;
       
        // Add in the size of a single buffer to account for the 0-th
        // iteration.
        points_per_iter * self.num_iters + points_per_buf
    }
}

/// Parse an algorithm from a JSON object of the form:
///
/// ```text
/// {
///     "algorithm": "chaos" | "chaos_sets",
///     ...params
/// }
/// ```
pub fn from_json(json: &JsonValue) -> Box<dyn Algorithm> {
    let valid_algorithms: Vec<&str> = vec!["chaos", "chaos_sets"];
    let algorithm_id = &json["algorithm"]
        .as_str()
        .expect("algorithm must be a string");

    match &algorithm_id[..] {
        "chaos" => ChaosGame::from_json(&json).to_box(),
        "chaos_sets" => ChaosSets::from_json(&json).to_box(),
        _ => panic!("Algorithm must be one of, {:?}", valid_algorithms)
    }
}
