use super::shape::Shape;

pub trait Object {
    fn tick(&mut self, w: &mut World);
}

pub struct World {
    objects: Vec<Box<dyn Object>>,
}

impl World {
    pub fn tick(&mut self) {}
}
