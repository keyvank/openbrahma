use crate::geometry::Shape;
use crate::io::{Actuator, Sense, Sensor};
use crate::Corpus;

pub struct Food {
    body: Box<dyn Shape>,
    health: u32,
}

impl Food {
    pub fn new(health: u32, body: Box<dyn Shape>) -> Food {
        Food { health, body }
    }
}

impl Corpus for Food {
    fn update(&mut self, _senses: &Vec<Sense>) -> Vec<Box<dyn Actuator>> {
        Vec::new()
    }
    fn shape(&self) -> &Box<dyn Shape> {
        &self.body
    }
    fn sensors(&self) -> Vec<Box<dyn Sensor>> {
        Vec::new()
    }
}
