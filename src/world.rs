use super::shape::{Shape, Transform};
use super::vector::Vector;

#[derive(Debug)]
pub enum Sense {}

#[derive(Debug)]
pub enum Action {
    Move(Transform),
}

pub trait Updatable {
    fn update(&mut self, senses: Vec<Sense>) -> Vec<Action>;
}

pub struct Object {
    body: Box<dyn Updatable>,
    trans: Transform,
}

impl Object {
    pub fn new(body: Box<dyn Updatable>, trans: Transform) -> Object {
        Object { body, trans }
    }
}

pub struct World {
    objects: Vec<Object>,
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
        for obj in self.objects.iter_mut() {
            for act in obj.body.update(Vec::new()) {
                match act {
                    Action::Move(t) => {}
                }
            }
        }
    }
}
