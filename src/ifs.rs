use json::JsonValue;

use crate::xforms;
use crate::xforms::Transform;
use crate::choosers;
use crate::choosers::Chooser;
use crate::vector::Vector3;

// Type aliases for brevity
pub type Xform<T> = Box<dyn Transform<T>>;
pub type XformSelector = Box<dyn Chooser>;

/// An Iterated Function System is a set of functions (transformations) that 
/// can be applied over and over to the same input set in various combinations. 
/// often these functions form a group or at least a semigroup under composition.
///
/// Since this application uses the Chaos Game and other similar Monte-Carlo
/// style algorithms, the transformations are selected randomly using a
/// Chooser.
///
/// Note that this can be used for both position vectors *and* color vectors!
///
/// TODO: Once I add multivectors, consider changing T to Vec3
#[derive(Debug)]
pub struct IFS<T> {
    /// A list of transformations to include
    xforms: Vec<Xform<T>>,
    /// The chooser determines the method for selecting a transformation
    /// randomly. Often this is a uniform distribution, but it could also
    /// be a Markov chain or weighted probability distribution.
    chooser: XformSelector,
}

impl<T> IFS<T> {
    pub fn new(xforms: Vec<Xform<T>>, chooser: XformSelector) -> Self {
        Self { xforms, chooser }
    }

    /// Transform an individual point using a randomly-selected transformation
    /// from this IFS. The Chooser determines the random distribution
    pub fn transform(&mut self, vector: &Vector3<T>) -> Vector3<T> {
        let index = self.chooser.choose();
        let xform = &self.xforms[index];
        xform.transform(vector)
    }

    /// Transform a vector containing points. This is used for transforming
    /// the points/colors of a Buffer.
    pub fn transform_points(
            &mut self, points: &Vec<Vector3<T>>) -> Vec<Vector3<T>> {
        let index = self.chooser.choose();
        let xform = &self.xforms[index];

        points.iter().map(|point| xform.transform(point)).collect()
    }
}

/// Parse an IFS from JSON of the form:
/// ```text
/// {
///     "chooser": <Chooser JSON>,
///     "xforms": [<XFormJson>, ...],
/// }
/// ```
pub fn from_json(json: &JsonValue) -> IFS<f32> {
    let xforms = parse_xforms(&json["xforms"]);
    let chooser = choosers::from_json(&json["chooser"], xforms.len());
    IFS::new(xforms, chooser)
}

/// Parse an array of transformations from JSON
/// See the `xforms` module for more information.
fn parse_xforms(xform_arr: &JsonValue) -> Vec<Xform<f32>> {
    xform_arr.members().map(|xform_desc| {
        xforms::from_json(&xform_desc)
    }).collect()
}
