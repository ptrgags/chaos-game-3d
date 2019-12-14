use crate::bbox::BBox;
use crate::buffers::Buffer;
use crate::vector::{Vec3, Color};

type Child<T> = Option<Box<T>>;

pub struct OctNode {
    children: [Child<OctNode>; 8],
    bounds: BBox,
    points: Buffer,
    capacity: usize
}

impl OctNode {
    pub fn root_node(radius: f32, capacity: usize) -> Self {
        Self {
            children: [None, None, None, None, None, None, None, None],
            bounds: BBox::new(
                -radius, radius, 
                -radius, radius, 
                -radius, radius),
            points: Buffer::new(),
            capacity
        }
    }

    pub fn child_node(bounds: BBox, capacity: usize) -> Self {
        Self {
            children: [None, None, None, None, None, None, None, None],
            bounds,
            points: Buffer::new(),
            capacity
        }
    }

    pub fn is_leaf(&self) -> bool {
        for child in self.children.iter() {
            if let Some(_) = child {
                return false;
            }
        }

        true
    }

    pub fn add(&mut self, point: Vec3, color: Color) {
        // Discard points outside the grid
        if !self.bounds.contains(&point) {
            return;
        }

        if self.is_leaf() {
            self.points.add(point, color);

            if self.points.count() > self.capacity {
                self.subdivide();
            }
        } else {
            let quadrant = self.bounds.find_quadrant(&point);
            match &mut self.children[quadrant] {
                Some(child) => { 
                    child.add(point, color);
                },
                None => {
                    self.add_child(quadrant, point, color);
                }
            }
        }
    }

    fn subdivide(&mut self) {
        /*
        let mut children: [OctNode; 8] = self.bounds.subdivide().into_iter().map(|bounds| {
            Self::child_node(bounds, self.capacity)
        }).collect();

        for bounds in child_bounds.into_iter() {
            Self::child_node(bounds, self.capacity);
        }
        */
    }

    fn add_child(&mut self, quadrant: usize, point: Vec3, color: Color) {
        let bounds = self.bounds.make_quadrant(quadrant);
        let mut child = Self::child_node(bounds, self.capacity);
        child.add(point, color);
        self.children[quadrant] = Some(Box::new(child));
    }
}

