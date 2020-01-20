use json::JsonValue;

use crate::ifs::{self, IFS};
use crate::initial_set::{self, InitialSet};
use crate::buffers::Buffer;
use crate::vector::Vec3;
use crate::pointclouds::{Cesium3DTilesWriter, PointCloudWriter};

/// A generic IFS-based rendering algorithm like the Chaos Game and other
/// related algorithms
pub trait Algorithm {
    /// Perform the main iterations
    /// TODO: Grab the iterations from JSON rather than the command line
    fn iterate(&mut self, n_iters: u32);

    /// Save the file to disk
    fn save(&mut self, fname: &str);
}

const STARTUP_ITERS: u32 = 10;

/// The basic Chaos Game algorithm (see Fractals Everywhere by Michael F. 
/// Barnsley)
pub struct ChaosGame {
    /// IFS for transforming the points
    position_ifs: IFS<f32>,
    /// IFS for transforming the colors
    color_ifs: IFS<f32>,
    /// Buffer to hold the results before saving to disk
    output_buffer: Buffer
}

impl ChaosGame {
    pub fn new(position_ifs: IFS<f32>, color_ifs: IFS<f32>) -> Self {
        Self {
            position_ifs,
            color_ifs,
            output_buffer: Buffer::new()
        }
    }

    /// Parse from a JSON object of the form
    ///
    /// ```text
    /// {
    ///     "algorithm": "chaos",
    ///     "ifs": <IFS JSON>
    ///     "color_ifs": <IFS JSON>
    /// }
    /// ```
    pub fn from_json(json: &JsonValue) -> Self {
        let position_ifs = ifs::from_json(&json["ifs"]);
        let color_ifs = ifs::from_json(&json["color_ifs"]);

        Self::new(position_ifs, color_ifs)
    }

    to_box!(Algorithm);
}

impl Algorithm for ChaosGame {
    fn iterate(&mut self, n_iters: u32) {
        // Start with a random position and color
        let mut pos = Vec3::random();
        let mut color_vec = Vec3::random_color();

        for i in 0..(STARTUP_ITERS + n_iters) {
            // Skip the first few iterations as they are often not on 
            // the fractal.
            if i >= STARTUP_ITERS {
                self.output_buffer.add(pos, color_vec)
            }

            pos = self.position_ifs.transform(&pos);
            color_vec = self.color_ifs.transform(&color_vec);
        }
    }

    fn save(&mut self, fname: &str) {
        let mut writer = Cesium3DTilesWriter::new(10000000.0);
        writer.add_points(&mut self.output_buffer);
        writer.save(fname);
    }
}

/// Similar to ChaosGame, but instead of operating on a single input point,
/// this allows "condensation sets" (using Michael F. Barnsley's terminology),
/// which is a set of input points that gets transformed as a single unit
/// at each iteration.
pub struct ChaosSets { 
    /// IFS for transforming the points
    position_ifs: IFS<f32>,
    /// IFS for transforming colors
    color_ifs: IFS<f32>,
    /// Pattern for the initial sets
    initial_set: Box<dyn InitialSet>,
    /// How many initial sets to create. Each one is transformed independently
    /// from the others.
    initial_copies: usize,
    /// Buffer to hold the results before saving to disk
    /// TODO: Definitely change this to an Octree once available
    output_buffer: Buffer,
}

impl ChaosSets {
    pub fn new(
            position_ifs: IFS<f32>, 
            color_ifs: IFS<f32>, 
            initial_set: Box<dyn InitialSet>, 
            initial_copies: usize) -> Self {
        Self {
            position_ifs,
            color_ifs,
            initial_set,
            initial_copies,
            output_buffer: Buffer::new(),
        }
    }

    /// Apply the position/color IFS to a buffer, and produce a new buffer
    pub fn transform_buffer(&mut self, buffer: Buffer) -> Buffer {
        let new_positions = 
            self.position_ifs.transform_points(buffer.get_points());
        let new_colors = self.color_ifs.transform_points(buffer.get_colors());

        Buffer::from_vectors(new_positions, new_colors)
    }

    /// Parse a Chaos Sets instance from JSON of the form:
    ///
    /// ```text
    /// {
    ///     "algorithm": "chaos_sets",
    ///     "initial_set": <InitialSet JSON>,
    ///     "initial_set_copies": N,
    ///     "ifs": <IFS JSON>,
    ///     "color_ifs": <IFS JSON>
    /// }
    /// ```
    pub fn from_json(json: &JsonValue) -> Self {
        let position_ifs = ifs::from_json(&json["ifs"]);
        let color_ifs = ifs::from_json(&json["color_ifs"]);
        let arranger = initial_set::from_json(&json["initial_set"]);
        let initial_copies: usize = json["initial_set_copies"]
            .as_usize()
            .expect("initial_copies must be a positive integer");

        Self::new(position_ifs, color_ifs, arranger, initial_copies)
    }

    to_box!(Algorithm);
}

impl Algorithm for ChaosSets {
    fn iterate(&mut self, n_iters: u32) {
        // Generate a number of initial sets. They will be transformed
        // independently. This helps to view more of the search space
        let mut buffers: Vec<Buffer> = (0..self.initial_copies).map(|_| {
            self.initial_set.generate()
        }).collect();

        // Only write the first copy to the output, since they are all in
        // the same location
        self.output_buffer.copy_from(&buffers[0]);

        // Every iteration, transform each buffer using the IFS, 
        // and plot the results in the output buffer.
        // TODO: This could totally be done in parallel. Try Rust threads!
        for _ in 0..n_iters {
            let mut new_buffers: Vec<Buffer> = Vec::new();
            for buf in buffers.into_iter() {
                let new_buf = self.transform_buffer(buf);
                self.output_buffer.copy_from(&new_buf);
                new_buffers.push(new_buf);
            }
            buffers = new_buffers;
        }
    }

    fn save(&mut self, fname: &str) {
        let mut writer = Cesium3DTilesWriter::new(10000000.0);
        writer.add_points(&mut self.output_buffer);
        writer.save(fname);
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
    let algorithm_id = &json["algorithm"]
        .as_str()
        .expect("algorithm must be a string");

    match &algorithm_id[..] {
        "chaos" => ChaosGame::from_json(&json).to_box(),
        "chaos_sets" => ChaosSets::from_json(&json).to_box(),
        _ => panic!("invalid algorithm!, {}", &algorithm_id[..])
    }
}
