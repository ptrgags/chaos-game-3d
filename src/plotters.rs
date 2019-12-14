use crate::vector::{Vec3, Color};
use crate::octtrees::OctNode;

pub trait Plotter {
    fn plot(&mut self, point: Vec3, color: Color);
}

pub struct ScatterPlot {
    root: OctNode
}

impl ScatterPlot {
    pub fn new(node_capacity: usize, radius: f32) -> Self {
        Self {
            root: OctNode::root_node(radius, node_capacity)
        }
    }

    pub fn add_point(&mut self, point: Vec3, color: Color) {
        self.root.add(point, color);
    }
}

impl Plotter for ScatterPlot {
    fn plot(&mut self, point: Vec3, color: Color) {
        self.add_point(point, color);
    }
}

pub struct DensityPlot {
    max_depth: u32,
    num_points: u64,
    root: OctNode
}
