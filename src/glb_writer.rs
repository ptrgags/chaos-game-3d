use std::fs::File;
use std::io::prelude::*;

use json::JsonValue;

use crate::buffers::OutputBuffer;

const GLTF_VERSION: u32 = 2;
const GLTF_POINTS: u32 = 0;

struct BufferView {
    byte_offset: u32,
    byte_length: u32
}

struct Accessor {
    buffer_view: u32,
    component_type: u32,
}

pub struct GlbWriter {
    /// Number of points in this glTF point cloud
    point_count: u32,
    /// Total length of binary glTF, including header and all chunks
    total_length: u32,
    json_chunk_length: u32,
    json_padding_length: u32,
    binary_chunk_length: u32,
    binary_padding_length: u32,
    buffer_length: u32,
    buffer: Vec<u8>,
}

impl GlbWriter {
    pub fn new() -> Self {
        Self {
            point_count: 0,
            total_length: 0,
            json_chunk_length: 0,
            json_padding_length: 0,
            binary_chunk_length: 0,
            binary_padding_length: 0,
            buffer_length: 0,
            buffer: vec![]
        }
    }

    pub fn write(&mut self, fname: &str, buffer: &OutputBuffer) {
        self.compute_lengths(&buffer);
        let json = json::stringify(self.make_json());

        let error_msg = format!("Cannot open {}", fname);
        let mut file = File::create(fname).expect(&error_msg);
        self.write_header(&mut file);
        self.write_json_chunk(&mut file, &json);
        self.write_binary_chunk(&mut file, &self.buffer)
    }

    fn compute_lengths(&mut self, buffer: &OutputBuffer) {
        let point_count = buffer.len() as u32;
        self.point_count = point_count;
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

                        }
                    },
                    "propertyTables" => object!{

                    }
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
                    "bufferView" => 0
                },
                object!{
                    "name" => "Colors",
                    "count" => self.point_count,
                    "bufferView" => 1
                }
            ],
            "bufferViews" => array![
                object!{
                    "name" => "Positions",
                    "buffer" => 0
                },
                object!{
                    "name" => "Colors",
                    "buffer" => 0
                },
                object!{
                    "name" => "Iteration Count",
                    "buffer" => 0
                },
                object!{
                    "name" => "Point ID",
                    "buffer" => 0
                },
                object!{
                    "name" => "Last xform ID",
                    "buffer" => 0
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