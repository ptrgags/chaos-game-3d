use std::fmt::{Debug, Formatter, Result};

use rand::Rng;
use rand::prelude::ThreadRng;
use json::JsonValue;

pub trait Chooser: Debug {
    fn choose(&mut self) -> usize;

    /// Some choosers have internal state, reset this when starting
    /// new points for scratch
    fn reset(&mut self) {}
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

pub struct NoBacktrackingChooser {
    last_selection: usize,
    num_xforms: usize,
    rng: ThreadRng,
}

impl NoBacktrackingChooser {
    pub fn new(n: usize) -> Self {
        if n % 2 == 1 {
            panic!("no_backtracking must have xforms in inverse pairs, e.g. [A, A^(-1), B, B^(-1)]")
        }

        Self {
            rng: rand::thread_rng(),
            last_selection: n + 1,
            num_xforms: n
        }
    }
}

impl Chooser for NoBacktrackingChooser {
    fn choose(&mut self) -> usize {
        let n = self.num_xforms;

        // For the first iteration, choose any transformation
        if self.last_selection == n + 1 {
            let selection = self.rng.gen_range(0usize, n);
            self.last_selection = selection;
            return selection;
        }

        // If we just applied transformation A, forbid its inverse A^(-1).
        // If we just applied an inverse A^(-1), forbid the original A.
        // This assumes transformations are listed in inverse pairs, i.e.
        // A, A^(-1), B, B^(-1) and so on
        let forbidden = if self.last_selection % 2 == 0 { 
            self.last_selection + 1
        } else { 
            self.last_selection - 1
        };

        // Pick a random number from 1 less than the entire range
        let mut selection = self.rng.gen_range(0usize, n - 1);

        // Shift the upper part of the range to avoid the forbidden
        // transformation. For example, 
        // forbidden: 3
        // xforms:       0 1 2 3 4 5
        // random range: 0 1 2 3 4
        // after shift:  0 1 2 - 4 5 (3 is avoided)
        // doing the selection this way avoids having to run the random
        // generator multiple times
        if selection >= forbidden {
            selection += 1;
        }

        self.last_selection = selection;
        selection
    }

    fn reset(&mut self) {
        self.last_selection = self.num_xforms + 1
    }
}

impl Debug for NoBacktrackingChooser {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "NoBacktrackingChooser({}, {})", self.num_xforms, self.last_selection)
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
        "no_backtracking" => Box::new(NoBacktrackingChooser::new(n)),
        _ => panic!("Invalid chooser type")
    }
}
