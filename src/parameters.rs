use std::fs::File;
use std::io::Read;

use json::{JsonValue, parse};
use crate::ifs;
use crate::ifs::IFS;
use crate::algorithms;
use crate::algorithms::Algorithm;

pub fn load_json_file(fname: &str) -> JsonValue {
    let mut file = File::open(fname).expect("Could not open file");
    let mut text = String::new();
    file.read_to_string(&mut text).expect("Could not read file contents");

    parse(&text).expect("Could not parse JSON")
}

pub fn load_ifs(fname: &str) -> IFS<f32> {
    let json = load_json_file(fname);
    ifs::from_json(&json["ifs"])
}

pub fn load_algorithm(fname: &str) -> Box<dyn Algorithm> {
    let json = load_json_file(fname);
    algorithms::from_json(&json);
}
