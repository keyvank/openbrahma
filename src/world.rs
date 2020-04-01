use super::sense::{Sense, Sensor};
use super::shape::{Shape, Transform};

#[derive(Debug)]
pub enum Action {
    Move(Transform),
}

pub trait Updatable {
    fn shape(&self) -> &dyn Shape;
    fn sensors(&self) -> Vec<Box<dyn Sensor>>;
    fn update(&mut self, senses: &Vec<Sense>) -> Vec<Action>;
}

pub struct Object {
    pub body: Box<dyn Updatable>,
    pub trans: Transform,
}

impl Object {
    pub fn new(body: Box<dyn Updatable>, trans: Transform) -> Object {
        Object { body, trans }
    }
}

pub struct World {
    pub objects: Vec<Object>,
}

impl World {
    pub fn new() -> World {
        World {
            objects: Vec::new(),
        }
    }
    pub fn add_object(&mut self, o: Object) {
        self.objects.push(o);
    }
    pub fn update(&mut self) {
        let senses = self
            .objects
            .iter()
            .map(|obj| {
                obj.body
                    .sensors()
                    .iter()
                    .map(|s| s.sense(&obj, &self))
                    .collect::<Vec<Sense>>()
            })
            .collect::<Vec<_>>();

        self.objects
            .iter_mut()
            .zip(senses.iter())
            .for_each(|(obj, senses)| {
                for act in obj.body.update(senses) {
                    match act {
                        Action::Move(t) => {}
                    }
                }
            });
    }
}
