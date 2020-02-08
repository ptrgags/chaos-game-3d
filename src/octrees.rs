use crate::bbox::BBox;
use crate::buffers::{OutputBuffer};
use crate::vector::Vec3;

/// Pointer to a child node, which can be null.
type Child<T> = Option<Box<T>>;

/// Octree node
pub struct OctNode {
    /// 8 children. This is always either completely empty or completely full 
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

    /// Add a point to the octree recursively. If there are already many points
    /// in the relevant leaf node, the point may be discarded.
    ///
    /// This returns true if the point was added, or false if the point was
    /// discarded because it didn't fit.
    fn add_point_recursive(
            &mut self, 
            point: Vec3, 
            color: Vec3, 
            depth: u8, 
            max_depth: u8) -> bool {
        let is_leaf = self.is_leaf();
        let is_full = self.is_full();
        if is_leaf && !is_full {
            // Base case 1: We're at a leaf with some space. just add the point. 
            self.points.add(point, color);
            self.count += 1;
            self.color_sum = self.color_sum + color;
            return true;
        } else if is_leaf && is_full && depth < max_depth {
            // Base case 2: We're at a full leaf. Subdivide this node and
            // retry the add operation on this node which is now an internal
            // node. Note that this always produces 8 children
            self.subdivide();
            return self.add_point_recursive(point, color, depth, max_depth);
        } else if is_leaf && is_full && depth == max_depth {
            // Base case 3: We're at a full leaf but we've hit the depth
            // limit. Just discard the point to prevent infinite loops
            return false;
        } else if !is_leaf {
            // Recursive case: Find the octant which the point is in, and
            // insert into the child node
            let quadrant = self.bounds.find_quadrant(&point);
            let child = &mut self.children[quadrant]
                .expect("Unexpected null child");
            let result = child.add_point_recursive(
                point, color, depth + 1, max_depth);
            if result {
                self.count += 1;
                self.color_sum = self.color_sum + color;
            }
            return result;
        } else {
            panic!(
                "Invalid case: is_leaf: {}, is_full: {}, depth: {}/{}",
                is_leaf,
                is_full,
                depth,
                max_depth);
        }
    }

    /// Subdivide a leaf node into an interior node with 8 children, one
    /// per octant.
    fn subdivide(&mut self) {
        assert!(self.is_leaf(), "can only subdivide leaf nodes");
        let child_bounds = self.bounds.subdivide();
        for (i, bounds) in child_bounds.into_iter().enumerate() {
            let child = Self::child_node(*bounds, self.capacity);
            self.children[i] = Some(Box::new(child))
        }
    }
}

