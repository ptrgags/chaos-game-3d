use std::fmt::{Debug, Formatter, Result};

use rand::Rng;
use rand::prelude::ThreadRng;
use json::JsonValue;

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


/// Parse a transformation chooser from the IFS JSON
/// 
/// ```text
/// {
///     "chooser": "uniform" (default "uniform"),
///     ...params
/// }
/// ```
pub fn from_json(json: &JsonValue, n: usize) -> Box<dyn Chooser> {
    let chooser_type = json.as_str().unwrap_or("uniform");

    match &chooser_type[..] {
        "uniform" => Box::new(UniformChooser::new(n)),
        _ => panic!("Invalid chooser type")
    }
}
