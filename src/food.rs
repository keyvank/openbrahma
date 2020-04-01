use super::shape::Shape;
use super::world::{Action, Sense, Sensor, Updatable, World};

pub struct Food<S: Shape> {
    body: S,
    health: u32,
}

impl<S: Shape> Food<S> {
    pub fn new(health: u32, body: S) -> Food<S> {
        Food { health, body }
    }
}

impl<S: Shape> Updatable for Food<S> {
    fn update(&mut self, senses: Vec<Sense>) -> Vec<Action> {
        Vec::new()
    }
    fn shape(&self) -> &dyn Shape {
        &self.body
    }
    fn sensors(&self) -> Vec<Box<dyn Sensor>> {
        Vec::new()
    }
}
