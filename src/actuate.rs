use super::shape::Transform;
use super::world::{Object, Updatable, World};

pub trait Actuator {
    fn actuate(&self, u: &Object, w: &mut World);
}

pub struct Move {
    pub trans: Transform,
}
impl Actuator for Move {
    fn actuate(&self, u: &Object, w: &mut World) {}
}
