use json::JsonValue;

/// Metadata about the fractal. This will be included in the tileset
/// when generating 3D Tiles Next for styling and other purposes
#[derive(Clone)]
pub struct FractalMetadata {
    /// unique ID for this fractal, corresponding to the filename
    pub id: String,
    /// Name of the fractal for display purposes
    pub name: String,
    /// Description of the fractal. Defaults to empty string
    pub description: String,
    /// The number of iterations
    pub iterations: u64,
    /// How many copies of the initial set
    pub cluster_copies: u16,
    /// How many points in the initial set in total
    pub cluster_point_count: u16,
    /// For ManyClusters, what is the maximum number of points in any
    /// sub cluster
    pub subcluster_max_point_count: u16,
    /// How many transformations are in the IFS
    pub ifs_xform_count: u8,
    /// How many transformations are in the color IFS. Default is 1 (identity)
    pub color_ifs_xform_count: u8,
    /// The algorithm that was used
    pub algorithm: String,
    /// how many points are stored in each octree node
    pub node_capacity: u16
}

impl FractalMetadata {
    // Extract the metadata from the JSON parameter file
    pub fn from_json(json: &JsonValue) -> Self {
        let id = &json["id"]
            .as_str().expect("id must be a string");
        let name = &json["name"]
            .as_str().expect("name must be a string");
        let description = &json["description"]
            .as_str().unwrap_or("");
        let algorithm = &json["algorithm"]
            .as_str().expect("algorithm must be a string");
        let iterations = &json["iters"]
            .as_u64().expect("iters must be a number");
        let cluster_copies = &json["cluster_copies"]
            .as_u16().unwrap_or(1);

        let plotter = &json["plotter"];
        let node_capacity = &plotter["node_capacity"].as_u16().unwrap_or(5000);

        let ifs = &json["ifs"];
        let ifs_xform_count = &ifs["xforms"].len();
        let color_ifs = &json["color_ifs"];
        let color_ifs_xform_count = match color_ifs {
            // if not present, the identity IFS is used, which has 1 xform
            JsonValue::Null => 1,
            JsonValue::Object(_) => color_ifs["xforms"].len(),
            _ => panic!("color_ifs must be an object")
        };

        Self {
            id: id.to_string(),
            name: name.to_string(),
            description: description.to_string(),
            iterations: *iterations,
            cluster_copies: *cluster_copies,
            // these point counts will be determined later
            cluster_point_count: 0,
            subcluster_max_point_count: 0,
            ifs_xform_count: *ifs_xform_count as u8,
            color_ifs_xform_count: color_ifs_xform_count as u8,
            algorithm: algorithm.to_string(),
            node_capacity: *node_capacity,
        }
    }

    // Generate the metadata for 3DTILES_metadata
    pub fn make_extension_json(&self) -> JsonValue {
        object!{
            "schema" => object!{
                "classes" => object!{
                    "tileset" => object!{
                        "properties" => object!{
                            "id" => object!{
                                "componentType" => "STRING",
                                "semantic" => "ID"
                            },
                            "name" => object!{
                                "componentType" => "STRING",
                                "semantic" => "NAME"
                            },
                            "description" => object!{
                                "componentType" => "STRING",
                                "semantic" => "DESCRIPTION"
                            },
                            "iterations" => object!{
                                "componentType" => "UINT64"
                            },
                            "cluster_point_count" => object!{
                                "componentType" => "UINT16"
                            },
                            "cluster_copies" => object!{
                                "componentType" => "UINT16"
                            },
                            "ifs_xform_count" => object!{
                                "componentType" => "UINT8"
                            },
                            "color_ifs_xform_count" => object!{
                                "componentType" => "UINT8"
                            },
                            "algorithm" => object!{
                                "componentType" => "STRING"
                            },
                            "node_capacity" => object!{
                                "componentType" => "UINT16"
                            }
                        }
                    }
                }
            },
            "tileset" => object!{
                "class" => "tileset",
                "properties" => object!{
                    "id" => self.id.clone(),
                    "name" => self.name.clone(),
                    "description" => self.description.clone(),
                    "iterations" => self.iterations,
                    "cluster_point_count" => self.cluster_point_count,
                    "cluster_copies" => self.cluster_copies,
                    "ifs_xform_count" => self.ifs_xform_count,
                    "color_ifs_xform_count" => self.color_ifs_xform_count,
                    "algorithm" => self.algorithm.clone(),
                    "node_capacity" => self.node_capacity,
                }
            }
        }
    }
}