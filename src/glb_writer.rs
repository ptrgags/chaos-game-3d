use std::fs::File;
use std::io::prelude::*;

use json::JsonValue;

use crate::buffers::OutputBuffer;
use crate::vector::Vec3;

const GLTF_VERSION: u32 = 2;
const GLTF_POINTS: u32 = 0;
const GLTF_FLOAT: u32 = 5126;

struct BufferView {
    byte_offset: u32,
    byte_length: u32
}

impl BufferView {
    pub fn new() -> Self {
        Self {
            byte_offset: 0,
            byte_length: 0
        }
    }
}

struct Accessor<'a> {
    buffer_view: u32,
    accessor_type: &'a str,
    component_type: u32,
    /// minimum value of each component of the accessor. Only required for
    /// position
    min: Option<Vec<f32>>,
    /// minimum value of each component of the accessor. Only required for
    /// position
    max: Option<Vec<f32>>,
}

impl<'a> Accessor<'a> {
    pub fn new() -> Self {
        Self {
            buffer_view: 0,
            component_type: 0,
            accessor_type: "unknown",
            min: None,
            max: None
        }
    }
}

const SIZE_VEC3: u32 = 3 * 4;

pub struct GlbWriter<'a> {
    /// Number of points in this glTF point cloud
    point_count: u32,
    /// Total length of binary glTF, including header and all chunks
    total_length: u32,
    json_chunk_length: u32,
    json_padding_length: u32,
    binary_chunk_length: u32,
    binary_padding_length: u32,
    buffer_length: u32,
    accessor_position: Accessor<'a>,
    accessor_color: Accessor<'a>,
    buffer_view_position: BufferView,
    buffer_view_color: BufferView,
    buffer_view_iterations: BufferView,
    buffer_view_point_id: BufferView,
    buffer_view_last_xforms: BufferView,
    buffer: Vec<u8>,
}

impl<'a> GlbWriter<'a> {
    pub fn new() -> Self {
        Self {
            point_count: 0,
            total_length: 0,
            json_chunk_length: 0,
            json_padding_length: 0,
            binary_chunk_length: 0,
            binary_padding_length: 0,
            buffer_length: 0,
            accessor_position: Accessor::new(),
            accessor_color: Accessor::new(),
            buffer_view_position: BufferView::new(),
            buffer_view_color: BufferView::new(),
            buffer_view_iterations: BufferView::new(),
            buffer_view_point_id: BufferView::new(),
            buffer_view_last_xforms: BufferView::new(),
            buffer: vec![]
        }
    }

    pub fn write(&mut self, fname: &str, buffer: &OutputBuffer) {
        self.compute_layout(&buffer);
        let json = json::stringify(self.make_json());

        let error_msg = format!("Cannot open {}", fname);
        let mut file = File::create(fname).expect(&error_msg);
        self.write_header(&mut file);
        self.write_json_chunk(&mut file, &json);
        self.write_binary_chunk(&mut file, &self.buffer)
    }

    fn compute_layout(&mut self, buffer: &OutputBuffer) {
        let point_count = buffer.len() as u32;
        self.point_count = point_count;

        let position_length = point_count * SIZE_VEC3;
        self.buffer_view_position.byte_offset = 0;
        self.buffer_view_position.byte_length = position_length;
        self.accessor_position.buffer_view = 0;
        self.accessor_position.accessor_type = "VEC3";
        self.accessor_position.component_type = GLTF_FLOAT;

        let (min, max) = compute_min_max(buffer.get_points());
        self.accessor_position.min = Some(min);
        self.accessor_position.max = Some(max);
    }

    fn make_json(&self) -> JsonValue {
        object!{
            "asset" => object!{
                "version" => "2.0"
            },
            "extensionsUsed" => array![
                "EXT_mesh_features"
            ],
            "extensions" => object!{
                "EXT_mesh_features" => object!{
                    "schema" => object!{
                        "classes" => object!{
                            "fractal" => object!{
                                "name" => "Fractal",
                                "description" => "Details about the fractal",
                                "properties" => object!{
                                    "iterations" => object!{
                                        "componentType" => "UINT64",
                                        "required" => true,
                                    },
                                    "point_id" => object!{
                                        "componentType" => "UINT16",
                                        "required" => true,
                                    },
                                    "last_xform" => object!{
                                        "componentType" => "UINT8",
                                        "required" => true,
                                    }
                                }
                            }
                        }
                    },
                    "propertyTables" => array![
                        object!{
                            "name" => "fractal",
                            "class" => "fractal",
                            "count" => self.point_count,
                            "properties" => object!{
                                "iterations" => object!{
                                    "bufferView" => 2
                                },
                                "point_id" => object!{
                                    "bufferView" => 3
                                },
                                "last_xform" => object!{
                                    "bufferView" => 4
                                },
                            }
                        }
                    ]
                }
            },
            "scene" => 0,
            "scenes" => array![
                object!{
                    "nodes" => array![0]
                }
            ],
            "nodes" => array![
                object!{
                    "mesh" => 0
                }
            ],
            "meshes" => array![
                object!{
                    "primitives" => array![
                        object!{
                            "attributes" => object!{
                                "POSITION" => 0,
                                "COLOR" => 1
                            },
                            "mode" => GLTF_POINTS,
                            "extensions" => object!{
                                "EXT_mesh_features" => object!{
                                    "propertyTables" => array![0],
                                    "featureIds" => array![
                                        object!{
                                            "offset" => 0,
                                            "repeat" => 1
                                        }
                                    ]
                                }
                            }
                        }
                    ]
                }
            ],
            "accessors" => array![
                object!{
                    "name" => "Positions",
                    "count" => self.point_count,
                    "bufferView" => self.accessor_position.buffer_view,
                    "min" => self.accessor_position.min.as_ref().unwrap().clone(),
                    "max" => self.accessor_position.max.as_ref().unwrap().clone(),
                    "type" => self.accessor_position.accessor_type,
                    "component_type" => self.accessor_position.component_type
                },
                object!{
                    "name" => "Colors",
                    "count" => self.point_count,
                    "bufferView" => self.accessor_color.buffer_view,
                    "type" => self.accessor_color.accessor_type,
                    "component_type" => self.accessor_color.component_type
                }
            ],
            "bufferViews" => array![
                object!{
                    "name" => "Positions",
                    "buffer" => 0,
                    "byteLength" => self.buffer_view_position.byte_length,
                    "byteOffset" => self.buffer_view_position.byte_offset,
                },
                object!{
                    "name" => "Colors",
                    "buffer" => 0,
                    "byteLength" => self.buffer_view_color.byte_length,
                    "byteOffset" => self.buffer_view_color.byte_offset,
                },
                object!{
                    "name" => "Iteration Count",
                    "buffer" => 0,
                    "byteLength" => self.buffer_view_iterations.byte_length,
                    "byteOffset" => self.buffer_view_iterations.byte_offset,
                },
                object!{
                    "name" => "Point ID",
                    "buffer" => 0,
                    "byteLength" => self.buffer_view_point_id.byte_length,
                    "byteOffset" => self.buffer_view_point_id.byte_offset,
                },
                object!{
                    "name" => "Last xform ID",
                    "buffer" => 0,
                    "byteLength" => self.buffer_view_last_xforms.byte_length,
                    "byteOffset" => self.buffer_view_last_xforms.byte_offset,
                }
            ],
            "buffers" => array![
                object!{
                    "byteLength" => self.buffer_length
                }
            ]
        }
    }

    fn write_header(&self, file: &mut File) {
        let error_msg = "could not write glb header";
        file.write_all(b"glTF").expect(error_msg);
        file.write_all(&GLTF_VERSION.to_le_bytes()).expect(error_msg);
        file.write_all(&self.total_length.to_le_bytes()).expect(error_msg);
    }

    fn write_json_chunk(&self, file: &mut File, json: &String) {
        let error_msg = "could not write JSON chunk";
        file.write_all(&self.json_chunk_length.to_le_bytes());
        file.write_all(b"BIN\0").expect(error_msg);
        file.write_all(&json.as_bytes()).expect(error_msg);
        let padding = Self::make_padding(self.json_padding_length, 0x00);
        file.write_all(&padding).expect(error_msg);
    }

    fn write_binary_chunk(&self, file: &mut File, buffer: &Vec<u8>) {
        let error_msg = "could not write binary chunk";
        file.write_all(&self.binary_chunk_length.to_le_bytes());
        file.write_all(b"BIN\0").expect(error_msg);
        file.write_all(&buffer).expect(error_msg);
        let padding = Self::make_padding(self.binary_padding_length, 0x00);
        file.write_all(&padding).expect(error_msg);
    }

    /// Create a padding of space charcters of a given length
    fn make_padding(byte_len: u32, pad_char: u8) -> Vec<u8> {
        // 0x20 is the space character
        (0..byte_len).map(|_| pad_char).collect()
    }
}

fn compute_min_max(positions: &Vec<Vec3>) -> (Vec<f32>, Vec<f32>) {
    let mut min = [f32::MAX; 3];
    let mut max = [f32::MIN; 3];
    for position in positions.iter() {
        let x = *position.x();
        let y = *position.y();
        let z = *position.z();
        min[0] = min[0].min(x);
        min[1] = min[1].min(y);
        min[2] = min[2].min(z);
        max[0] = max[0].max(x);
        max[1] = max[1].max(y);
        max[2] = max[2].max(z);
    }

    (min.to_vec(), max.to_vec())
}