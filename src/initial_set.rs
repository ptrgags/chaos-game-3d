use crate::buffers::Buffer;
use crate::vector::Vec3;

use rand::Rng;
use rand::prelude::ThreadRng;
use json::JsonValue;

pub trait InitialSet {
    fn generate(&self) -> Buffer; 
}

pub struct RandomBox {
    center: Vec3,
    dimensions: Vec3,
    color: Vec3,
    rng: ThreadRng,
}

impl RandomBox {
    pub fn new(center: Vec3, dimensions: Vec3, color: Vec3) -> Self {
        Self {
            center,
            dimensions,
            color,
            rng: rand::thread_rng(), 
        }
    }
}

impl InitialSet for RandomBox {
    fn generate(&self) -> Buffer {
        let x = 
    }
}

pub struct RandomLine {
    start: Vec3,
    end: Vec3,
    color: Vec3,
    rng: ThreadRng,
}

impl RandomLine {
    pub fn new(start: Vec3, end: Vec3, color: Vec3) -> Self {
        Self {
            start, 
            end,
            color,
            rng: rand::thread_rng(), 
        }
    }
}

impl InitialSet for RandomLine {
    fn generate(&self) -> Buffer {

    }
}

pub fn from_json(json: &JsonValue) -> Box<dyn InitialSet> {
    let type_id = &json["type"]
        .as_str()
        .expect("type must be a string");

    match &type_id[..] {
        "box" => Box::new(RandomBox::from_json(&json)) as Box<dyn InitialSet>,
        "line" => Box::new(RandomLine::from_json(&json)) as Box<dyn InitialSet>,
        _ => panic!("invalid initial set type {}", type_id)
    }
}
