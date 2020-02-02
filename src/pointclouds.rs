use std::fs::{File, create_dir_all};
use std::io::prelude::*;

use json;

use crate::buffers::{InternalBuffer, OutputBuffer};

/// A writer takes a point cloud (collection of (position, color))
/// and writes it to a file.
pub trait PointCloudWriter {
    /// Add points from a buffer
    fn add_points(&mut self, other: &mut InternalBuffer);
    /// Write the results to a file
    fn save(&self, fname: &str);
}

/// Output one big CSV file with a list of the points.
pub struct CSVWriter {
    buffer: OutputBuffer
}

impl CSVWriter {
    pub fn new() -> Self {
        CSVWriter { buffer: OutputBuffer::new() }
    }
}

impl PointCloudWriter for CSVWriter {
    fn add_points(&mut self, other: &mut InternalBuffer) { 
        for (point, color) in other.points_iter() {
            self.buffer.add(point.to_vec3(), color.to_vec3())
        }
    }

    fn save(&self, fname: &str) {
        let mut file = File::create(fname).expect("Failed to create CSV file");
        for (point, color) in self.buffer.clone().points_iter() {
            writeln!(
                file, 
                "{},{},{},{},{},{}", 
                point.x(), 
                point.y(), 
                point.z(),
                color.x(),
                color.y(),
                color.z()).expect("could not write CSV line");
        }
    }
}

/// Generate a Cesium 3D tiles point cloud tileset from the points
/// This follows the [official specifications](https://github.com/AnalyticalGraphicsInc/3d-tiles/tree/master/specification)
pub struct Cesium3DTilesWriter {
    buffer: OutputBuffer,
    scale: f32,
}

impl Cesium3DTilesWriter {
    pub fn new(scale:f32) -> Self {
        Self {
            buffer: OutputBuffer::new(),
            scale,
        }
    }

    /// Create the tileset JSON file. This serves as an index of the tiles
    /// in the tileset. the points themselves are stored in separate .pnts
    /// files
    pub fn make_tileset_json(&self, dir_name: &str) {
        let json = object!{
            "asset" => object!{
                "version" => "1.0",
            },
            "geometricError" => self.scale,
            "root" => object!{
                "boundingVolume" => object!{
                   "box" => array![
                       0.0, 0.0, 0.0,
                       self.scale, 0.0, 0.0,
                       0.0, self.scale, 0.0,
                       0.0, 0.0, self.scale
                   ]
                },
                "geometricError" => 0.0,
                "refine" => "ADD",
                "content" => object!{
                    "uri" => "points.pnts"
                }
            },
        };

        let fname = format!("{}/tileset.json", dir_name);
        let mut file = File::create(fname)
            .expect("failed to open tileset.json");
        file.write_all(json::stringify(json).as_bytes())
            .expect("failed to write tileset.json");
    }

    /// Generate a .pnts file to store all the points and color in the tile
    pub fn make_pnts_file(&self, dir_name: &str) { 
        let fname = format!("{}/points.pnts", dir_name);
        let mut file = File::create(fname)
            .expect("failed to open points.pnts");
        let bin_padding_len = self.write_pnts_header(&mut file);
        self.write_pnts_body(&mut file, bin_padding_len);
    }

    /// Write the header portion of the .pnts file
    fn write_pnts_header(&self, file: &mut File) -> u32 {
        let num_positions = self.buffer.len() as u32;

        const FLOAT_SIZE: u32 = 4;
        const POSITION_SIZE: u32 = 3 * FLOAT_SIZE;
        const COLOR_SIZE: u32 = 3;
        let positions_length = num_positions * POSITION_SIZE;
        let colors_length = num_positions * COLOR_SIZE;
        let feature_table_binary_length = positions_length + colors_length;
        let rgb_offset = positions_length;
        let bin_remainder = feature_table_binary_length % 8;
        let bin_padding_len = (8 - bin_remainder) % 8;

        let feature_table = object!{
            "POINTS_LENGTH" => num_positions,
            "POSITION" => object!{
                "byteOffset" => 0
            },
            "RGB" => object!{
                "byteOffset" => rgb_offset
            }
        };
        let feature_table_json = json::stringify(feature_table);
        let feature_table_json_bytes: &[u8] = feature_table_json.as_bytes();
        let feature_table_json_length = feature_table_json_bytes.len() as u32;

        let header_and_json_len = HEADER_LENGTH + feature_table_json_length;
        let remainder = header_and_json_len % 8;
        let json_padding_len = (8 - remainder) % 8;

        // byte length
        const HEADER_LENGTH: u32 = 28;
        const BATCH_TABLE_JSON_LENGTH: u32 = 0;
        const BATCH_TABLE_BINARY_LENGTH: u32 = 0;
        const BATCH_TABLE_LENGTH: u32 = 
            BATCH_TABLE_JSON_LENGTH + BATCH_TABLE_BINARY_LENGTH;

        let message = "error writing pnts header";

        // magic
        file.write_all(b"pnts").expect(message);

        // version
        file.write_all(&1u32.to_le_bytes()).expect(message);

        // Total length
        let total_json_len = feature_table_json_length + json_padding_len;
        let total_bin_len = feature_table_binary_length + bin_padding_len;
        let total_length = 
            HEADER_LENGTH 
            + total_json_len
            + total_bin_len
            + BATCH_TABLE_LENGTH;
        file.write_all(&total_length.to_le_bytes()).expect(message);

        // feature table JSON length
        file.write_all(&total_json_len.to_le_bytes())
            .expect(message);

        // Feature table binary length
        file.write_all(&total_bin_len.to_le_bytes())
            .expect(message);

        // This doesn't use the batch table feature so set both
        // json/binary length to 0
        file.write_all(&BATCH_TABLE_JSON_LENGTH.to_le_bytes()).expect(message);
        file.write_all(&BATCH_TABLE_BINARY_LENGTH.to_le_bytes())
            .expect(message);

        // Feature table JSON. technically it's part of the body but it
        // feels like header info.
        file.write_all(feature_table_json_bytes).expect(message);

        let padding: Vec<u8> = (0..json_padding_len).map(|_| 0x20u8).collect();
        file.write_all(&padding).expect(message);

        bin_padding_len
    }

    /// Write the body portion of the .pnts file, a packed list of points.
    fn write_pnts_body(&self, file: &mut File, bin_padding_len: u32) {
        let mut positions: Vec<u8> = Vec::new();
        let mut colors: Vec<u8> = Vec::new();

        for (point, color) in self.buffer.clone().points_iter() {
            let point_bytes: [u8; 12] = point.scale(self.scale).pack();
            positions.extend_from_slice(&point_bytes);

            let color_bytes: [u8; 3] = color.to_color().pack();
            colors.extend_from_slice(&color_bytes);
        }

        let message = "Could not write pnts file body";
        file.write_all(&positions).expect(message);
        file.write_all(&colors).expect(message);

        let padding: Vec<u8> = (0..bin_padding_len).map(|_| 0x20u8).collect();
        file.write_all(&padding).expect(message);
    }
}

impl PointCloudWriter for Cesium3DTilesWriter {
    fn add_points(&mut self, other: &mut InternalBuffer) { 
        for (point, color) in other.points_iter() {
            self.buffer.add(point.to_vec3(), color.to_vec3())
        }
    }

    fn save(&self, dir_name: &str) {
        create_dir_all(dir_name).expect("could not create tileset dir");
        self.make_tileset_json(dir_name);
        self.make_pnts_file(dir_name);
    }
}
