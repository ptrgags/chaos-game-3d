pub trait InitialSet {
    fn generate_set(&self) -> Buffer; 
}

pub struct Box {
    center: Vec3,
    dimensions: Vec3,
    color: Vec3,
}
