use std::collections::HashSet;

use crate::half_multivector::HalfMultivector;
use crate::fractal_metadata::FractalMetadata;

type Coordinates = (u64, u64, u64);

pub struct BoxCountingEstimator {
    /// Radius of the octree's largest bounding volume
    radius: f64,
    /// number of levels of the tree to do the calculation for.
    /// Larger values are more accurate, but are more expensive to compute
    levels: usize,
    // how many boxes across in each dimension per level of the tree.
    // this is essentially just a lookup table of 2^d where d is the level
    boxes_per_side: Vec<f64>,
    /// Size of the box at each level of the tree
    box_side_lengths: Vec<f64>,
    /// At each level of the tree, keep track of the coordinates of any box
    /// that has at least one point. We only care about the count, so a set
    /// is used to limit the memory needed
    boxes: Vec<HashSet<Coordinates>>,
}

impl BoxCountingEstimator {
    pub fn new(radius: f64, levels: usize) -> Self {
        // there are 2^d boxes in each dimension of the octree at level d
        let boxes_per_side = 
            (0..levels).map(|d| (d as f64).exp2()).collect();
        
        // The side length of a box at level d is
        // 2 * radius / 2^d
        let max_side_length = 2.0 * radius;            
        let box_side_lengths = (0..levels)
            .map(|d| max_side_length / (d as f64).exp2())
            .collect();

        let boxes = vec![HashSet::new(); levels];

        Self {
            radius,
            levels,
            boxes_per_side,
            box_side_lengths,
            boxes
        }
    }

    pub fn add_point(&mut self, position: &HalfMultivector) {
        let [mut x, mut y, mut z] = position.get_vector_components();

        // Measure coordinates from the corner of the box, (-r, -r, -r)
        let corner = -self.radius;
        x -= corner;
        y -= corner;
        z -= corner;

        // Compute the point's coordinates at each level of tree and
        // update the appropriate hash set
        for d in 0..self.levels {
            let side_length = self.box_side_lengths[d];
            let x_coord = (x / side_length).floor() as u64;
            let y_coord = (y / side_length).floor() as u64;
            let z_coord = (z / side_length).floor() as u64;
            let coords = (x_coord, y_coord, z_coord);
            self.boxes[d].insert(coords);
        }
    }

    /// Estimate the fractal dimension
    /// 
    /// Fractal dimension is defined as
    /// 
    /// lim(epsilon -> 0) (log(N(epsilon)) / log(1/epsilon))
    /// 
    /// Where epsilon is the box side length and N(epsilon) is the box count
    /// at the corresponding level.
    /// 
    /// In my case, 
    ///        epsilon = self.box_side_lengths[i]
    ///     N(epsilon) = self.boxes[i].len()
    /// 
    /// Taking a linear regression of the results will compute the slope, which
    /// is an estimate for this limit.
    /// 
    ///
    /// See also this article: http://paulbourke.net/fractals/cubecount/ where
    /// I got the linear reegression idea from
    pub fn estimate_fractal_dimension(&self) -> f64 {
        println!("Estimating fractal dimension...");
        // computing log(N(epsilon))
        let log_box_counts: Vec<f64> = self.boxes
            .iter()
            // It doesn't matter which logarithm base we use here as long as
            // the numerator and denominator are consistent. So stick to binary
            .map(|x| (x.len() as f64).log2())
            .collect();
        
        // Computing log(1/epsilon) = -log(epsilon)
        let log_side_lengths: Vec<f64> = self.box_side_lengths
            .iter()
            .map(|x| -x.log2())
            .collect();

        let (fractal_dimension, _) = linreg::linear_regression(
            &log_side_lengths, &log_box_counts
        ).unwrap();

        println!("Estimated fractal dimension: {}", fractal_dimension);
        
        fractal_dimension
    }

    pub fn update_metadata(&self, metadata: &mut FractalMetadata) {
        metadata.fractal_dimension = self.estimate_fractal_dimension();
        metadata.fractal_dimension_levels = self.levels as u8;
        metadata.box_sizes = self.box_side_lengths.clone();
        metadata.box_counts = 
            self.boxes.iter().map(|x| x.len() as u64).collect();
    }
}