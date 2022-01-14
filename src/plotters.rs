use json::JsonValue;

use crate::fractal_metadata::FractalMetadata;
use crate::octrees::OctNode;
use crate::tileset_writer::{TilesetWriter, TileType};
use crate::point::{InternalPoint, OutputPoint};


/// Octree-based point cloud plotter. There are different types for raw
/// scatter plots and density plots
pub trait Plotter {
    /// Plot a single point
    fn plot_point(&mut self, point: OutputPoint);

    /// Plot many points from a buffer. This is just an iteration of
    /// plot_point().
    fn plot_points(&mut self, points: &Vec<InternalPoint>) {
        for point in points {
            self.plot_point(OutputPoint::from(point.clone()));
        }
    }

    /// Save the plot to a tileset with the given directory name
    fn save(&mut self, dirname: &str, metadata: &FractalMetadata);
}

/// Scatter plots follow the usual scheme of octrees: add points to the node.
/// if a node becomes overfilled, split it into up to 8 child nodes.
///
/// This plotter also has a maximum depth to prevent infinite loops if
/// a voxel at the smallest resolution becomes full of points.
pub struct ScatterPlot {
    root: OctNode,
    max_depth: u8,
    tile_type: TileType
}

impl ScatterPlot {
    pub fn new(
            radius: f32,
            node_capacity: usize,
            max_depth: u8,
            tile_type: TileType) -> Self {
        Self {
            root: OctNode::root_node(radius, node_capacity),
            max_depth,
            tile_type
        }
    }

    /// Load a plotter from JSON of the form:
    /// {
    ///     "type": "scatter",
    ///     "format": "pnts",
    ///     "max_depth": d,
    ///     "node_capacity: n,
    ///     "radius": r,
    /// }
    pub fn from_json(json: &JsonValue) -> Self {
        let format = json["format"]
            .as_str()
            .expect("format must be a string");
        let tile_type = match format {
            "pnts" => TileType::Pnts,
            "glb" => TileType::Glb,
            _ => panic!("format must be either pnts or glb")
        };

        let max_depth = json["max_depth"]
            .as_u8()
            .expect("max_depth must be a positive integer");
        let capacity = json["node_capacity"]
            .as_usize()
            .expect("node_capacity must be a positive integer");
        let radius = json["radius"]
            .as_f32()
            .expect("radius must be a float");

        Self::new(radius, capacity, max_depth, tile_type)
    }

    to_box!(Plotter);
}

impl Plotter for ScatterPlot {
    fn plot_point(&mut self, point: OutputPoint) {
        self.root.add_point(point, self.max_depth);
    }

    /// Save the tileset into a directory of the given name. This creates
    /// the directory if it does not already exist
    fn save(&mut self, dirname: &str, metadata: &FractalMetadata) {
        // Decimate the mesh recursively to generate LODs
        self.root.decimate();
        let writer = TilesetWriter::new(
            self.tile_type.clone(), metadata.clone());
        writer.save(dirname, &self.root);
    }
}

/// Parse a point cloud plotter from a JSON object of the form:
///
/// ```text
/// {
///     "type": "scatter",
///     ...params
/// }
/// ```
pub fn from_json(json: &JsonValue) -> Box<dyn Plotter> {
    let valid_plotters: Vec<&str> = vec!["scatter"];
    let plotter_type = &json["type"]
        .as_str()
        .expect("plotter type must be a string");

    match &plotter_type[..] {
        "scatter" => ScatterPlot::from_json(&json).to_box(),
        _ => panic!("Plotter type must be one of, {:?}", valid_plotters)
    }
}
