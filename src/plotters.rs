use std::fs::{File, create_dir_all};
use std::io::prelude::*;

use json::JsonValue;

use crate::vector::Vec3;
use crate::octrees::OctNode;
use crate::buffers::InternalBuffer;

/// Octree-based point cloud plotter. There are different types for raw
/// scatter plots and density plots
pub trait Plotter {
    /// Plot a single point
    fn plot_point(&mut self, point: Vec3, color: Vec3);

    /// Plot many points from a buffer. This is just an iteration of
    /// plot_point().
    fn plot_buffer(&mut self, buffer: &InternalBuffer) {
        for (point, color) in buffer.points_iter() {
            self.plot_point(point.to_vec3(), color.to_vec3());
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
        const SCALE: f32 = 10000000.0;
        let max_depth = json["max_depth"]
            .as_u8()
            .expect("max_depth must be a positive integer");
        let capacity = json["node_capacity"]
            .as_usize()
            .expect("node_capacity must be a positive integer");
        let radius = json["radius"]
            .as_f32()
            .expect("radius must be a float");

        Self::new(SCALE * radius, capacity, max_depth)
    }

    to_box!(Plotter);

    fn make_tileset_json(&self, dirname: &str) {
        println!("{}", dirname);
        let prefix = "0";
        let root_tile = Self::make_tileset_json_recursive(&self.root, &prefix);
        const SCALE: f32 = 10000000.0;
        let tileset = object!{
            "asset" => object!{
                "version" => "1.0",
            },
            "geometricError" => 5.0 * SCALE,
            "root" => root_tile
        };

        let fname = format!("{}/tileset.json", dirname);
        let mut file = File::create(fname)
            .expect("failed to open tileset.json");
        file.write_all(json::stringify(tileset).as_bytes())
            .expect("failed to write tileset.json");
    }

    fn make_tileset_json_recursive(tree: &OctNode, prefix: &str) -> JsonValue {
        if tree.is_leaf() && tree.is_empty() {
            JsonValue::Null
        } else if tree.is_leaf() { 
            let fname = format!("{}.pnts", prefix);
            object!{
                "boundingVolume" => tree.bounding_volume_json(),
                "geometricError" => 0.0,
                "refine" => "ADD",
                "content" => object!{
                    "uri" => fname
                }
            }
        } else {
            let mut children: Vec<JsonValue> = Vec::new();
            for (quadrant, child) in tree.labeled_children().iter() {
                let new_prefix = format!("{}/{}", prefix, quadrant);
                let child_json = 
                    Self::make_tileset_json_recursive(child, &new_prefix);
                if child_json.is_object() {
                    children.push(child_json);
                }
            }

            object!{
                "boundingVolume" => tree.bounding_volume_json(),
                "geometricError" => tree.geometric_error(),
                "refine" => "ADD",
                "children" => JsonValue::Array(children)
            }
        }
    }
    
    fn make_pnts_files(&self, dirname: &str) {
        let prefix = format!("{}/0", dirname);
        Self::make_pnts_files_recursive(&self.root, &prefix);
    }

    fn make_pnts_files_recursive(tree: &OctNode, prefix: &str) {
        if tree.is_leaf() && tree.is_empty() {
            // If the leaf is empty, no need to generate a file
            return;
        } else if tree.is_leaf() { 
            let fname = format!("{}.pnts", prefix);
            tree.write_pnts(&fname)
        } else {
            let error_msg = format!("could not create directory {}", prefix);
            create_dir_all(prefix).expect(&error_msg);
            for (quadrant, child) in tree.labeled_children().iter() {
                let new_prefix = format!("{}/{}", prefix, quadrant);
                Self::make_pnts_files_recursive(child, &new_prefix);
            }
        }
    }
}

impl Plotter for ScatterPlot {
    fn plot_point(&mut self, point: Vec3, color: Vec3) {
        self.root.add_point(point, color, self.max_depth);
    }

    fn save(&mut self, dirname: &str) {
        create_dir_all(dirname).expect("could not create tileset directory");
        self.make_tileset_json(dirname);
        self.make_pnts_files(dirname);
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
