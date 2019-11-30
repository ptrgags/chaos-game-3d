use json::JsonValue;

use crate::xforms;
use crate::xforms::Transform;
use crate::choosers;
use crate::choosers::Chooser;
use crate::vector::Vector3;

pub type Xform<T> = Box<dyn Transform<T>>;
pub type XformSelector = Box<dyn Chooser>;

#[derive(Debug)]
pub struct IFS<T> {
    xforms: Vec<Xform<T>>,
    chooser: XformSelector,
}

impl<T> IFS<T> {
    pub fn new(xforms: Vec<Xform<T>>, chooser: XformSelector) -> Self {
        Self { xforms, chooser }
    }

    pub fn transform(&mut self, vector: &Vector3<T>) -> Vector3<T> {
        let index = self.chooser.choose();
        let xform = &self.xforms[index];
        xform.transform(vector)
    }
}

pub fn from_json(json: &JsonValue) -> IFS<f32> {
    let xforms = parse_xforms(&json["xforms"]);
    let chooser = choosers::from_json(&json["chooser"], xforms.len());
    IFS::new(xforms, chooser)
}

fn parse_xforms(xform_arr: &JsonValue) -> Vec<Xform<f32>> {
    xform_arr.members().map(|xform_desc| {
        xforms::from_json(&xform_desc)
    }).collect()
}
