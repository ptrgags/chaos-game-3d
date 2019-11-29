use crate::vector::{Vec3, Color};

pub struct Buffer {
    points: Vec<Vec3>,
    colors: Vec<Color>,
}

impl Buffer {
    pub fn new() -> Self {
        Self {
            points: Vec::new(),
            colors: Vec::new(),
        }
    }
}
