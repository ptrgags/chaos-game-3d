use crate::vector::Vec3;
use crate::octrees::OctNode;

pub trait Plotter {
    fn plot(&mut self, point: Vec3, color: Vec3);
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

    pub fn add_point(&mut self, point: Vec3, color: Vec3) {
        self.root.add(point, color);
    }
}

impl Plotter for ScatterPlot {
    fn plot(&mut self, point: Vec3, color: Vec3) {
        self.add_point(point, color);
    }
}
