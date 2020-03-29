use super::shape::Shape;

pub trait Object {
    fn tick(&mut self);
}

pub struct World {
    objects: Vec<Box<dyn Object>>,
}

impl World {
    pub fn tick(&mut self) {
        for obj in self.objects.iter_mut() {
            obj.tick();
        }
    }
}
