#[macro_use]
extern crate json;
extern crate rand;

mod xforms;
mod vector;
mod quaternion;
mod ifs;
mod initial_set;
mod choosers;
mod parameters;
mod algorithms;
mod buffers;
mod pointclouds;

use std::env;

fn let_the_chaos_begin(in_fname: &str, out_fname: &str, n_str: &str) {
    let mut chaos = parameters::load_algorithm(in_fname);
    let n: u32 = n_str.parse().expect("n must be a u32");
    chaos.iterate(n);
    chaos.save(out_fname);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.as_slice() {
        [_, in_file, out_file, n] => let_the_chaos_begin(in_file, out_file, n),
        _ => panic!("Usage: chaos-game-3d in_file out_file num_iters")
    }
}
