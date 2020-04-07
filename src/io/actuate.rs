use crate::geometry::Transform;
use crate::{Object, ObjectId, Updatable, World};

pub enum Action {
    Create(Transform, Box<dyn Updatable>),
    Delete(ObjectId),
    Update(ObjectId, Box<dyn FnOnce(&mut Object)>),
}

pub trait Actuator {
    fn actuate(&self, u: &Object, w: &World) -> Vec<Action>;
}

pub struct Move {
    pub trans: Transform,
}
impl Actuator for Move {
    fn actuate(&self, u: &Object, w: &World) -> Vec<Action> {
        let trans = self.trans.transform(u.trans);
        vec![Action::Update(
            u.id,
            Box::new(move |o: &mut Object| {
                o.trans = trans;
            }),
        )]
    }
}
