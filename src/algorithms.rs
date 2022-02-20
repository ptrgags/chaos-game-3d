use json::JsonValue;

use crate::ifs::{self, IFS};
use crate::clusters::{self, Cluster};
use crate::plotters::{self, Plotter};
use crate::vector::Vec3;
use crate::half_multivector::HalfMultivector;
use crate::point::{InternalPoint};
use crate::fractal_metadata::FractalMetadata;

/// A generic IFS-based rendering algorithm like the Chaos Game and other
/// related algorithms
pub trait Algorithm {
    /// Perform the main iterations of the algorithm
    fn iterate(&mut self);
    /// Save the file to disk
    fn save(&mut self);
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
        let mut pos = HalfMultivector::from_vec3(&Vec3::random());
        let mut color_vec = HalfMultivector::from_vec3(&Vec3::random_color());
        const UPDATE_FREQ: usize = 100000;
        let complexity = self.complexity() / UPDATE_FREQ;

        // For the basic chaos game, everything is the same feature
        let cluster_coordinates: Vec3 = Vec3::zero();

        for i in 0..(STARTUP_ITERS + self.num_iters) {
            // Skip the first few iterations as they are often not on 
            // the fractal.
            if i >= STARTUP_ITERS {
                let point = InternalPoint {
                    position: pos.clone(),
                    color: color_vec.clone(),
                    cluster_coordinates,
                    iteration: i as u64,
                    cluster_copy: 0,
                    cluster_id: 0,
                    point_id: 0,
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

    fn save(&mut self) {
        let fname = format!("./viewer/{}", self.metadata.id);
        self.output.save(&fname, &self.metadata);
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
    cluster: Box<dyn Cluster>,
    /// How many initial clusters to create. Each one is transformed independently
    /// from the others.
    cluster_copies: usize,
    /// Octree-based plotter for storing the output
    output: Box<dyn Plotter>,
    /// Number of iterations to perform.
    num_iters: usize,
}

impl ChaosSets {

    /// Apply the position/color IFS to a buffer, and produce a new buffer
    pub fn transform_cluster(
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
            cluster_coordinates: point.cluster_coordinates.clone(),
            iteration,
            cluster_copy: point.cluster_copy,
            cluster_id: point.cluster_id,
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
    ///     "cluster": <Cluster JSON>,
    ///     "cluster_copies": N,
    ///     "ifs": <IFS JSON>,
    ///     "color_ifs": <IFS JSON>,
    ///     "plotter": <Plotter JSON>,
    ///     "iters": M
    /// }
    /// ```
    pub fn from_json(json: &JsonValue) -> Self {
        let position_ifs = ifs::from_json(&json["ifs"]);
        let color_ifs = ifs::from_json(&json["color_ifs"]);
        let cluster = clusters::from_json(&json["cluster"]);
        let plotter = plotters::from_json(&json["plotter"]);
        let cluster_copies: usize = json["cluster_copies"]
            .as_usize()
            .expect("initial_copies must be a positive integer");
        let num_iters = json["iters"]
            .as_usize()
            .expect("iters must be a positive integer");
        let mut metadata = FractalMetadata::from_json(json);
        metadata.cluster_point_count = cluster.point_count() as u16;
        metadata.subcluster_max_point_count = 
            cluster.subcluster_max_point_count() as u16;

        Self {
            metadata,
            position_ifs,
            color_ifs,
            cluster,
            cluster_copies,
            output: plotter,
            num_iters,
        }
    }

    to_box!(Algorithm);

    /// Iterate a single cluster
    fn iterate_cluster(&mut self, cluster_copy: u16) {
        // Some IFS choosers are stateful, so reset the state to ensure
        // each cluster gets a unique path
        // NOTE: for the future: this is not thread-safe. If I want to
        // use threading someday, each thread needs a copy of the chooser
        self.position_ifs.reset();
        self.color_ifs.reset();

        let mut buffer = self.cluster.generate(cluster_copy, 0);
        self.output.plot_points(&buffer);

        for i in 0..self.num_iters {
            let new_buffer = self.transform_cluster(buffer, i as u64);
            self.output.plot_points(&new_buffer);
            buffer = new_buffer;
        }
    }
}

impl Algorithm for ChaosSets {
    fn iterate(&mut self) {
        for i in 0..self.cluster_copies {
            self.iterate_cluster(i as u16);
        }
    }

    fn save(&mut self) {
        let fname = format!("./viewer/{}", self.metadata.id);
        self.output.save(&fname, &self.metadata);
    }

    /// Complexity in this case is O(m * n * p) where m is the points each 
    /// initial set, n is the number of copies of the initial set, and p is
    /// the number of iterations.
    fn complexity(&self) -> usize {
        let points_per_buf = self.cluster.point_count();
        let points_per_iter = points_per_buf * self.cluster_copies;
       
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
