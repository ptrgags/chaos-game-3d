use std::fmt::{Debug, Formatter, Result};

extern crate rand;
use rand::Rng;
use rand::prelude::ThreadRng;

pub trait Chooser: Debug {
    fn choose(&mut self) -> usize;
}

pub struct UniformChooser {
    rng: ThreadRng,
    num_xforms: usize,
}

impl UniformChooser {
    pub fn new(n: usize) -> Self {
        Self {
            rng: rand::thread_rng(),
            num_xforms: n
        }
    }
}

impl Chooser for UniformChooser {
    fn choose(&mut self) -> usize {
        self.rng.gen_range(0usize, self.num_xforms)
    }
}

impl Debug for UniformChooser {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "UniformChooser({})", self.num_xforms)
    }
}
