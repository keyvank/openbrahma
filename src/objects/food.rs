use crate::geometry::Shape;
use crate::io::{Actuator, Sense, Sensor};
use crate::Corpus;
use serde::{Deserialize, Serialize};
use std::any::Any;

#[derive(Clone, Serialize, Deserialize)]
pub struct Food {
    body: Shape,
    pub health: u32,
}

impl Food {
    pub fn new(health: u32, body: Shape) -> Food {
        Food { health, body }
    }
}

impl Corpus for Food {
    fn update(&mut self, _senses: &Vec<Sense>) -> Vec<Box<dyn Actuator>> {
        Vec::new()
    }
    fn shape(&self) -> &Shape {
        &self.body
    }
    fn sensors(&self) -> Vec<Box<dyn Sensor>> {
        Vec::new()
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
