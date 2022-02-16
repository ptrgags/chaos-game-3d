use std::fs::File;
use std::io::prelude::*;
use std::collections::HashSet;

use json::JsonValue;

use crate::point::OutputPoint;

/// glTF version number. 2.0 is the latest as of this writing.
const GLTF_VERSION: u32 = 2;
/// glTF mode for point clouds
const GLTF_POINTS: u32 = 0;
/// glTF constant for FLOAT component type
const GLTF_FLOAT: u32 = 5126;
/// glTF constant for UNSIGNED_BYTE component type
const GLTF_UNSIGNED_BYTE: u32 = 5121;
/// Length of the glTF header
const GLTF_HEADER_LENGTH: u32 = 12;
/// Length of a chunk header (length + type)
const GLTF_CHUNK_HEADER_LENGTH: u32 = 8;

/// Size of a single-precision float
const SIZE_FLOAT: u32 = 4;
/// Size of a vec3
const SIZE_VEC3: u32 = 3 * SIZE_FLOAT;
/// Size of an RGB color encoded as unsigned bytes
const SIZE_COLOR_RGB: u32 = 3;

/// Alignment for each buffer view. I'm setting them all to 8
/// as this is the simplest way to satisfy both glTF and EXT_mesh_features
/// requirements without making the code complex
const ALIGNMENT: u32 = 8;

/// Padding character for the BIN chunk
const PADDING_BINARY: u8 = 0x00;
/// Padding character for the JSON chunk
const PADDING_JSON: u8 = ' ' as u8;

/// A struct for keeping track of the size of a buffer view within the buffer 
struct BufferView {
    /// Human-readable name for the buffer view
    pub name: String,
    /// The index of the bufer view
    pub id: u32,
    /// The offset within the buffer that marks the start of the buffer view
    pub byte_offset: u32,
    /// The length of the buffer view
    pub byte_length: u32,
    /// For buffer views that are not aligned, 
    pub byte_stride: Option<u32>,
    /// How many additional bytes of padding are needed to meet alignment
    /// requirements
    pub padding_length: u32,
}

impl BufferView {
    /// Create an empty buffer view
    pub fn new(name: &str, id: u32, byte_offset: u32, byte_length: u32)
            -> Self {
        Self {
            name: String::from(name),
            id,
            byte_offset,
            byte_length,
            byte_stride: None,
            padding_length: compute_padding_length(byte_length, ALIGNMENT)
        }
    }

    // Compute the offset after this buffer view and after the pad
    pub fn after_offset(&self) -> u32 {
        self.byte_offset + self.byte_length + self.padding_length
    }

    pub fn to_json(&self) -> JsonValue {
        let mut result = object!{
            "name" => self.name.clone(),
            "buffer" => 0,
            "byteOffset" => self.byte_offset,
            "byteLength" => self.byte_length,
        };

        if let Some(stride) = self.byte_stride {
            result["byteStride"] = JsonValue::Number(stride.into());
        }

        result
    }
}

/// A struct for keeping track of glTF accessor settings
struct Accessor {
    /// Attribute semantic since every accesor will also be a vertex
    /// attribute
    semantic: String,
    /// ID of this accessor.
    accessor_id: u32,
    /// JSON description of the accesor
    json: JsonValue
}

impl Accessor {
    /// Create an empty accessor
    pub fn new(semantic: &str, accessor_id: u32, json: JsonValue) -> Self {
        Self {
            semantic: String::from(semantic),
            accessor_id,
            json
        }
    }
}

/// One of the two chunks in the GLB file (JSON and BIN)
struct Chunk {
    /// The length of the chunk data, not including header or padding
    chunk_length: u32,
    /// How many bytes of padding are needed to meet alignment requirements
    padding_length: u32
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            chunk_length: 0,
            padding_length: 0,
        }
    }

    /// Get the length of data + padding
    pub fn data_length(&self) -> u32 {
        self.chunk_length + self.padding_length
    }

    /// Get the length of header, data and padding
    pub fn total_length(&self) -> u32 {
        GLTF_CHUNK_HEADER_LENGTH + self.chunk_length + self.padding_length
    }
}

/// An object that can write a point cloud as a binary GLTF (GLB) file.
pub struct GlbWriter {
    /// Number of points in this glTF point cloud
    point_count: u32,
    /// Total length of binary glTF, including header and all chunks
    total_length: u32,
    /// The layout of the JSON chunk
    json_chunk: Chunk,
    /// The layout of the BIN chunk
    binary_chunk: Chunk,
    /// The total length of the buffer in the BIN chunk
    buffer_length: u32,
    /// The glTF accessors.
    accessors: Vec<Accessor>,
    /// The glTF buffer views.
    buffer_views: Vec<BufferView>,
    // The final JSON string that will be written to the JSON chunk
    json: String,
}

impl GlbWriter {
    pub fn new() -> Self {
        Self {
            point_count: 0,
            total_length: 0,
            json_chunk: Chunk::new(),
            binary_chunk: Chunk::new(),
            buffer_length: 0,
            accessors: Vec::new(),
            buffer_views: Vec::new(),
            json: String::new(),
        }
    }

    /// Write a list of points to disk in GLB format
    pub fn write(&mut self, fname: &str, buffer: &Vec<OutputPoint>) {
        self.compute_layout(&buffer);
        self.make_json(&buffer);

        // Now that we have both chunks, update the total length
        self.total_length = 
            GLTF_HEADER_LENGTH + 
            self.json_chunk.total_length() + 
            self.binary_chunk.total_length();

        let error_msg = format!("Cannot open {}", fname);
        let mut file = File::create(fname).expect(&error_msg);
        self.write_header(&mut file);
        self.write_json_chunk(&mut file, &self.json);
        self.write_binary_chunk(&mut file, buffer);
    }

    /// Compute the layout of the .glb file (byte offsets and lengths)
    fn compute_layout(&mut self, buffer: &Vec<OutputPoint>) {
        let point_count = buffer.len() as u32;
        self.point_count = point_count;

        // min/max is required for positions
        let (min, max) = compute_min_max(buffer);

        // vec3 POSITION -------------------------------------------------
        let position_length = point_count * SIZE_VEC3;
        let position_bv = BufferView::new(
            "Positions", 
            self.buffer_views.len() as u32,
            0,
            position_length
        );

        // Get the id of this buffer view (for use in the accessor)
        // and the offset of the next buffer view so the buffer view struct
        // can be pushed to the vec.
        let mut bv_id = position_bv.id;
        let mut next_bv_offset = position_bv.after_offset();
        self.buffer_views.push(position_bv);

        let position_accessor = Accessor::new(
            "POSITION",
            self.accessors.len() as u32,
            object!{
                "name" => "Positions",
                "bufferView" => bv_id,
                "count" => point_count,
                "min" => min,
                "max" => max,
                "type" => "VEC3",
                "componentType" => GLTF_FLOAT
            }
        );
        self.accessors.push(position_accessor);

        // vec3 COLOR_0 -------------------------------------------------
        // stored as 3 x UNSIGNED_BYTE + 1 byte padding
        let color_length = point_count * (SIZE_COLOR_RGB + 1);
        let mut color_bv = BufferView::new(
            "Colors",
            self.buffer_views.len() as u32,
            next_bv_offset,
            color_length
        );
        // a normalized vec3 is only 3 bytes, so set the stride
        color_bv.byte_stride = Some(SIZE_COLOR_RGB + 1);
        bv_id = color_bv.id;
        next_bv_offset = color_bv.after_offset();
        self.buffer_views.push(color_bv);

        let color_accessor = Accessor::new(
            "COLOR_0",
            self.accessors.len() as u32,
            object! {
                "name" => "Colors",
                "bufferView" => bv_id,
                "count" => point_count,
                "type" => "VEC3",
                "componentType" => GLTF_UNSIGNED_BYTE,
                "normalized" => true,
            }
        );
        self.accessors.push(color_accessor);
        
        // vec3 _CLUSTER_COORDINATES (aka uvw coordinates) ------------------
        let uvw_length = point_count * SIZE_VEC3;
        let uvw_bv = BufferView::new(
            "Cluster Coordinates",
            self.buffer_views.len() as u32,
            next_bv_offset,
            uvw_length
        );
        bv_id = uvw_bv.id;
        next_bv_offset = uvw_bv.after_offset();
        self.buffer_views.push(uvw_bv);

        let uvw_accessor = Accessor::new(
            "_CLUSTER_COORDINATES",
            self.accessors.len() as u32,
            object!{
                "name" => "Cluster Coordinates",
                "bufferView" => bv_id,
                "count" => point_count,
                "type" => "VEC3",
                "componentType" => GLTF_FLOAT
            }
        );
        self.accessors.push(uvw_accessor);

        // float _FEATURE_ID_0 (iterations) ----------------------------------
        let iteration_length = point_count * SIZE_FLOAT;
        let iteration_bv = BufferView::new(
            "Feature ID 0 (iterations)",
            self.buffer_views.len() as u32,
            next_bv_offset,
            iteration_length
        );
        bv_id = iteration_bv.id;
        next_bv_offset = iteration_bv.after_offset();
        self.buffer_views.push(iteration_bv);

        let iteration_accessor = Accessor::new(
            "_FEATURE_ID_0",
            self.accessors.len() as u32,
            object!{
                "name" => "Feature ID 0 (iterations)",
                "bufferView" => bv_id,
                "count" => point_count,
                "type" => "SCALAR",
                "componentType" => GLTF_FLOAT
            }
        );
        self.accessors.push(iteration_accessor);

        // float _FEATURE_ID_1 (cluster_copy) --------------------------------
        let cluster_copy_length = point_count * SIZE_FLOAT;
        let cluster_copy_bv = BufferView::new(
            "Feature ID 1 (cluster copy)",
            self.buffer_views.len() as u32,
            next_bv_offset,
            cluster_copy_length
        );
        bv_id = cluster_copy_bv.id;
        next_bv_offset = cluster_copy_bv.after_offset();
        self.buffer_views.push(cluster_copy_bv);

        let cluster_copy_accessor = Accessor::new(
            "_FEATURE_ID_1",
            self.accessors.len() as u32,
            object!{
                "name" => "Feature ID 1 (cluster copy)",
                "bufferView" => bv_id,
                "count" => point_count,
                "type" => "SCALAR",
                "componentType" => GLTF_FLOAT
            }
        );
        self.accessors.push(cluster_copy_accessor);

        // float _FEATURE_ID_2 (cluster id) --------------------------------
        let cluster_id_length = point_count * SIZE_FLOAT;
        let cluster_id_bv = BufferView::new(
            "Feature ID 2 (cluster id)",
            self.buffer_views.len() as u32,
            next_bv_offset,
            cluster_id_length
        );
        bv_id = cluster_id_bv.id;
        next_bv_offset = cluster_id_bv.after_offset();
        self.buffer_views.push(cluster_id_bv);

        let cluster_id_accessor = Accessor::new(
            "_FEATURE_ID_2",
            self.accessors.len() as u32,
            object!{
                "name" => "Feature ID 2 (cluster id)",
                "bufferView" => bv_id,
                "count" => point_count,
                "type" => "SCALAR",
                "componentType" => GLTF_FLOAT
            }
        );
        self.accessors.push(cluster_id_accessor);

        // float _FEATURE_ID_3 (point id) ----------------------------------
        let point_id_length = point_count * SIZE_FLOAT;
        let point_id_bv = BufferView::new(
            "Feature ID 3 (point id)",
            self.buffer_views.len() as u32,
            next_bv_offset,
            point_id_length
        );
        bv_id = point_id_bv.id;
        next_bv_offset = point_id_bv.after_offset();
        self.buffer_views.push(point_id_bv);

        let point_id_accessor = Accessor::new(
            "_FEATURE_ID_3",
            self.accessors.len() as u32,
            object!{
                "name" => "Feature ID 3 (point id)",
                "bufferView" => bv_id,
                "count" => point_count,
                "type" => "SCALAR",
                "componentType" => GLTF_FLOAT
            }
        );
        self.accessors.push(point_id_accessor);

        // float _LAST_XFORM ----------------------------------------------
        let last_xform_length = point_count * SIZE_FLOAT;
        let last_xform_bv = BufferView::new(
            "Last xform applied",
            self.buffer_views.len() as u32,
            next_bv_offset,
            last_xform_length
        );
        bv_id = last_xform_bv.id;
        next_bv_offset = last_xform_bv.after_offset();
        self.buffer_views.push(last_xform_bv);

        let last_xform_accessor = Accessor::new(
            "_LAST_XFORM",
            self.accessors.len() as u32,
            object!{
                "name" => "Last xform applied",
                "bufferView" => bv_id,
                "count" => point_count,
                "type" => "SCALAR",
                "componentType" => GLTF_FLOAT
            }
        );
        self.accessors.push(last_xform_accessor);

        // float _LAST_COLOR_XFORM ----------------------------------------
        let last_color_xform_length = point_count * SIZE_FLOAT;
        let last_color_xform_bv = BufferView::new(
            "Last xform applied",
            self.buffer_views.len() as u32,
            next_bv_offset,
            last_color_xform_length
        );
        bv_id = last_color_xform_bv.id;
        next_bv_offset = last_color_xform_bv.after_offset();
        self.buffer_views.push(last_color_xform_bv);

        let last_color_xform_accessor = Accessor::new(
            "_LAST_COLOR_XFORM",
            self.accessors.len() as u32,
            object!{
                "name" => "Last color xform applied",
                "bufferView" => bv_id,
                "count" => point_count,
                "type" => "SCALAR",
                "componentType" => GLTF_FLOAT
            }
        );
        self.accessors.push(last_color_xform_accessor);

        // binary chunk layout ---------------------------------------------

        // The offset after the last buffer view is equal to the length of
        // the entire buffer.
        let buffer_length = next_bv_offset;
        self.binary_chunk.chunk_length = buffer_length;
        // Since the buffer views are already padded, no extra padding is needed
        self.binary_chunk.padding_length = 0;

        self.buffer_length = buffer_length;
    }

    /// Create the glTF JSON for the JSON chunk
    fn make_json(&mut self, buffer: &Vec<OutputPoint>) {
        let accessors: Vec<JsonValue> = 
            self.accessors.iter().map(|x| x.json.clone()).collect();
        let buffer_views: Vec<JsonValue> =
            self.buffer_views.iter().map(|x| x.to_json()).collect();

        let mut attributes = object!{};
        for accessor in self.accessors.iter() {
            attributes[&accessor.semantic] = 
                JsonValue::Number(accessor.accessor_id.into());
        }

        let feature_id_json = self.compute_feature_id_json(&buffer);

        let json = object!{
            "asset" => object!{
                "version" => "2.0"
            },
            "extensions" => object!{
                "EXT_structural_metadata" => object!{
                    "schema" => object!{
                        "classes" => object!{
                            "fractal" => object!{
                                "name" => "Fractal",
                                "description" => "Per-point fractal properties",
                                "properties" => object!{
                                    "cluster_coordinates" => object!{
                                        "type" => "VEC3",
                                        "componentType" => "FLOAT32"
                                    },
                                    "iteration" => object!{
                                        "type" => "SCALAR",
                                        "componentType" => "FLOAT32"
                                    },
                                    "cluster_copy" => object!{
                                        "type" => "SCALAR",
                                        "componentType" => "FLOAT32"
                                    },
                                    "cluster_id" => object!{
                                        "type" => "SCALAR",
                                        "componentType" => "FLOAT32"
                                    },
                                    "point_id" => object!{
                                        "type" => "SCALAR",
                                        "componentType" => "FLOAT32"
                                    },
                                    "last_xform" => object!{
                                        "type" => "SCALAR",
                                        "componentType" => "FLOAT32"
                                    },
                                    "last_color_xform" => object!{
                                        "type" => "SCALAR",
                                        "componentType" => "FLOAT32"
                                    }
                                }
                            }
                        }
                    },
                    "propertyMappings" => object!{
                        "class" => "fractal",
                        "properties" => object!{
                            "cluster_coordinates" => object!{
                                "attribute" => "_CLUSTER_COORDINATES"                                
                            },
                            "iteration" => object!{
                                // There's nothing in the spec that says I 
                                // can't do this ;)
                                "attribute" => "_FEATURE_ID_0"
                            },
                            "cluster_copy" => object!{
                                "attribute" => "_FEATURE_ID_1"
                            },
                            "cluster_id" => object!{
                                "attribute" => "_FEATURE_ID_2"
                            },
                            "point_id" => object!{
                                "attribute" => "_FEATURE_ID_3"
                            },
                            "last_xform" => object!{
                                "attribute" => "_LAST_XFORM"
                            },
                            "last_color_xform" => object!{
                                "attribute" => "_LAST_COLOR_XFORM"
                            }
                        }
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
                    "mesh" => 0,
                    "matrix" => array![
                        1, 0, 0, 0,
                        0, 0, -1, 0,
                        0, 1, 0, 0,
                        0, 0, 0, 1
                    ]
                }
            ],
            "meshes" => array![
                object!{
                    "primitives" => array![
                        object!{
                            "attributes" => attributes.clone(),
                            "mode" => GLTF_POINTS,
                            "extensions" => object!{
                                "EXT_mesh_features" => object!{
                                    "featureIds" => feature_id_json.clone(),
                                },
                                "EXT_structural_metadata" => object!{
                                    "propertyMappings" => array![0]
                                }
                            }
                        }
                    ]
                }
            ],
            "accessors" => accessors,
            "bufferViews" => buffer_views,
            "buffers" => array![
                object!{
                    "byteLength" => self.buffer_length
                }
            ]
        };

        let json_str = json::stringify(json);
        let length = json_str.as_bytes().len() as u32;
        self.json = json_str;
        self.json_chunk.chunk_length = length;
        self.json_chunk.padding_length = compute_padding_length(length, ALIGNMENT);
    }

    fn compute_feature_id_json(&self, buffer: &Vec<OutputPoint>) -> JsonValue {
        // Count the number of unique IDs for each of the feature Ids
        // since featureCount is required.
        let mut iterations: HashSet<u64> = HashSet::new();
        let mut cluster_copies: HashSet<u16> = HashSet::new();
        let mut cluster_ids: HashSet<u16> = HashSet::new();
        let mut point_id: HashSet<u16> = HashSet::new();
        for point in buffer.iter() {
            iterations.insert(point.iteration);
            cluster_copies.insert(point.cluster_copy);
            cluster_ids.insert(point.cluster_id);
            point_id.insert(point.point_id);
        }

        object!{
            "iterations" => object!{
                "featureCount" => iterations.len(),
                "attribute" => 0
            },
            "cluster_copy" => object!{
                "featureCount" => cluster_copies.len(),
                "attribute" => 1
            },
            "cluster_id" => object!{
                "featureCount" => cluster_ids.len(),
                "attribute" => 2
            },
            "point_id" => object!{
                "featureCount" => point_id.len(),
                "attribute" => 3
            }
        }
    }

    /// Write the GLB header to the file
    fn write_header(&self, file: &mut File) {
        let error_msg = "could not write glb header";
        file.write_all(b"glTF").expect(error_msg);
        file.write_all(&GLTF_VERSION.to_le_bytes()).expect(error_msg);
        file.write_all(&self.total_length.to_le_bytes()).expect(error_msg);
    }

    /// Write the JSON chunk to the file
    fn write_json_chunk(&self, file: &mut File, json: &str) {
        let error_msg = "could not write JSON chunk";
        file.write_all(&self.json_chunk.data_length().to_le_bytes()).expect(error_msg);
        file.write_all(b"JSON").expect(error_msg);
        file.write_all(&json.as_bytes()).expect(error_msg);
        let padding = make_padding(self.json_chunk.padding_length, PADDING_JSON);
        file.write_all(&padding).expect(error_msg);
    }

    /// Write the binary chunk to the file
    fn write_binary_chunk(&self, file: &mut File, buffer: &Vec<OutputPoint>) {
        let error_msg = "could not write binary chunk";
        file.write_all(&self.binary_chunk.data_length().to_le_bytes()).expect(error_msg);
        file.write_all(b"BIN\0").expect(error_msg);
        self.write_buffer(file, buffer);
    }

    /// Write the binary buffer from a list of points
    fn write_buffer(&self, file: &mut File, buffer: &Vec<OutputPoint>) {
        let mut positions: Vec<u8> = Vec::new();
        let mut colors: Vec<u8> = Vec::new();
        let mut cluster_coordinates: Vec<u8> = Vec::new();
        let mut iterations: Vec<u8> = Vec::new();
        let mut cluster_copies: Vec<u8> = Vec::new();
        let mut cluster_ids: Vec<u8> = Vec::new();
        let mut point_ids: Vec<u8> = Vec::new();
        let mut last_xforms: Vec<u8> = Vec::new();
        let mut last_color_xforms: Vec<u8> = Vec::new();
        

        for point in buffer {
            positions.extend_from_slice(&point.position.pack());

            // colors is a normalized vec3. Since each one is only 3 bytes,
            // we need to add a fourth byte of padding.
            colors.extend_from_slice(&point.color.to_color().pack());
            colors.push(0x00);

            cluster_coordinates.extend_from_slice(
                &point.cluster_coordinates.pack());

            // all of the ids, though they are actually integers, are stored
            // in the glTF as FLOAT accessors since this is easiest for use
            // on the GPU.
            let iteration = point.iteration as f32;
            iterations.extend_from_slice(&iteration.to_le_bytes());

            let cluster_copy = point.cluster_copy as f32;
            cluster_copies.extend_from_slice(&cluster_copy.to_le_bytes());

            let cluster_id = point.cluster_id as f32;
            cluster_ids.extend_from_slice(&cluster_id.to_le_bytes());

            let point_id = point.point_id as f32;
            point_ids.extend_from_slice(&point_id.to_le_bytes());

            let last_xform = point.last_xform as f32;
            last_xforms.extend_from_slice(&last_xform.to_le_bytes());

            let last_color_xform = point.last_color_xform as f32;
            last_color_xforms.extend_from_slice(&last_color_xform.to_le_bytes());
        }

        // Make a parallel vector of data to match the buffer views
        let bv_data = vec![
            positions,
            colors,
            cluster_coordinates,
            iterations,
            cluster_copies,
            cluster_ids,
            point_ids,
            last_xforms,
            last_color_xforms,
        ];

        let n = bv_data.len();
        for i in 0..n {
            let buffer_view = &self.buffer_views[i];
            let data = &bv_data[i];
            let padding = 
                make_padding(buffer_view.padding_length, PADDING_BINARY);
            
            let message = format!("Could not write bufferView {}", buffer_view.name);
            file.write_all(&data).expect(&message);
            
            let padding_message = format!("could not write padding for bufferView {}", buffer_view.name);
            file.write_all(&padding).expect(&padding_message);
        }
    }
}

/// Compute the number of padding bytes needed to meet an alignment requirement
fn compute_padding_length(byte_length: u32, alignment_bytes: u32) -> u32 {
    let remainder = byte_length % alignment_bytes;
    if remainder == 0 {
        // if already aligned, don't add padding
        0
    } else {
        alignment_bytes - remainder
    }
}

/// Create a padding of space charcters of a given length
fn make_padding(byte_len: u32, pad_char: u8) -> Vec<u8> {
    // 0x20 is the space character
    (0..byte_len).map(|_| pad_char).collect()
}

/// Iterate over a list of points and compute the min/max position
fn compute_min_max(points: &Vec<OutputPoint>) -> (Vec<f32>, Vec<f32>) {
    let mut min = [f32::MAX; 3];
    let mut max = [f32::MIN; 3];
    for point in points.iter() {
        let position = point.position;
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