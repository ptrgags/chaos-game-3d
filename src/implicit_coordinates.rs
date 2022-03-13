/// coordinates of a tile for implicit tiling
pub struct ImplicitCoordinates {
    /// How many levels are in each subtree
    pub subtree_levels: usize,
    /// The level of this tile. The root is level 0, and levels increase
    /// by one each time.
    pub level: usize,
    /// The x coordinate of this tile
    pub x: usize,
    /// The y coordinate of this tile
    pub y: usize,
    /// The z coordinate of this tile
    pub z: usize  
}

impl ImplicitCoordinates {
    /// The root is always (level, x, y, z) = (0, 0, 0, 0)
    pub fn root(subtree_levels: usize) -> Self {
        Self {
            subtree_levels,
            level: 0,
            x: 0,
            y: 0,
            z: 0,
        }
    }

    /// Given the parent coordinates and which of the 8 children, create
    /// a new set of coordinates for that child tile.
    pub fn get_child_coordinates(&self, child_index: usize) -> Self {
        let level = self.level + 1;

        // the 3 bits of the child index are simply appended to the
        // x, y, and z coordinates.
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