use json::JsonValue;

use crate::xforms::{self, Transform, TranslatedSandwich};
use crate::choosers::{self, Chooser, UniformChooser};
use crate::multivector::Multivector;

// Type aliases for brevity
pub type Xform = Box<dyn Transform>;
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
pub struct IFS {
    /// A list of transformations to include
    xforms: Vec<Xform>,
    /// The chooser determines the method for selecting a transformation
    /// randomly. Often this is a uniform distribution, but it could also
    /// be a Markov chain or weighted probability distribution.
    chooser: XformSelector,
    /// The index of the last transform applied
    last_xform: usize,
}

impl IFS {
    pub fn new(xforms: Vec<Xform>, chooser: XformSelector) -> Self {
        Self { xforms, chooser, last_xform: 0}
    }

    /// Create the simplest possible IFS: the identity transformation
    /// and a unfiorm chooser
    pub fn identity() -> Self {
        let identity_xform = Box::new(TranslatedSandwich::identity());
        Self {
            xforms: vec![identity_xform],
            chooser: Box::new(UniformChooser::new(1)),
            last_xform: 0,
        }
    }

    /// Get the index of the last transformation applied. This is metadata
    /// that will be included in the point cloud (glTF only)
    pub fn get_last_xform(&self) -> u8 {
        self.last_xform as u8
    }

    /// Transform an individual point using a randomly-selected transformation
    /// from this IFS. The Chooser determines the random distribution
    pub fn transform(&mut self, vector: &Multivector) -> Multivector {
        let index = self.chooser.choose();
        let xform = &self.xforms[index];
        self.last_xform = index;
        xform.transform(vector)
    }

    /// Transform a vector containing points. This is used for transforming
    /// the points/colors of a Buffer.
    pub fn transform_points(
            &mut self, points: &Vec<Multivector>) -> Vec<Multivector> {
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
pub fn from_json(json: &JsonValue) -> IFS {
    match json {
        JsonValue::Null => IFS::identity(),
        JsonValue::Object(_) => {
            let xforms = parse_xforms(&json["xforms"]);
            let chooser = choosers::from_json(&json["chooser"], xforms.len());
            IFS::new(xforms, chooser)
        },
        _ => panic!("IFS JSON must be an object or null")
    }
}

/// Parse an array of transformations from JSON.
/// See the `xforms` module for more information.
///
/// There is also a shortcut transformation:
/// ["+inverse"]
///
/// When this element is encountered, the previous transformation's inverse
/// is added. This is a handy shortcut since often I want to describe groups
/// of transformations which requires specifying their inverses.
fn parse_xforms(xform_arr: &JsonValue) -> Vec<Xform> {
    let mut result = Vec::new();
 
    for xform_desc in xform_arr.members() {
        let type_name = xform_desc[0]
            .as_str()
            .expect("xforms: transformation type must be a string");

        match type_name {
            "+inverse" => add_inverse(&mut result),
            _ => {
                let xform = xforms::from_json(&xform_desc);
                result.push(xform);
            }
        };
    }

    result
}

/// For brevity, instead of typing out a function and its inverses, just
/// add the shortcut ["+inverse"] after a transformation, and its inverse
/// will be added to the list
fn add_inverse(results: &mut Vec<Xform>) {
    if results.is_empty() {
        panic!(concat!(
            "[\"+inverse\"] should be listed after a transformation in the ",
            "xforms list"));
    }

    let n = results.len();
    let inv = results[n - 1]
        .inverse()
        .expect(
            "[\"+inverse\"] may only go after an invertible transformation");
    results.push(inv);
}
