#[macro_use]
extern crate json;
extern crate rand;

#[macro_use]
mod macros;

mod algorithms;
mod bbox;
mod choosers;
mod clusters;
mod fractal_metadata;
mod glb_writer;
mod ifs;
mod implicit_coordinates;
mod half_multivector;
mod octrees;
mod plotters;
mod pnts_writer;
mod point;
mod tileset_writer;
mod vector;
mod xforms;

use std::env;
use std::fs::File;
use std::io::Read;

use json::{JsonValue, parse};
use crate::algorithms::Algorithm;

pub fn load_json_file(fname: &str) -> JsonValue {
    let mut file = File::open(fname).expect("Could not open file");
    let mut text = String::new();
    file.read_to_string(&mut text).expect("Could not read file contents");

    parse(&text).expect("Could not parse JSON")
}

pub fn load_algorithm(fname: &str) -> Box<dyn Algorithm> {
    let json = load_json_file(fname);
    algorithms::from_json(&json)
}

fn let_the_chaos_begin(in_fname: &str) {
    let mut chaos = load_algorithm(in_fname);
    println!("Estimated complexity: {} points", chaos.complexity());
    chaos.iterate();
    chaos.save();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.as_slice() {
        [_, in_file] => let_the_chaos_begin(in_file),
        _ => panic!("Usage: chaos-game-3d in_file")
    }
}
