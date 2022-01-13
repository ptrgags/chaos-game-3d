use std::fs::{File, create_dir_all, remove_dir_all};
use std::io::prelude::*;
use std::path::Path;

use json::JsonValue;

use crate::octrees::OctNode;
use crate::pnts_writer::PntsWriter;
use crate::glb_writer::GlbWriter;

#[derive(Clone, PartialEq)]
pub enum TileType {
    Pnts,
    Glb,
}

impl TileType {
    pub fn get_extension(&self) -> &str {
        match self {
            Self::Pnts => "pnts",
            Self::Glb => "glb"
        }
    }
}

pub struct TilesetWriter {
    tile_type: TileType
}

impl TilesetWriter {
    pub fn new(tile_type: TileType) -> Self {
        Self {
            tile_type
        }
    }

    pub fn save(&self, dirname: &str, root: &OctNode) {
        if Path::new(dirname).exists() {
            remove_dir_all(dirname)
                .expect("could not remove old tileset directory");
        }
        create_dir_all(dirname).expect("could not create tileset directory");
        println!("Generating tileset JSON...");
        self.make_tileset_json(dirname, root);
        println!("Generating .pnts files...");
        self.make_contents(dirname, root);
    }

    /// Generate a tileset.json file by traversing the tree and collecting
    /// data
    ///
    /// See https://github.com/CesiumGS/3d-tiles/tree/master/specification#reference-tileset
    fn make_tileset_json(&self, dirname: &str, root: &OctNode) {
        let prefix = "0";
        let root_tile = self.make_tileset_json_recursive(root, &prefix);
        let tileset = object!{
            "asset" => object!{
                "version" => "1.0",
            },
            "geometricError" => 1e7,
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
    fn make_tileset_json_recursive(&self, tree: &OctNode, prefix: &str)
            -> JsonValue {
        if tree.is_leaf() && tree.is_empty() {
            JsonValue::Null
        } else if tree.is_leaf() { 
            let fname = format!(
                "{}.{}", prefix, self.tile_type.get_extension());
            object!{
                "boundingVolume" => tree.bounding_volume_json(),
                "geometricError" => 0.0,
                "refine" => "REPLACE",
                "content" => object!{
                    "uri" => fname
                }
            }
        } else {
            let mut children: Vec<JsonValue> = Vec::new();
            for (quadrant, child) in tree.labeled_children().iter() {
                let new_prefix = format!("{}/{}", prefix, quadrant);
                let child_json = 
                    self.make_tileset_json_recursive(child, &new_prefix);
                if child_json.is_object() {
                    children.push(child_json);
                }
            }

            let fname = format!(
                "{}.{}",prefix, self.tile_type.get_extension());
            object!{
                "boundingVolume" => tree.bounding_volume_json(),
                "geometricError" => tree.geometric_error(),
                "refine" => "REPLACE",
                "children" => JsonValue::Array(children),
                "content" => object!{
                    "uri" => fname
                }
            }
        }
    }

    /// Generate the contnet files, one per tile that contains data
    fn make_contents(&self, dirname: &str, root: &OctNode) {
        let prefix = format!("{}/0", dirname);
        self.make_contents_recursive(root, &prefix);
    }

    /// Traverse the tree, generating content files at leaves and directories
    /// at interior nodes.
    fn make_contents_recursive(&self, tree: &OctNode, prefix: &str) {
        if tree.is_leaf() {
            self.make_content(tree, prefix);
        } else {
            let error_msg = format!("could not create directory {}", prefix);
            create_dir_all(prefix).expect(&error_msg);
            for (quadrant, child) in tree.labeled_children().iter() {
                let new_prefix = format!("{}/{}", prefix, quadrant);
                self.make_contents_recursive(child, &new_prefix);
            }

            self.make_content(tree, prefix);
        }
    }

    // Generate a 3D model for a tile content.
    fn make_content(&self, tree: &OctNode, prefix: &str) {
        // No need to create an empty point cloud
        if tree.is_empty() {
            return;
        }

        let points = tree.get_points();
        match self.tile_type {
            TileType::Pnts => {
                let mut writer = PntsWriter::new();
                let fname = format!("{}.pnts", prefix);
                writer.write(&fname, points);
            },
            TileType::Glb => {
                let mut writer = GlbWriter::new();
                let fname = format!("{}.glb", prefix);
                writer.write(&fname, points);
            }
        }
    }
}