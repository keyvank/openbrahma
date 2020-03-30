use super::shape::Shape;
use super::world::{Action, Updatable, World};

pub struct Food<S: Shape> {
    body: S,
    health: u32,
}

impl<S: Shape> Updatable for Food<S> {
    fn update(&mut self) -> Vec<Action> {
        Vec::new()
    }
}
