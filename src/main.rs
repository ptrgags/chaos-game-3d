#[macro_use]
extern crate json;
extern crate rand;

#[macro_use]
mod macros;

mod algorithms;
mod bbox;
mod choosers;
mod fractal_metadata;
mod glb_writer;
mod ifs;
mod half_multivector;
mod initial_set;
//mod multivector;
mod octrees;
mod parameters;
mod plotters;
mod pnts_writer;
mod point;
mod tileset_writer;
mod vector;
mod xforms;

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
