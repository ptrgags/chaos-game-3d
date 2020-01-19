use json::JsonValue;

use crate::ifs::{self, IFS};
use crate::initial_set::{self, InitialSet};
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
                self.output_buffer.add(pos, color_vec)
            }

            pos = self.position_ifs.transform(&pos);
            color_vec = self.color_ifs.transform(&color_vec);
        }
    }

    fn save(&mut self, fname: &str) {
        let mut writer = Cesium3DTilesWriter::new(10000000.0);
        writer.add_points(&mut self.output_buffer);
        writer.save(fname);
    }
}

/// Similar to ChaosGame, but instead of operating on a single input point,
/// this allows "condensation sets" (using Michael F. Barnsley's terminology),
/// which is a set of input points that gets transformed as a single unit
/// at each iteration.
pub struct ChaosSets { 
    position_ifs: IFS<f32>,
    color_ifs: IFS<f32>,
    initial_set: Box<dyn InitialSet>,
    initial_copies: usize,
    output_buffer: Buffer,
}

impl ChaosSets {
    pub fn new(
            position_ifs: IFS<f32>, 
            color_ifs: IFS<f32>, 
            initial_set: Box<dyn InitialSet>, 
            initial_copies: usize) -> Self {
        Self {
            position_ifs,
            color_ifs,
            initial_set,
            initial_copies,
            output_buffer: Buffer::new(),
        }
    }

    pub fn transform_buffer(&self, buffer: Buffer) -> Buffer {
        let mut new_buf = Buffer::new();

        for (pos, color) in buffer.points_iter() {
            let new_pos = self.position_ifs.transform(&pos);
            let new_color = self.color_ifs.transform(&color);
            new_buf.add(new_pos, new_color);
        }

        return new_buf;
    }

    pub fn from_json(json: &JsonValue) -> Self {
        let position_ifs = ifs::from_json(&json["ifs"]);
        let color_ifs = ifs::from_json(&json["color_ifs"]);
        let arranger = initial_set::from_json(&json["initial_set"]);
        let initial_copies: usize = json["initial_set_copies"]
            .as_usize()
            .expect("initial_copies must be a positive integer");

        Self::new(position_ifs, color_ifs, arranger, initial_copies)
    }
}

impl Algorithm for ChaosSets {
    fn iterate(&mut self, n_iters: u32) {
        let mut buffers: Vec<Buffer> = (0..self.initial_copies).map(|_| {
            self.initial_set.generate()
        }).collect();
        self.output_buffer.copy_from(&buffers[0]);

        for _ in 0..n_iters {
            let new_buffers: Vec<Buffer> = Vec::new();
            for buf in buffers.into_iter() {
                let new_buf = self.transform_buffer(buf);
                self.output_buffer.copy_from(&new_buf);
                new_buffers.push(new_buf);
            }
            buffers = new_buffers;
        }
    }

    fn save(&mut self, fname: &str) {
        let mut writer = Cesium3DTilesWriter::new(10000000.0);
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
        "chaos_sets" => Box::new(ChaosSets::from_json(&json)) as Box<dyn Algorithm>,
        _ => panic!("invalid algorithm!, {}", &algorithm_id[..])
    }
}
