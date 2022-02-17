// This program always uses octrees
//const BRANCHING_FACTOR: usize = 8;

pub struct ImplicitCoordinates {
    pub subtree_levels: usize,
    pub level: usize,
    pub x: usize,
    pub y: usize,
    pub z: usize  
}

impl ImplicitCoordinates {
    pub fn root(subtree_levels: usize) -> Self {
        Self {
            subtree_levels,
            level: 0,
            x: 0,
            y: 0,
            z: 0,
        }
    }

    pub fn get_child_coordinates(&self, child_index: usize) -> Self {
        let level = self.level + 1;
        let x_bit = child_index & 1;
        let y_bit = (child_index >> 1) & 1;
        let z_bit = (child_index >> 2) & 1;

        let x = self.x << 1 | x_bit;
        let y = self.y << 1 | y_bit;
        let z = self.z << 1 | z_bit;

        Self {
            subtree_levels: self.subtree_levels,
            level,
            x,
            y,
            z
        }
    }
}