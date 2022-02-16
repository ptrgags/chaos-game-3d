use std::fs::{File, create_dir_all, remove_dir_all};
use std::io::prelude::*;
use std::path::Path;

use json::JsonValue;

use crate::fractal_metadata::FractalMetadata;
use crate::octrees::OctNode;
use crate::pnts_writer::PntsWriter;
use crate::glb_writer::GlbWriter;

/// Type of 3D Tiles content
#[derive(Clone, PartialEq)]
pub enum ContentType {
    /// .pnts: 3D Tiles 1.0 Point Cloud
    Pnts,
    /// .glb: Binary glTF. For 3D Tiles, this requires 3DTILES_content_gltf
    Glb,
}

impl ContentType {
    // Get the file extension for this content
    pub fn get_extension(&self) -> &str {
        match self {
            Self::Pnts => "pnts",
            Self::Glb => "glb"
        }
    }
}

/// An object that can generate a 3D Tileset
pub struct TilesetWriter {
    /// The type of content to store in each tile
    content_type: ContentType,
    /// Metadata to include in the tileset when using 3D Tiles Next 
    /// (.glb content)
    metadata: FractalMetadata
}

impl TilesetWriter {
    pub fn new(content_type: ContentType, metadata: FractalMetadata) -> Self {
        Self {
            content_type,
            metadata
        }
    }

    /// Save the tileset to disk in the specified directory. The directory will
    /// be removed first if it exists, so use with care!
    pub fn save(&self, dirname: &str, root: &OctNode) {
        if Path::new(dirname).exists() {
            remove_dir_all(dirname)
                .expect("could not remove old tileset directory");
        }
        create_dir_all(dirname).expect("could not create tileset directory");
        println!("Generating tileset JSON...");
        self.make_tileset_json(root, dirname);
        println!("Generating point cloud files...");
        self.make_contents(root, dirname);
    }

    /// Generate a tileset.json file by traversing the tree and collecting
    /// data
    ///
    /// See https://github.com/CesiumGS/3d-tiles/tree/master/specification#reference-tileset
    fn make_tileset_json(&self, root: &OctNode, dirname: &str) {
        let prefix = "0";
        let root_tile = self.make_tileset_json_recursive(root, &prefix);
        let mut tileset = object!{
            "asset" => object!{
                "version" => "1.0",
            },
            "geometricError" => 1e7,
            "root" => root_tile,
            "extensionsUsed" => array!["3DTILES_metadata"],
            "extensions" => object!{
                "3DTILES_metadata" => self.metadata.make_extension_json()
            }
        };

        // If using GLB output, we also need to add the
        // 3DTILES_content_gltf extension
        if self.content_type == ContentType::Glb {
            tileset["extensionsRequired"] = array!["3DTILES_content_gltf"];
            tileset["extensionsUsed"].push("3DTILES_content_gltf").unwrap();
            tileset["extensions"]["3DTILES_content_gltf"] = object!{
                "extensionsUsed" => array!["EXT_mesh_features"],
            };
        }

        let fname = format!("{}/tileset.json", dirname);
        let mut file = File::create(fname)
            .expect("failed to open tileset.json");
        file.write_all(json::stringify(tileset).as_bytes())
            .expect("failed to write tileset.json");
    }

    /// Generate the tree of tiles including URIs to each .pnts file
    ///
    /// See https://github.com/CesiumGS/3d-tiles/tree/master/specification#reference-tile
    fn make_tileset_json_recursive(&self, tree: &OctNode, dirname: &str)
            -> JsonValue {
        if tree.is_leaf() && tree.is_empty() {
            JsonValue::Null
        } else if tree.is_leaf() {
            // TODO: Fix this
            let fname = format!(
                "{}.{}", dirname, self.content_type.get_extension());
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
            for (octant, child) in tree.labeled_children().iter() {
                let new_prefix = format!("{}/{}", dirname, octant);
                let child_json = 
                    self.make_tileset_json_recursive(child, &new_prefix);
                if child_json.is_object() {
                    children.push(child_json);
                }
            }

            let fname = format!(
                "{}.{}", dirname, self.content_type.get_extension());
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

    /// Traverse the tree, generating content files at leaves and directories
    /// at interior nodes.
    fn make_contents(&self, tree: &OctNode, dirname: &str) {
        if tree.is_leaf() {
            self.make_content(tree, dirname);
        } else {
            for child in tree.get_children().iter() {
                self.make_contents(child, dirname);
            }
            self.make_content(tree, dirname);
        }
    }

    // Generate a 3D model for a tile content.
    fn make_content(&self, tree: &OctNode, dirname: &str) {
        // No need to create an empty point cloud
        if tree.is_empty() {
            return;
        }

        let points = tree.get_points();
        let directory = tree.get_directory_name(dirname);
        create_dir_all(directory).expect("could not create directory");

        match self.content_type {
            ContentType::Pnts => {
                let mut writer = PntsWriter::new();
                let fname = tree.get_file_name(dirname, "pnts");
                writer.write(&fname, points);
            },
            ContentType::Glb => {
                let mut writer = GlbWriter::new();
                let fname = tree.get_file_name(dirname, "glb");
                writer.write(&fname, points);
            }
        }
    }
}