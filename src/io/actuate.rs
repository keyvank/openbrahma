use crate::geometry::Transform;
use crate::{Object, Updatable, World};

pub enum Action {
    Create(Transform, Box<dyn Updatable>),
    Delete(usize),
    Put(usize, Transform),
}

pub trait Actuator {
    fn actuate(&self, u: &Object, w: &World) -> Vec<Action>;
}

pub struct Move {
    pub trans: Transform,
}
impl Actuator for Move {
    fn actuate(&self, u: &Object, w: &World) -> Vec<Action> {
        vec![Action::Put(u.id, self.trans.transform(u.trans))]
    }
}
