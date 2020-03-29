use super::shape::Shape;

pub struct Food<S: Shape> {
    body: S,
    health: u32,
}
