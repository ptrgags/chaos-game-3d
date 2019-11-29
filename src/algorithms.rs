use json::JsonValue;

use crate::ifs::IFS;
use crate::buffers::Buffer;
use crate::vector::Vec3;

pub trait Algorithm {
    fn iterate(&mut self, n_iters: u32);
    fn save(&self, fname: &str);
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
}

impl Algorithm for ChaosGame {
    fn iterate(&mut self, n_iters: u32) {
        let mut pos = Vec3::rand_pos();
        let mut color_vec = Vec3::rand_pos();
        for i in 0..(STARTUP_ITERS + n_iters) {
            if i > STARTUP_ITERS {
                self.output_buffer.add(pos, color_vec.to_color())
            }

            pos = self.position_ifs.transform(pos);
            color_vec = self.color_ifs.transform(color_vec);
        }
    }

    fn save(&self, _fname: &str) {
        for (pos, color) in self.output_buffer {
            println!(
                "{} {} {} {} {} {}", 
                pos.x(), 
                pos.y(), 
                pos.z(), 
                color.x(), 
                color.y(), 
                color.z())
        }
    }
}

pub fn from_json(json: &JsonValue) {

}
