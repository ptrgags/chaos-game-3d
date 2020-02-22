use crate::buffers::OutputBuffer;
use std::fs::File;
use std::io::prelude::*;

/// Cesium Point Cloud format version 1.0
const PNTS_VERSION: u32 = 1;

/// The .pnts format uses single-precision floats for compactness
const FLOAT_SIZE: u32 = 4;

/// A position is a triple of floats, x, y, z
const POSITION_SIZE: u32 = 3 * FLOAT_SIZE;

/// Colors are represented with 3 bytes, r, g, b
const COLOR_SIZE: u32 = 3;

/// Each section of the .pnts file must be alligned to 8-byte boundaries
const ALIGNMENT: u32 = 8;

/// The header is always 28 bytes
const HEADER_LENGTH: u32 = 28;

/// This application doesn't need the batch table
const BATCH_TABLE_JSON_LENGTH: u32 = 0;
const BATCH_TABLE_BINARY_LENGTH: u32 = 0;
const BATCH_TABLE_LENGTH: u32 = 
    BATCH_TABLE_JSON_LENGTH + BATCH_TABLE_BINARY_LENGTH;

/// Custom writer for a Cesium 3D Tiles .pnts file that records points and
/// colors to a binary file. see
/// https://github.com/AnalyticalGraphicsInc/3d-tiles/tree/master/specification/TileFormats/PointCloud
/// for more information.
///
/// Note that these fields are u32 and not usize, this is for compliance with
/// the spec.
pub struct PntsWriter {
    /// String representation of the feature table JSON
    feature_table_json: String,
    /// How much padding to align the feature table JSON portion with the
    /// 8-byte boundary
    json_padding_len: u32,
    /// How much padding to align the binary feature table portion with the
    /// 8-byte boundary
    bin_padding_len: u32,
    /// JSON length + padding
    total_json_len: u32,
    /// Binary length + padding
    total_bin_len: u32,
    /// Total length of the .pnts file
    total_len: u32
}

impl PntsWriter {
    pub fn new() -> Self {
        Self {
            feature_table_json: String::new(),
            json_padding_len: 0,
            bin_padding_len: 0,
            total_json_len: 0,
            total_bin_len: 0,
            total_len: 0
        }
    }

    /// Write all the points in a buffer to a .pnts file of the given filename
    pub fn write(&mut self, fname: &str, buffer: &OutputBuffer) {
        let error_msg = format!("Cannot open {}", fname);
        let mut file = File::create(fname).expect(&error_msg);
        self.prepare_header(buffer);
        self.write_header(&mut file);
        self.write_body(&mut file, buffer);
    }

    /// Create the feature table JSON and compute lengths of parts of the
    /// binary file. This updates the lengths stored in the struct for later
    fn prepare_header(&mut self, buffer: &OutputBuffer) {
        // The .pnts format stores  each feature contiguously. If the
        // the positions go first, we need to compute where the colors go
        let num_positions = buffer.len() as u32;
        let positions_length = num_positions * POSITION_SIZE;
        let rgb_offset = positions_length;

        // Create the feature table in JSON format, and serialize it to
        // a byte slice. This will be the returned value
        let feature_table = object!{
            "POINTS_LENGTH" => num_positions,
            "POSITION" => object!{
                "byteOffset" => 0
            },
            "RGB" => object!{
                "byteOffset" => rgb_offset
            }
        };

        // Serialize to bytes, this will be the return value
        self.feature_table_json = json::stringify(feature_table);
        let feature_table_json_bytes: &[u8] = 
            self.feature_table_json.as_bytes();

        // However, while we have other size values on hand, let's compute
        // some of the lengths we need to compute elsewhere in the header
        let feature_table_json_length = feature_table_json_bytes.len() as u32;

        // Align the json to an 8-byte boundary
        let header_and_json_len = HEADER_LENGTH + feature_table_json_length;
        self.json_padding_len = Self::compute_padding_len(header_and_json_len);

        // Align the binary portion to an 8-byte boundary
        let colors_length = num_positions * COLOR_SIZE;
        let feature_table_binary_length = positions_length + colors_length;
        self.bin_padding_len = 
            Self::compute_padding_len(feature_table_binary_length);

        // Total length is header + feature table + empty batch table
        self.total_json_len = feature_table_json_length + self.json_padding_len;
        self.total_bin_len = feature_table_binary_length + self.bin_padding_len;
        self.total_len = 
            HEADER_LENGTH 
            + self.total_json_len
            + self.total_bin_len
            + BATCH_TABLE_LENGTH;
    }


    /// Write the header portion of the .pnts file
    fn write_header(&self, file: &mut File) {
        let error_msg = "could not write pnts header";

        // magic number
        file.write_all(b"pnts").expect(error_msg);

        // .pnts version number
        file.write_all(&PNTS_VERSION.to_le_bytes()).expect(error_msg);

        // Total length of file
        file.write_all(&self.total_len.to_le_bytes()).expect(error_msg);  

        // JSON length including padding
        file.write_all(&self.total_json_len.to_le_bytes())
            .expect(error_msg);

        // Feature table binary length
        file.write_all(&self.total_bin_len.to_le_bytes())
            .expect(error_msg);

        // We don't need to use the batch table feature so set both
        // json/binary length to 0
        file.write_all(&BATCH_TABLE_JSON_LENGTH.to_le_bytes()).expect(error_msg);
        file.write_all(&BATCH_TABLE_BINARY_LENGTH.to_le_bytes())
            .expect(error_msg);

        // Feature table JSON. technically it's part of the body but it
        // feels like header info.
        let json_bytes = self.feature_table_json.as_bytes();
        file.write_all(json_bytes).expect(error_msg);

        // Padding for the feature table json length
        let padding = Self::make_padding(self.json_padding_len);
        file.write_all(&padding).expect(error_msg);
    }

    /// Write the body portion of the .pnts file, a packed list of points.
    fn write_body(&self, file: &mut File, buffer: &OutputBuffer) {
        let mut positions: Vec<u8> = Vec::new();
        let mut colors: Vec<u8> = Vec::new();

        for (point, color) in buffer.points_iter() {
            let point_bytes: [u8; 12] = point.pack();
            positions.extend_from_slice(&point_bytes);

            let color_bytes: [u8; 3] = color.to_color().pack();
            colors.extend_from_slice(&color_bytes);
        }

        let error_msg = "Could not write pnts file body";
        file.write_all(&positions).expect(error_msg);
        file.write_all(&colors).expect(error_msg);

        let padding = Self::make_padding(self.bin_padding_len);
        file.write_all(&padding).expect(error_msg);
    }

    /// Given a number of bytes, compute how much padding is needed to
    /// align to 8 bytes
    fn compute_padding_len(num_bytes: u32) -> u32 {
        let remainder = num_bytes % ALIGNMENT;
        (ALIGNMENT - remainder) % ALIGNMENT
    }

    /// Create a padding of space charcters of a given length
    fn make_padding(byte_len: u32) -> Vec<u8> {
        // 0x20 is the space character
        (0..byte_len).map(|_| 0x20u8).collect()
    }
}
