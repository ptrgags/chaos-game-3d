#[macro_use]
extern crate json;
extern crate rand;

#[macro_use]
mod macros;
mod xforms;
mod vector;
mod ifs;
mod initial_set;
mod choosers;
mod parameters;
mod algorithms;
mod buffers;
mod pointclouds;
mod multivector;

use std::env;

fn let_the_chaos_begin(in_fname: &str, out_fname: &str) {
    let mut chaos = parameters::load_algorithm(in_fname);
    println!("Estimated complexity: {} points", chaos.complexity());
    chaos.iterate();
    chaos.save(out_fname);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.as_slice() {
        [_, in_file, out_file] => let_the_chaos_begin(in_file, out_file),
        _ => panic!("Usage: chaos-game-3d in_file out_file")
    }
}
