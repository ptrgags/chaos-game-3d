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
    /// Root of the underlying octree
    root: OctNode,
    /// Maximum depth of the tree, beyond which points are discarded
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
            .expect("max_depth must be a positive integer");
        let capacity = json["node_capacity"]
            .as_usize()
            .expect("node_capacity must be a positive integer");
        let radius = json["radius"]
            .as_f32()
            .expect("radius must be a float");

        Self::new(radius, capacity, max_depth)
    }

    to_box!(Plotter);

    /// Generate a tileset.json file by traversing the tree and collecting
    /// metadata.
    ///
    /// See https://github.com/CesiumGS/3d-tiles/tree/master/specification#reference-tileset
    fn make_tileset_json(&self, dirname: &str) {
        let prefix = "0";
        let root_tile = Self::make_tileset_json_recursive(&self.root, &prefix);
        let tileset = object!{
            "asset" => object!{
                "version" => "1.0",
            },
            "geometricError" => 5.0,
            "root" => root_tile
        };

        let fname = format!("{}/tileset.json", dirname);
        let mut file = File::create(fname)
            .expect("failed to open tileset.json");
        file.write_all(json::stringify(tileset).as_bytes())
            .expect("failed to write tileset.json");
    }

    /// Generate the tree of tiles including URIs to each .pnts file
    ///
    /// See https://github.com/CesiumGS/3d-tiles/tree/master/specification#reference-tile
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
    
    /// Generate the .pnts files, one for each tile that contains data.
    fn make_pnts_files(&self, dirname: &str) {
        let prefix = format!("{}/0", dirname);
        Self::make_pnts_files_recursive(&self.root, &prefix);
    }

    /// Traverse the tree, generating .pnts files at leaves and directories
    /// at interior nodes.
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

    /// Save the tileset into a directory of the given name. This creates
    /// the directory if it does not already exist
    fn save(&mut self, dirname: &str) {
        create_dir_all(dirname).expect("could not create tileset directory");
        self.make_tileset_json(dirname);
        self.make_pnts_files(dirname);
    }
}

/// Allow for different calculations of density plots
pub enum DensityType {
    /// Basic density: points at a node / total points
    Density,
    /// Like basic density, but take a logarithm of the result
    LogDensity,
}

/// Density plot: instead of storing every point, use the octree as a voxel
/// grid and count up points that land in each bucket. When it comes time
/// to generate a point cloud, the counts will be divided by the total number
/// of points to compute the density. One point will be generated per voxel
/// at the center of the cell.
pub struct DensityPlot {
    /// Support both density and log density plots to allow experimentation
    density_type: DensityType,
    /// Root of the underlying octree
    root: OctNode,
    /// Depth of the tree. The grid will have a resolution of 2^depth in all
    /// three directions for a grid of 8^depth voxels
    depth: u8,
    /// The number of points that were successfully plotted in the octree
    /// (out-of-bounds values are ignored). Storing this in 64-bits since
    /// 4 billion iterations won't be enough for me ;)
    total_points: u64,
}

impl DensityPlot {
    pub fn new(density_type: DensityType, radius: f32, depth: u8) -> Self {
        Self {
            density_type,
            root: OctNode::root_node(radius, 1),
            depth,
            total_points: 0,
        }
    }

    /// Load a plotter from JSON of the form:
    /// {
    ///     "type": "density",
    ///     "density_type": "density" | "log_density",
    ///     "depth": d,
    ///     "radius": r,
    /// }
    pub fn from_json(json: &JsonValue) -> Self {
        let depth = json["depth"]
            .as_u8()
            .expect("max_depth must be a positive integer");

        let density_type_str = json["density"]
            .as_str()
            .expect("density_type must be a string");

        let density_type = match density_type_str {
            "density" => DensityType::Density,
            "log_density" => DensityType::LogDensity,
            _ => panic!("density_type must be either density or log_density")
        };

        let radius = json["radius"]
            .as_f32()
            .expect("radius must be a float");

        Self::new(density_type, radius, depth)
    }

    to_box!(Plotter);

    fn compute_densities(&mut self) {
        //TODO
        /*
        Self::compute_densities_recursive(self.root, self.total_points); 
        */
    }

    fn compute_densities_recursive(tree: &OctNode) {
        // TODO
        
    }

    /// Generate a tileset.json file by traversing the tree and collecting
    /// metadata. This must be called after self.compute_densities() so the
    /// points will be generated.
    ///
    /// See https://github.com/CesiumGS/3d-tiles/tree/master/specification#reference-tileset
    fn make_tileset_json(&self, dirname: &str) {
        // TODO
        /*
        let prefix = "0";
        let root_tile = Self::make_tileset_json_recursive(&self.root, &prefix);
        let tileset = object!{
            "asset" => object!{
                "version" => "1.0",
            },
            "geometricError" => 5.0,
            "root" => root_tile
        };

        let fname = format!("{}/tileset.json", dirname);
        let mut file = File::create(fname)
            .expect("failed to open tileset.json");
        file.write_all(json::stringify(tileset).as_bytes())
            .expect("failed to write tileset.json");
            */
    }

    fn make_pnts_files(&self, dirname: &str) {
        /*
        let prefix = format!("{}/0", dirname);
        Self::make_pnts_files_recursive(&self.root, &prefix);
        */
    }

}

impl Plotter for DensityPlot {
    fn plot_point(&mut self, point: Vec3, color: Vec3) {
        self.total_points += 
            self.root.count_point(point, color, self.depth) as u64;
    }

    /// Save the tileset into a directory of the given name. This creates
    /// the directory if it does not already exist
    fn save(&mut self, dirname: &str) {
        create_dir_all(dirname).expect("could not create tileset directory");
        self.compute_densities();
        self.make_tileset_json(dirname);
        self.make_pnts_files(dirname);
    }
}

/// Parse a point cloud plotter from a JSON object of the form:
///
/// ```text
/// {
///     "type": "scatter" | "density",
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
        "density" => DensityPlot::from_json(&json).to_box(),
        _ => panic!("Plotter type must be one of, {:?}", valid_plotters)
    }
}
