use crate::vector::Vec3;

/// A buffer is a container of colored points.
/// It is stored as a pair of parallel vectors of points and colors.
#[derive(Clone)]
pub struct Buffer {
    points: Vec<Vec3>,
    colors: Vec<Vec3>,
}

impl Buffer {
    pub fn new() -> Self {
        Self {
            points: Vec::new(),
            colors: Vec::new(),
        }
    }

    /// Move points from other to the end of self, leaving other empty.
    pub fn move_from(&mut self, other: &mut Self) {
        self.points.append(&mut other.points);
        self.colors.append(&mut other.colors);
    }

    /// Copy points from other to the end of self.
    pub fn copy_from(&mut self, other: &Self) {
        self.points.extend(other.points.iter().cloned());
        self.colors.extend(other.colors.iter().cloned());
    }

    /// Add a new point to the buffer
    pub fn add(&mut self, point: Vec3, color: Vec3) {
        self.points.push(point);
        self.colors.push(color);
    }

    /// How many points are in this buffer?
    pub fn len(&self) -> usize {
        return self.points.len();
    }

    /// Get an iterator over this buffer. This iterator clones values
    pub fn points_iter(self) -> BufferIterator {
        BufferIterator {
            buffer: self,
            index: 0
        }
    }
}

/// Iterate over a buffer's (point, color) pairs. This clones points rather
/// than taking a reference or taking ownership. I may regret this someday,
/// we'll see.
pub struct BufferIterator {
    buffer: Buffer,
    index: usize
}

impl Iterator for BufferIterator {
    type Item = (Vec3, Vec3);

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
