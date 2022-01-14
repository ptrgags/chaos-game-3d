use json::JsonValue;

#[derive(Clone)]
pub struct FractalMetadata {
    pub iterations: u64,
    pub initial_set_point_count: u16,
    pub initial_set_copies: u16,
    pub ifs_xform_count: u8,
    pub color_ifs_xform_count: u8,
    pub algorithm: String,
    pub node_capacity: u16
}

impl FractalMetadata {
    pub fn from_json(json: &JsonValue) -> Self {
        let algorithm = &json["algorithm"]
            .as_str().expect("algorithm must be a string");
        let iterations = &json["iters"]
            .as_u64().expect("iters must be a number");
        let initial_set_copies = &json["initial_set_copies"]
            .as_u16().unwrap_or(1);

        let initial_set = &json["initial_set"];
        let initial_set_point_count = match initial_set {
            JsonValue::Null => 1,
            JsonValue::Object(_) => 
                initial_set["num_points"]
                    .as_u16().expect("initial_set.num_points must be a number"),
            _ => panic!("")
        };

        let plotter = &json["plotter"];
        let node_capacity = &plotter["node_capacity"]
            .as_u16().expect("plotter.node_capacity must be a number");

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
            iterations: *iterations,
            initial_set_point_count: initial_set_point_count,
            initial_set_copies: *initial_set_copies,
            ifs_xform_count: *ifs_xform_count as u8,
            color_ifs_xform_count: color_ifs_xform_count as u8,
            algorithm: algorithm.to_string(),
            node_capacity: *node_capacity,
        }
    }

    pub fn make_extension_json(&self) -> JsonValue {
        object!{
            "schema" => object!{
                "classes" => object!{
                    "tileset" => object!{
                        "properties" => object!{
                            "iterations" => object!{
                                "componentType" => "UINT64"
                            },
                            "initial_set_point_count" => object!{
                                "componentType" => "UINT16"
                            },
                            "initial_set_copies" => object!{
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
                    "iterations" => self.iterations,
                    "initial_set_point_count" => self.initial_set_point_count,
                    "initial_set_copies" => self.initial_set_copies,
                    "ifs_xform_count" => self.ifs_xform_count,
                    "color_ifs_xform_count" => self.color_ifs_xform_count,
                    "algorithm" => self.algorithm.clone(),
                    "node_capacity" => self.node_capacity,
                }
            }
        }
    }
}