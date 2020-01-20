use crate::bbox::BBox;
use crate::buffers::Buffer;
use crate::vector::Vec3;

type Child<T> = Option<Box<T>>;

pub struct OctNode {
    children: [Child<OctNode>; 8],
    bounds: BBox,
    points: Buffer,
    capacity: usize,
    count: usize,
    color_sum: Vec3,
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
            capacity,
            count: 0,
            color_sum: Vec3::zero(),
        }
    }

    pub fn child_node(bounds: BBox, capacity: usize) -> Self {
        Self {
            children: [None, None, None, None, None, None, None, None],
            bounds,
            points: Buffer::new(),
            capacity,
            count: 0,
            color_sum: Vec3::zero(),
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
                    if (depth < max_depth) {
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

