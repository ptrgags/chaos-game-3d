use json::JsonValue;

use crate::fractal_metadata::FractalMetadata;
use crate::fractal_dimension::BoxCountingEstimator;
use crate::octrees::OctNode;
use crate::tileset_writer::{TilesetWriter, ContentType};
use crate::point::{InternalPoint, OutputPoint};

/// Octree-based point cloud plotter. There are different types for raw
/// scatter plots and density plots
pub trait Plotter {
    /// Plot a single point
    fn plot_point(&mut self, point: InternalPoint);

    /// Plot many points from a buffer. This is just an iteration of
    /// plot_point().
    fn plot_points(&mut self, points: &Vec<InternalPoint>) {
        for point in points {
            self.plot_point(point.clone());
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
    /// Root of the octree
    root: OctNode,
    /// Fractal dimension estimator
    dimension_estimator: BoxCountingEstimator,
    /// Depth of the octree.
    max_depth: u8,
    /// Are the tiles pnts or glb?
    tile_type: ContentType
}

impl ScatterPlot {
    /// Load a plotter from JSON of the form:
    /// {
    ///     "type": "scatter",
    ///     "format": "pnts" | "glb" (default "glb"),
    ///     "max_depth": d (default 10),
    ///     "node_capacity: n (default 5000),
    ///     "radius": r,
    ///     "fractal_dimension_depth: n (default 20)
    /// }
    pub fn from_json(json: &JsonValue) -> Self {
        let format = json["format"]
            .as_str()
            .unwrap_or("glb");
        let tile_type = match format {
            "pnts" => ContentType::Pnts,
            "glb" => ContentType::Glb,
            _ => panic!("format must be either pnts or glb")
        };

        let max_depth = json["max_depth"].as_u8().unwrap_or(10);
        let fractal_dimension_levels = json["fractal_dimension_levels"]
            .as_usize()
            .unwrap_or(20);
        let capacity = json["node_capacity"].as_usize().unwrap_or(5000);
        let radius = json["radius"]
            .as_f32()
            .expect("radius must be a float");
        
        let root = OctNode::root_node(radius, capacity);
        let dimension_estimator = BoxCountingEstimator::new(
            radius as f64, fractal_dimension_levels);

        Self {
            root,
            dimension_estimator,
            max_depth,
            tile_type
        }
    }

    to_box!(Plotter);
}

impl Plotter for ScatterPlot {
    fn plot_point(&mut self, point: InternalPoint) {
        self.dimension_estimator.add_point(&point.position);
        self.root.add_point(OutputPoint::from(point), self.max_depth);
    }

    /// Save the tileset into a directory of the given name. This creates
    /// the directory if it does not already exist
    fn save(&mut self, dirname: &str, metadata: &FractalMetadata) {
        let mut metadata = metadata.clone();
        self.dimension_estimator.update_metadata(&mut metadata);

        // Decimate the mesh recursively to generate LODs
        self.root.decimate();
        let writer = TilesetWriter::new(
            self.tile_type.clone(), metadata);
        writer.save(dirname, &self.root);
    }
}

/// Parse a point cloud plotter from a JSON object of the form:
///
/// ```text
/// {
///     "type": "scatter" (default "scatter"),
///     ...params
/// }
/// ```
pub fn from_json(json: &JsonValue) -> Box<dyn Plotter> {
    let valid_plotters: Vec<&str> = vec!["scatter"];
    let plotter_type = &json["type"].as_str().unwrap_or("scatter");

    match &plotter_type[..] {
        "scatter" => ScatterPlot::from_json(&json).to_box(),
        _ => panic!("Plotter type must be one of, {:?}", valid_plotters)
    }
}
