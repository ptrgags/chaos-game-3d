use crate::bbox::BBox;
use crate::buffers::{OutputBuffer};
use crate::vector::Vec3;

/// Pointer to a child node, which can be null.
type Child<T> = Option<Box<T>>;

/// Octree node
pub struct OctNode {
    /// 8 children. This is always either completely 
    children: [Child<OctNode>; 8],
    /// Bounding box for this node
    bounds: BBox,
    /// Store points in this node. They are stored as Vec3 to be more compact
    /// and because this matches the 3D Tiles spec
    points: OutputBuffer,
    /// How many points can fit in this node
    capacity: usize,
    /// How many fits currently are in this node
    count: usize,
    /// Total color. This can be used along with count to compute the average
    /// color
    color_sum: Vec3,
}

impl OctNode {
    /// Create an empty root node surrounding the origin. The half-width "radius"
    /// of the box must be specified since all other node bounding boxes are
    /// derived from this node.
    pub fn root_node(radius: f32, capacity: usize) -> Self {
        Self {
            children: [None, None, None, None, None, None, None, None],
            bounds: BBox::new(
                -radius, radius, 
                -radius, radius, 
                -radius, radius),
            points: OutputBuffer::new(),
            capacity,
            count: 0,
            color_sum: Vec3::zero(),
        }
    }

    /// Create an empty child node with given bounds
    pub fn child_node(bounds: BBox, capacity: usize) -> Self {
        Self {
            children: [None, None, None, None, None, None, None, None],
            bounds,
            points: OutputBuffer::new(),
            capacity,
            count: 0,
            color_sum: Vec3::zero(),
        }
    }

    /// Check if a node is a leaf node by checking that it has no children
    pub fn is_leaf(&self) -> bool {
        for child in self.children.iter() {
            if let Some(_) = child {
                return false;
            }
        }

        true
    }

    /// Check if a node is full up to capacity
    pub fn is_full(&self) -> bool {
        self.points.len() == self.capacity
    }

    /// Add a point from the top of the tree down. If this overfills the node,
    /// subdivide it as necessary, up to the given max depth.
    pub fn add_point(
            &mut self, point: Vec3, color: Vec3, max_depth: u8) {
        // Discard points outside the grid
        if !self.bounds.contains(&point) {
            return;
        } 

        self.add_point_recursive(point, color, 0, max_depth);
    }

    // TODO: Plan this algorithm a little more
    fn add_point_recursive(
            &mut self, point: Vec3, color: Vec3, depth: u8, max_depth: u8) {
        if self.is_leaf() {
            if self.points.len() < self.capacity {
                self.points.add(point, color);
                self.count += 1;
                self.color_sum = self.color_sum + color;
            } else {
                self.subdivide();
            }
        } else {
            let quadrant = self.bounds.find_quadrant(&point);
            match &mut self.children[quadrant] {
                Some(child) => {
                    child.add_point_recursive(
                        point, color, depth + 1, max_depth);
                },
                None => {
                    // Limit the tree's depth to prevent infinite loops.
                    // This can happen if a point is repeated many times.
                    if depth < max_depth {
                        self.add_child(quadrant);
                    }
                }
            }
        }
    }


    /*
    pub fn add(&mut self, point: Vec3, color: Vec3) {

        if self.is_leaf() {
            self.points.add(point, color);

            if self.points.len() > self.capacity {
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
    */

    /*
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
    */

    fn add_child(&mut self, quadrant: usize, point: Vec3, color: Vec3) {
        let bounds = self.bounds.make_quadrant(quadrant);
        let mut child = Self::child_node(bounds, self.capacity);
        child.add(point, color);
        self.children[quadrant] = Some(Box::new(child));
    }
}

