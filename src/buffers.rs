use crate::vector::{Vec3, Color};

#[derive(Clone)]
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

    pub fn add(&mut self, point: Vec3, color: Color) {
        self.points.push(point);
        self.colors.push(color);
    }

    pub fn len(&self) -> usize {
        return self.points.len();
    }
}

impl IntoIterator for Buffer {
    type Item = (Vec3, Color);
    type IntoIter = BufferIterator;

    fn into_iter(self) -> Self::IntoIter {
        BufferIterator {
            buffer: self,
            index: 0
        }
    }
}

pub struct BufferIterator {
    buffer: Buffer,
    index: usize
}

impl Iterator for BufferIterator {
    type Item = (Vec3, Color);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.buffer.len() {
            return None
        }

        let pos = self.buffer.points[self.index];
        let color = self.buffer.colors[self.index];
        self.index += 1;
        
        Some((pos, color))
    }
}
