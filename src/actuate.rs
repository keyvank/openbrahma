use super::shape::Transform;
use super::world::{Object, Updatable, World};

pub enum Action {
    Create(Transform, Box<dyn Updatable>),
    Delete(usize),
    Put(usize, Transform),
}

pub trait Actuator {
    fn actuate(&self, u: &Object, w: &mut World) -> Vec<Action>;
}

pub struct Move {
    pub trans: Transform,
}
impl Actuator for Move {
    fn actuate(&self, u: &Object, w: &mut World) -> Vec<Action> {
        Vec::new()
    }
}
