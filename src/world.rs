use super::actuate::{Action, Actuator};
use super::sense::{Sense, Sensor};
use super::shape::{Shape, Transform};
use std::collections::HashMap;

pub trait Updatable {
    fn shape(&self) -> &dyn Shape;
    fn sensors(&self) -> Vec<Box<dyn Sensor>>;
    fn update(&mut self, senses: &Vec<Sense>) -> Vec<Box<dyn Actuator>>;
}

pub struct Object {
    id: usize,
    pub body: Box<dyn Updatable>,
    pub trans: Transform,
}

impl Object {
    pub fn new(id: usize, body: Box<dyn Updatable>, trans: Transform) -> Object {
        Object { id, body, trans }
    }
}

pub struct World {
    pub objects: HashMap<usize, Object>,
    pub next_id: usize,
}

impl World {
    pub fn new() -> World {
        World {
            objects: HashMap::new(),
            next_id: 0,
        }
    }
    pub fn add_object(&mut self, body: Box<dyn Updatable>, trans: Transform) {
        self.objects
            .insert(self.next_id, Object::new(self.next_id, body, trans));
        self.next_id += 1;
    }
    pub fn update(&mut self) {
        let senses = self
            .objects
            .iter()
            .map(|(id, obj)| {
                obj.body
                    .sensors()
                    .iter()
                    .map(|s| s.sense(&obj, &self))
                    .collect::<Vec<Sense>>()
            })
            .collect::<Vec<_>>();

        let actuators = self
            .objects
            .iter_mut()
            .zip(senses.iter())
            .map(|((id, obj), senses)| obj.body.update(senses))
            .collect::<Vec<_>>();

        let actions = self
            .objects
            .iter()
            .zip(actuators.iter())
            .map(|((id, obj), actuators)| {
                actuators
                    .iter()
                    .map(|a| a.actuate(&obj, &self))
                    .collect::<Vec<_>>()
            })
            .flatten()
            .collect::<Vec<_>>();
    }
}
