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
    fn actuate(&self, u: &Object, _w: &World) -> Vec<Action> {
        let trans = self.trans.transform(u.trans);
        vec![Action::Update(
            u.id,
            Box::new(move |o: &mut Object| {
                o.trans = trans;
            }),
        )]
    }
}

pub struct Eat {
    pub id: ObjectId,
}

impl Actuator for Eat {
    fn actuate(&self, u: &Object, w: &World) -> Vec<Action> {
        if let Some(food) = w.objects.get(&self.id) {
            let boundary =
                food.body.shape().bounding_circle().r + u.body.shape().bounding_circle().r;
            if (food.trans.trans - u.trans.trans).len() < boundary {
                return vec![
                    Action::Delete(self.id),
                    Action::Update(
                        u.id,
                        Box::new(|_o| {
                            // o.health += self.health
                        }),
                    ),
                ];
            }
        }
        Vec::new()
    }
}
