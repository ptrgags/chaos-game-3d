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

/// A chooser for tilings with pairs of inverse transformations. After taking
/// a step, the next step must not be the inverse to avoid backtracking.
/// This is most effective for 1 or 2 pairs of transformations, beyond that
/// the difference falls off roughly with the square of the number of pairs.
pub struct NoBacktrackingChooser {
    // The last transformation that was applied
    last_selection: usize,
    // The total number of transformations
    num_xforms: usize,
    // The random number generator
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

/// A chooser that uses a Markov Chain to provide more control over the
/// probability distribution.
/// 
/// Instead of entering probabilities, this takes a matrix of weights, where
/// the row indicates the last transformation applied and the column indicates
/// a transation from last -> current transformation. The weights are
/// automatically turned into cumulative proability distributions
/// 
/// For increased control, there's also one extra row for initial weights.
/// This is a probability distribution for the first transformation
pub struct MarkovChooser {
    /// cumulative probabilities for the initial iteration.
    /// This is useful for hinting where to start
    initial_probabilities: Vec<f64>,
    /// cumulative probabilities matrix for subsequent iterations
    cumulative_probabilities: Vec<Vec<f64>>,
    // The total number of transformations
    num_xforms: usize,
    // The last transformation that was applied
    last_selection: usize,
    // The random number generator.
    rng: ThreadRng,
}

impl MarkovChooser {
    /// Parse a Markov chain chooser from JSON of the form:
    /// 
    /// {
    ///     "type": "markov",
    ///     "initial_weights": [w0, w1, ...]
    ///     "weights": [
    ///       [w00, w01, ...],
    ///       [w10, w11, ...],
    ///       ...
    ///     ]
    /// }
    pub fn from_json(json: &JsonValue) -> Self {
        let weights = Self::parse_weights(json);
        let n = weights.len();

        let initial_weights = Self::parse_weights_row(&json["initial_weights"]);
        let initial_probabilities =
            Self::weights_to_cumulative_probabilities(&initial_weights);
        
        // sanity check
        if initial_probabilities.len() != n {
            panic!("initial_weights must be the same length as the weights matrix rows");
        }

        let mut cumulative_probabilities = Vec::new();
        for row in weights {
            if row.len() != n {
                panic!("weights must be a square matrix");
            }

            let cumulative_row = 
                Self::weights_to_cumulative_probabilities(&row);
            cumulative_probabilities.push(cumulative_row);
        }

        Self {
            initial_probabilities,
            cumulative_probabilities,
            num_xforms: n,
            last_selection: n + 1,
            rng: rand::thread_rng(),
        }
    }

    fn parse_weights(json: &JsonValue) -> Vec<Vec<f64>> {
        let mut weights = Vec::new();
        for row_json in json["weights"].members() {
            let row = Self::parse_weights_row(row_json);
            weights.push(row);
        }
        weights
    }

    fn parse_weights_row(json: &JsonValue) -> Vec<f64> {
        match json {
            JsonValue::Array(components) => 
                components.iter()
                    .map(|x| x.as_f64().expect("weights must be numbers"))
                    .collect(),
            _ => panic!("weights row must be an array of numbers")
        }
    }

    fn weights_to_cumulative_probabilities(weights: &Vec<f64>) -> Vec<f64> {
        let n = weights.len();
        let mut probabilities = vec![0.0; n];

        let weight_sum: f64 = weights.iter().sum();

        let mut cumulative_sum = 0.0;
        for (i, weight) in weights.iter().enumerate() {
            cumulative_sum += weight;
            probabilities[i] = cumulative_sum / weight_sum;
        }
        probabilities
    }
}

impl Chooser for MarkovChooser {
    fn choose(&mut self) -> usize {
        let n = self.num_xforms;


        let probabilities;
        if self.last_selection == n + 1 {
            // For the first iteration, use the initial weights.
            probabilities = &self.initial_probabilities;
        } else {
            // Get the row of cumulative probabilities from the previous
            // transformation.
            probabilities = &self.cumulative_probabilities[self.last_selection];
        }
        
        let value: f64 = self.rng.gen_range(0.0, 1.0);
        for (i, probability) in probabilities.iter().enumerate() {
            if value <= *probability {
                self.last_selection = i;
                return i;
            }
        }

        panic!("Should never reach here - maybe probabilities weren't normalized correctly?");
    }

    fn reset(&mut self) {
        self.last_selection = self.num_xforms + 1;
    }
}

impl Debug for MarkovChooser {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "MarkovChooser({}, {}, {:?})",
            self.num_xforms,
            self.last_selection,
            self.cumulative_probabilities)
    }
}

/// Parse a transformation chooser from the IFS JSON
/// 
/// ```text
/// {
///     "type": "uniform" (default "uniform"),
///     ...params
/// }
/// ```
pub fn from_json(json: &JsonValue, n: usize) -> Box<dyn Chooser> {
    let chooser_type = json["type"].as_str().unwrap_or("uniform");

    match &chooser_type[..] {
        "uniform" => Box::new(UniformChooser::new(n)),
        "no_backtracking" => Box::new(NoBacktrackingChooser::new(n)),
        "markov" => Box::new(MarkovChooser::from_json(json)),
        _ => panic!("Invalid chooser type")
    }
}
