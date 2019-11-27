extern crate rand;
#[macro_use]
extern crate json;

use rand::Rng;
use std::fs::File;
use std::io::prelude::*;

type Vector = (f32, f32, f32);
type Color = (u8, u8, u8);
type VectorList = Vec<Vector>;
type ColorList = Vec<Color>;

trait Transform {
    fn transform(&self, vector: &Vector) -> Vector;
}

struct SierpinskiTransform {
    translation: Vector
}

impl SierpinskiTransform {
    pub fn new(translation: Vector) -> SierpinskiTransform {
        SierpinskiTransform { translation }
    }
}

impl Transform for SierpinskiTransform {
    fn transform(&self, vector: &Vector) -> Vector {
        let (x, y, z) = vector;
        let (tx, ty, tz) = self.translation;
        let x2 = 0.5 * x + tx;
        let y2 = 0.5 * y + ty;
        let z2 = 0.5 * z + tz;

        (x2, y2, z2)
    }
}


/*
fn print_point(vector: &Vector, index: u32) {
    const SCALE: f32 = 10000000.0;
    let (x, y, z) = vector;
    let shade = index % 256;
    let shade2 = (index + 64) % 256;
    println!("{},{},{},{},{},{}", x * SCALE, y * SCALE, z * SCALE, 0, shade, shade2);
}
*/

fn write_pnts(fname: &str, positions: &VectorList, colors: &ColorList) 
        -> std::io::Result<()> {

    let num_positions = positions.len() as u32;
    const FLOAT_SIZE: u32 = 4;
    const POSITION_SIZE: u32 = 3 * FLOAT_SIZE;
    const COLOR_SIZE: u32 = 3;
    let positions_length = num_positions * POSITION_SIZE;
    let colors_length = num_positions * COLOR_SIZE;
    let feature_table_binary_length = positions_length + colors_length;
    let rgb_offset = positions_length;

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

    // byte length
    const HEADER_LENGTH: u32 = 28;
    const BATCH_TABLE_JSON_LENGTH: u32 = 0;
    const BATCH_TABLE_BINARY_LENGTH: u32 = 0;
    const BATCH_TABLE_LENGTH: u32 = BATCH_TABLE_JSON_LENGTH + BATCH_TABLE_BINARY_LENGTH;

    let mut file = File::create(fname)?;
    // magic
    file.write_all(b"pnts")?;

    // version
    file.write_all(&1u32.to_le_bytes())?;

    // Total length
    let total_length = HEADER_LENGTH + feature_table_json_length + feature_table_binary_length + BATCH_TABLE_LENGTH;
    file.write_all(&total_length.to_le_bytes())?;

    // feature table JSON length
    file.write_all(&feature_table_json_length.to_le_bytes())?;

    // Feature table binary length
    file.write_all(&feature_table_binary_length.to_le_bytes())?;

    // This doesn't use the batch table feature so set both json/binary length to 0
    file.write_all(&BATCH_TABLE_JSON_LENGTH.to_le_bytes())?;
    file.write_all(&BATCH_TABLE_BINARY_LENGTH.to_le_bytes())?;

    Ok(())
}

fn main() {
    let functions: [SierpinskiTransform; 4] = [
        SierpinskiTransform::new((-0.5, -0.5, 0.0)),
        SierpinskiTransform::new((0.5, -0.5, 0.0)),
        SierpinskiTransform::new((0.0, 0.5, -0.5)),
        SierpinskiTransform::new((0.0, 0.5, 0.5))
    ];

    const DISCARD_ITERS: u32 = 5;
    const TOTAL_ITERS: u32 = 1000000;
    let mut rng = rand::thread_rng();

    let start_point: Vector = (
        rng.gen_range(-1.0f32, 1.0f32),
        rng.gen_range(-1.0f32, 1.0f32),
        rng.gen_range(-1.0f32, 1.0f32)
    );

    let mut current_point = start_point;
    let mut points: VectorList = Vec::new();
    let mut colors: ColorList = Vec::new();
    for i in 0..TOTAL_ITERS {
        if i < DISCARD_ITERS {
            continue;
        }

        points.push(current_point);
        colors.push((0u8, (i % 256) as u8, (i % 256 + 64) as u8));

        let index = rng.gen_range(0usize, 4usize);
        current_point = functions[index].transform(&current_point);
    }

    write_pnts("test.pnts", &points, &colors).expect("whoops, something went wrong");
}
