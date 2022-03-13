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
    metadata: FractalMetadata,
    /// The directory where the tileset will go
    /// ./viewer/{tileset_id}
    tileset_dir: String,
    /// The directory where the point files will go, 
    /// ./viewer/{tileset_id}/points
    points_dir: String,
}

impl TilesetWriter {
    pub fn new(
            tileset_id: &str,
            content_type: ContentType,
            metadata: FractalMetadata)
            -> Self {
        Self {
            content_type,
            metadata,
            tileset_dir: format!("./viewer/{}", tileset_id),
            points_dir: format!("./viewer/{}/points", tileset_id)
        }
    }

    /// Save the tileset to disk in the specified directory. The directory will
    /// be removed first if it exists, so use with care!
    pub fn save(&self, root: &OctNode) {
        self.init_directories();

        println!("Generating tileset JSON in {}", &self.tileset_dir);
        self.make_tileset_json(root);

        println!("Generating point cloud files in {}", self.points_dir);
        self.make_contents(root);
    }

    fn init_directories(&self) {
        // Remove the old directory
        if Path::new(&self.tileset_dir).exists() {
            let message = format!(
                "Could not remove old tileset in {}", self.tileset_dir);
            remove_dir_all(&self.tileset_dir).expect(&message);
        }

        create_dir_all(&self.tileset_dir)
            .expect("Could not create tileset directory");
        create_dir_all(&self.points_dir)
            .expect("Could not create points directory");
    }

    /// Generate a tileset.json file by traversing the tree and collecting
    /// data
    ///
    /// See https://github.com/CesiumGS/3d-tiles/tree/master/specification#reference-tileset
    fn make_tileset_json(&self, root: &OctNode) {
        let root_tile = 
            self.make_tileset_json_recursive(root, "points");
        let mut tileset = object!{
            "asset" => object!{
                "version" => "1.0",
            },
            "geometricError" => 1e7,
            "root" => root_tile,
            "extensionsUsed" => array!["3DTILES_metadata"],
            "schema" => self.metadata.make_schema_json(),
            "metadata" => self.metadata.make_metadata_json()
        };

        // If using GLB output, we also need to add the
        // 3DTILES_content_gltf extension
        if self.content_type == ContentType::Glb {
            tileset["extensionsRequired"] = array!["3DTILES_content_gltf"];
            tileset["extensionsUsed"].push("3DTILES_content_gltf").unwrap();
            tileset["extensions"]["3DTILES_content_gltf"] = object!{
                "extensionsUsed" => array![
                    "EXT_mesh_features",
                    "EXT_structural_metadata"
                ],
            };
        }

        let fname = format!("{}/tileset.json", self.tileset_dir);
        let message = format!("Failed to create {}", fname);
        let mut file = File::create(fname.clone()).expect(&message);

        let message = format!("Failed to write {}", fname);
        file.write_all(json::stringify(tileset).as_bytes()).expect(&message);
    }

    /// Generate the tree of tiles including URIs to each .pnts file
    ///
    /// See https://github.com/CesiumGS/3d-tiles/tree/master/specification#reference-tile
    fn make_tileset_json_recursive(&self, tree: &OctNode, content_dirname: &str)
            -> JsonValue {
        if tree.is_leaf() && tree.is_empty() {
            JsonValue::Null
        } else if tree.is_leaf() {
            let fname = tree.get_file_name(
                content_dirname, self.content_type.get_extension());
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
            for child in tree.get_children().iter() {
                let child_json = 
                    self.make_tileset_json_recursive(child, content_dirname);
                if child_json.is_object() {
                    children.push(child_json);
                }
            }

            let fname = tree.get_file_name(
                content_dirname, self.content_type.get_extension());
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
    fn make_contents(&self, tree: &OctNode) {
        if tree.is_leaf() {
            self.make_content(tree);
        } else {
            for child in tree.get_children().iter() {
                self.make_contents(child);
            }
            self.make_content(tree);
        }
    }

    // Generate a 3D model for a tile content.
    fn make_content(&self, tree: &OctNode) {
        // No need to create an empty point cloud
        if tree.is_empty() {
            return;
        }

        let points = tree.get_points();
        let directory = tree.get_directory_name(&self.points_dir);
        create_dir_all(&directory).expect("could not create directory");

        match self.content_type {
            ContentType::Pnts => {
                let mut writer = PntsWriter::new();
                let fname = tree.get_file_name(&self.points_dir, "pnts");
                writer.write(&fname, points);
            },
            ContentType::Glb => {
                let mut writer = GlbWriter::new();
                let fname = tree.get_file_name(&self.points_dir, "glb");
                writer.write(&fname, points);
            }
        }
    }
}