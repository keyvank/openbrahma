use super::shape::Shape;
use super::vector::Vector;

pub enum Action {
    Move(Vector),
}

pub trait Updatable {
    fn update(&mut self) -> Vec<Action>;
}

pub struct World {
    objects: Vec<Box<dyn Updatable>>,
}

impl World {
    pub fn new() -> World {
        World {
            objects: Vec::new(),
        }
    }
    pub fn add_object(&mut self, o: Box<dyn Updatable>) {
        self.objects.push(o);
    }
    pub fn update(&mut self) {
        for obj in self.objects.iter_mut() {
            obj.update();
        }
    }
}
