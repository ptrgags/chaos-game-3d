use json::JsonValue;

use crate::ifs;
use crate::ifs::IFS;
use crate::buffers::Buffer;
use crate::vector::Vec3;
use crate::pointclouds::{/*CSVWriter, */Cesium3DTilesWriter, PointCloudWriter};

pub trait Algorithm {
    fn iterate(&mut self, n_iters: u32);
    fn save(&mut self, fname: &str);
}

const STARTUP_ITERS: u32 = 10;

pub struct ChaosGame {
    position_ifs: IFS<f32>,
    color_ifs: IFS<f32>,
    output_buffer: Buffer
}

impl ChaosGame {
    pub fn new(position_ifs: IFS<f32>, color_ifs: IFS<f32>) -> Self {
        Self {
            position_ifs,
            color_ifs,
            output_buffer: Buffer::new()
        }
    }

    pub fn from_json(json: &JsonValue) -> Self {
        let position_ifs = ifs::from_json(&json["ifs"]);
        let color_ifs = ifs::from_json(&json["color_ifs"]);

        Self::new(position_ifs, color_ifs)
    }
}

impl Algorithm for ChaosGame {
    fn iterate(&mut self, n_iters: u32) {
        let mut pos = Vec3::random();
        let mut color_vec = Vec3::random_color();
        for i in 0..(STARTUP_ITERS + n_iters) {
            if i >= STARTUP_ITERS {
                self.output_buffer.add(pos, color_vec.to_color())
            }

            pos = self.position_ifs.transform(&pos);
            color_vec = self.color_ifs.transform(&color_vec);
        }
    }

    fn save(&mut self, fname: &str) {
        let mut writer = Cesium3DTilesWriter::new(10000000.0);//CSVWriter::new();
        writer.add_points(&mut self.output_buffer);
        writer.save(fname);
    }
}

pub fn from_json(json: &JsonValue) -> Box<dyn Algorithm> {
    let algorithm_id = &json["algorithm"]
        .as_str()
        .expect("algorithm must be a string");

    match &algorithm_id[..] {
        "chaos" => Box::new(ChaosGame::from_json(&json)) as Box<dyn Algorithm>,
        _ => panic!("invalid algorithm!")
    }
}
