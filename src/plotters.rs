use json::JsonValue;

use crate::vector::Vec3;
use crate::octrees::OctNode;
use crate::buffers::Buffer;

/// Octree-based point cloud plotter. There are different types for raw
/// scatter plots and density plots
pub trait Plotter {
    /// Plot a single point
    fn plot_point(&mut self, point: Vec3, color: Vec3);

    /// Plot many points from a buffer. This is just an iteration of
    /// plot_point().
    fn plot_buffer(&mut self, buffer: &Buffer) {
        for (point, color) in buffer.points_iter() {
            self.plot_point(point, color);
        }
    }

    /// Save the plot to a tileset with the given directory name
    fn save(&mut self, dirname: &str);
}

/// Scatter plots follow the usual scheme of octrees: add points to the node.
/// if a node becomes overfilled, split it into up to 8 child nodes.
///
/// This plotter also has a maximum depth to prevent infinite loops if
/// a voxel at the smallest resolution becomes full of points.
pub struct ScatterPlot {
    root: OctNode,
    max_depth: u8,
}

impl ScatterPlot {
    pub fn new(radius: f32, node_capacity: usize, max_depth: u8) -> Self {
        Self {
            root: OctNode::root_node(radius, node_capacity),
            max_depth,
        }
    }

    /// Load a plotter from JSON of the form:
    /// {
    ///     "type": "scatter",
    ///     "max_depth": d,
    ///     "node_capacity: n,
    ///     "radius": r,
    /// }
    pub fn from_json(json: &JsonValue) -> Self {
        let max_depth = json["max_depth"]
            .as_u8()
            .expect("node_capacity must be a positive integer");
        let capacity = json["node_capacity"]
            .as_usize()
            .expect("node_capacity must be a positive integer");
        let radius = json["radius"]
            .as_f32()
            .expect("radius must be a float");

        Self::new(radius, capacity, max_depth)
    }

    to_box!(Plotter);
}

impl Plotter for ScatterPlot {
    fn plot_point(&mut self, _point: Vec3, _color: Vec3) {
        panic!("Not implemented yet!");
    }

    fn save(&mut self, _dirname: &str) {
        panic!("not implemented yet!");
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
