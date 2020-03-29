use super::shape::Shape;
use super::world::Object;

pub struct Food<S: Shape> {
    body: S,
    health: u32,
}

impl<S: Shape> Object for Food<S> {}
