use std::fs::File;
use std::io::prelude::*;

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

/// Size of a vec3
const SIZE_VEC3: u32 = 3 * 4;
/// Size of an RGB color encoded as unsigned bytes
const SIZE_COLOR_RGB: u32 = 3;
// Size of an unsigned 64-bit integer in bytes
const SIZE_U64: u32 = 8;
// Size of an unsigned 16-bit integer in bytes
const SIZE_U16: u32 = 2;
// Size of an unsigned 8-bit integer in bytes
const SIZE_U8: u32 = 1;

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
    /// The offset within the buffer that marks the start of the buffer view
    byte_offset: u32,
    /// The length of the buffer view
    byte_length: u32,
    /// How many additional bytes of padding are needed to meet alignment
    /// requirements
    padding_length: u32,
}

impl BufferView {
    /// Create an empty buffer view
    pub fn new() -> Self {
        Self {
            byte_offset: 0,
            byte_length: 0,
            padding_length: 0
        }
    }

    // Compute the offset after this buffer view and after the pad
    pub fn after_offset(&self) -> u32 {
        self.byte_offset + self.byte_length + self.padding_length
    }
}

/// A struct for keeping track of glTF accessor settings
struct Accessor {
    /// The index of the buffer view that stores this accessor. This program
    /// uses one buffer view per accessor. 
    buffer_view: u32,
    /// The type of this glTF accessor (e.g. )
    component_type: u32,
    /// minimum value of each component of the accessor. Only required for
    /// position
    min: Option<Vec<f32>>,
    /// minimum value of each component of the accessor. Only required for
    /// position
    max: Option<Vec<f32>>,
}

impl Accessor {
    /// Create an empty accessor
    pub fn new() -> Self {
        Self {
            buffer_view: 0,
            component_type: 0,
            min: None,
            max: None
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
    /// The POSITION accessor
    accessor_position: Accessor,
    /// The COLOR_0 accessor
    accessor_color: Accessor,
    /// The FEATURE_ID_0 accessor
    accessor_feature_ids: Accessor,
    /// The bufferView layout for the POSITION attribute
    buffer_view_position: BufferView,
    /// The bufferView layout for the COLOR_0 attribute
    buffer_view_color: BufferView,
    /// The bufferView layout for the FEATURE_ID_0 attribute
    buffer_view_feature_ids: BufferView,
    /// The bufferView layout for the "iterations" metadata property
    buffer_view_iterations: BufferView,
    /// The bufferView layout for the "point_id" metadata property
    buffer_view_point_id: BufferView,
    /// The bufferView layout for the "last_xforms" metadata property
    buffer_view_last_xforms: BufferView,
    /// The bufferView layout for the "color_xforms" metadata property
    buffer_view_last_color_xforms: BufferView,
    /// The glTF JSON that goes into the JSON chunk
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
            accessor_position: Accessor::new(),
            accessor_color: Accessor::new(),
            accessor_feature_ids: Accessor::new(),
            buffer_view_position: BufferView::new(),
            buffer_view_color: BufferView::new(),
            buffer_view_feature_ids: BufferView::new(),
            buffer_view_iterations: BufferView::new(),
            buffer_view_point_id: BufferView::new(),
            buffer_view_last_xforms: BufferView::new(),
            buffer_view_last_color_xforms: BufferView::new(),
            json: String::new(),
        }
    }

    /// Write a list of points to disk in GLB format
    pub fn write(&mut self, fname: &str, buffer: &Vec<OutputPoint>) {
        self.compute_layout(&buffer);
        self.make_json();

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

        // vec3 position
        let position_length = point_count * SIZE_VEC3;
        self.buffer_view_position.byte_offset = 0;
        self.buffer_view_position.byte_length = position_length;
        self.buffer_view_position.padding_length = compute_padding_length(
            position_length, ALIGNMENT);

        self.accessor_position.buffer_view = 0;
        self.accessor_position.component_type = GLTF_FLOAT;

        // min/max is required for positions
        let (min, max) = compute_min_max(buffer);
        self.accessor_position.min = Some(min);
        self.accessor_position.max = Some(max);

        // vec3 color stored as 3 x UNSIGNED_BYTE + 1 byte padding
        let color_length = point_count * (SIZE_COLOR_RGB + 1);
        self.buffer_view_color.byte_offset = self.buffer_view_position.after_offset();
        self.buffer_view_color.byte_length = color_length;
        self.buffer_view_color.padding_length = compute_padding_length(
            color_length, ALIGNMENT);

        self.accessor_color.buffer_view = 1;
        self.accessor_color.component_type = GLTF_UNSIGNED_BYTE;
        
        // u16 feature ID + 2 byte padding
        let feature_id_length = point_count * (SIZE_U16 + 2);
        self.buffer_view_feature_ids.byte_offset = self.buffer_view_color.after_offset();
        self.buffer_view_feature_ids.byte_length = feature_id_length;
        self.buffer_view_feature_ids.padding_length = compute_padding_length(
            feature_id_length, ALIGNMENT);

        self.accessor_feature_ids.buffer_view = 2;
        self.accessor_feature_ids.component_type = GLTF_UNSIGNED_BYTE;

        // metadata buffer views
        // u64 iteration count
        let iterations_length = point_count * SIZE_U64;
        self.buffer_view_iterations.byte_offset = self.buffer_view_feature_ids.after_offset();
        self.buffer_view_iterations.byte_length = iterations_length;
        // 8-byte values will always be aligned so padding = default of 0

        // u16 point ID
        let ids_length = point_count * SIZE_U16;
        self.buffer_view_point_id.byte_offset = self.buffer_view_iterations.after_offset();
        self.buffer_view_point_id.byte_length = ids_length;
        self.buffer_view_point_id.padding_length = compute_padding_length(
            ids_length, ALIGNMENT);

        // u8 ID of the last transform applied
        let last_xforms_length = point_count * SIZE_U8;
        self.buffer_view_last_xforms.byte_offset = self.buffer_view_point_id.after_offset();
        self.buffer_view_last_xforms.byte_length = last_xforms_length;
        self.buffer_view_last_xforms.padding_length = compute_padding_length(
            last_xforms_length, ALIGNMENT);

        // u8 ID of the last color transform applied
        let last_color_xforms_length = point_count * SIZE_U8;
        self.buffer_view_last_color_xforms.byte_offset = self.buffer_view_last_xforms.after_offset();
        self.buffer_view_last_color_xforms.byte_length = last_color_xforms_length;
        self.buffer_view_last_color_xforms.padding_length = compute_padding_length(
            last_color_xforms_length, ALIGNMENT);
        
        // The offset after the last buffer view is equal to the length of
        // the entire buffer.
        let buffer_length = self.buffer_view_last_color_xforms.after_offset();
        self.binary_chunk.chunk_length = buffer_length;
        // Since the buffer views are already padded, no extra padding is needed
        self.binary_chunk.padding_length = 0;

        self.buffer_length = buffer_length;
    }

    /// Create the glTF JSON for the JSON chunk
    fn make_json(&mut self) {
        let json = object!{
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
                                        "description" => "Iteration number when this point is plotted. Note that the first few iterations are not plotted, so iterations 0-9 will not appear.",
                                        "componentType" => "UINT64",
                                        "required" => true,
                                    },
                                    "point_id" => object!{
                                        "description" => "ID for the point within the initial set. If the basic scatterplot is used, this will always be 0. When algorithm chaos_sets is used, this will be the ID within the initial set, while FEATURE_ID_0 will contain the id of the initial set copy",
                                        "componentType" => "UINT16",
                                        "required" => true,
                                    },
                                    "last_xform" => object!{
                                        "componentType" => "UINT8",
                                        "required" => true,
                                    },
                                    "last_color_xform" => object!{
                                        "componentType" => "UINT8",
                                        "required" => true
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
                                    "bufferView" => 3
                                },
                                "point_id" => object!{
                                    "bufferView" => 4
                                },
                                "last_xform" => object!{
                                    "bufferView" => 5
                                },
                                "last_color_xform" => object!{
                                    "bufferView" => 6
                                }
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
                            "attributes" => object!{
                                "POSITION" => 0,
                                "COLOR_0" => 1,
                                "FEATURE_ID_0" => 2
                            },
                            "mode" => GLTF_POINTS,
                            "extensions" => object!{
                                "EXT_mesh_features" => object!{
                                    "propertyTables" => array![0],
                                    "featureIds" => array![
                                        object!{
                                            "offset" => 0,
                                            "repeat" => 1
                                        },
                                        object!{
                                            "attribute" => 0
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
                    "type" => "VEC3",
                    "componentType" => self.accessor_position.component_type
                },
                object!{
                    "name" => "Colors",
                    "count" => self.point_count,
                    "bufferView" => self.accessor_color.buffer_view,
                    "type" => "VEC3",
                    "normalized" => true,
                    "componentType" => self.accessor_color.component_type,
                },
                object!{
                    "name" => "Feature IDs",
                    "count" => self.point_count,
                    "bufferView" => self.accessor_feature_ids.buffer_view,
                    "type" => "SCALAR",
                    "componentType" => self.accessor_feature_ids.component_type
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
                    "byteStride" => 4
                },
                object!{
                    "name" => "Feature IDs",
                    "buffer" => 0,
                    "byteLength" => self.buffer_view_feature_ids.byte_length,
                    "byteOffset" => self.buffer_view_feature_ids.byte_offset,
                    "byteStride" => 4
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
                },
                object!{
                    "name" => "Last color xform ID",
                    "buffer" => 0,
                    "byteLength" => self.buffer_view_last_color_xforms.byte_length,
                    "byteOffset" => self.buffer_view_last_color_xforms.byte_offset,
                }
            ],
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
        let mut feature_ids: Vec<u8> = Vec::new();
        let mut iterations: Vec<u8> = Vec::new();
        let mut point_ids: Vec<u8> = Vec::new();
        let mut last_xforms: Vec<u8> = Vec::new();
        let mut last_color_xforms: Vec<u8> = Vec::new();
        let mut padding;

        for point in buffer {
            positions.extend_from_slice(&point.position.pack());

            // align to 4-byte-boundaries
            colors.extend_from_slice(&point.color.to_color().pack());
            colors.push(0x00);

            // align to 4-byte boundaries
            //feature_ids.extend_from_slice(&point.feature_id.to_le_bytes());
            feature_ids.push(0x00);
            feature_ids.push(0x00);
            feature_ids.push(0x00);
            feature_ids.push(0x00);

            iterations.extend_from_slice(&point.iteration.to_le_bytes());
            point_ids.extend_from_slice(&point.point_id.to_le_bytes());
            last_xforms.push(point.last_xform);
            last_color_xforms.push(point.last_color_xform);
        }

        // Positions
        padding = make_padding(
            self.buffer_view_position.padding_length, PADDING_BINARY);
        
        file.write_all(&positions).expect("Could not write positions");
        file.write_all(&padding).expect("Could not write position padding");

        // Colors
        padding = make_padding(
            self.buffer_view_color.padding_length, PADDING_BINARY);

        file.write_all(&colors).expect("Could not write colors");
        file.write_all(&padding).expect("Could not write color padding");
        
        // Feature IDs
        padding = make_padding(
            self.buffer_view_feature_ids.padding_length, PADDING_BINARY);

        file.write_all(&feature_ids).expect("Could not write feature IDs");
        file.write_all(&padding).expect("Could not write feature ID padding");

        // Iteration numbers
        padding = make_padding(
            self.buffer_view_iterations.padding_length, PADDING_BINARY);

        file.write_all(&iterations).expect("Could not write iterations");
        file.write_all(&padding).expect("Could not write iterations padding");

        // Point IDs within an initial set
        padding = make_padding(
            self.buffer_view_point_id.padding_length, PADDING_BINARY);

        file.write_all(&point_ids).expect("Could not write point IDs");
        file.write_all(&padding).expect("Could not write point ID padding");

        // Transformation indices for position IFS
        padding = make_padding(
            self.buffer_view_last_xforms.padding_length, PADDING_BINARY);

        file.write_all(&last_xforms).expect("Could not write last xforms");
        file.write_all(&padding).expect("Could not write last xform padding");

        // Transformation indices for color IFS
        padding = make_padding(
            self.buffer_view_last_color_xforms.padding_length, PADDING_BINARY);

        file.write_all(&last_color_xforms).expect("Could not write last xforms");
        file.write_all(&padding).expect("Could not write last xform padding");
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