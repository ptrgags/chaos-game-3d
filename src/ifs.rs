use crate::xforms::Transform;
use crate::choosers::Chooser;

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
}
