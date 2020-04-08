use crate::geometry::Transform;
use crate::objects::{Creature, Food};
use crate::{Corpus, Object, ObjectId, World};

pub enum Action {
    Create(Transform, Box<dyn Corpus>),
    Delete(ObjectId),
    Update(ObjectId, Box<dyn FnOnce(&mut Object)>),
    Terminate,
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
        if let Some(food_obj) = w.objects.get(&self.id) {
            if let Some(food) = food_obj.body.as_any().downcast_ref::<Food>() {
                let food_health = food.health;
                if u.intersects(food_obj) {
                    return vec![
                        Action::Delete(self.id),
                        Action::Update(
                            u.id,
                            Box::new(move |o| {
                                if let Some(c) = o.body.as_any_mut().downcast_mut::<Creature>() {
                                    c.health += food_health;
                                };
                            }),
                        ),
                    ];
                }
            }
        }
        Vec::new()
    }
}

pub struct Die;
impl Actuator for Die {
    fn actuate(&self, u: &Object, _w: &World) -> Vec<Action> {
        vec![Action::Delete(u.id)]
    }
}

pub struct End;
impl Actuator for End {
    fn actuate(&self, _u: &Object, _w: &World) -> Vec<Action> {
        vec![Action::Terminate]
    }
}
