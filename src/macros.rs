/// to_box(Trait)
/// A lot of my types need to be boxed, so this macro defines a method
/// to_box() that wraps a struct in a Box<dyn Trait>.
macro_rules! to_box {
    ($type:ident) => {
        pub fn to_box(self) -> Box<dyn $type> {
            Box::new(self)
        }
    }
}
