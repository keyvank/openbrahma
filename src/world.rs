use crate::geometry::{Shape, Transform};
use crate::io::{Action, Actuator, Sense, Sensor};
use std::any::Any;
use std::collections::HashMap;

pub type ObjectId = usize;

pub trait Corpus {
    fn shape(&self) -> &Box<dyn Shape>;
    fn sensors(&self) -> Vec<Box<dyn Sensor>>;
    fn update(&mut self, senses: &Vec<Sense>) -> Vec<Box<dyn Actuator>>;
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

pub struct Object {
    pub id: ObjectId,
    pub body: Box<dyn Corpus>,
    pub trans: Transform,
}

impl Object {
    pub fn new(id: ObjectId, body: Box<dyn Corpus>, trans: Transform) -> Object {
        Object { id, body, trans }
    }
    pub fn intersects(&self, other: &Object) -> bool {
        let boundary =
            self.body.shape().bounding_circle().r + other.body.shape().bounding_circle().r;
        (self.trans.trans - other.trans.trans).len() < boundary
    }
}

pub struct World {
    pub objects: HashMap<ObjectId, Object>,
    pub next_id: ObjectId,
}

impl World {
    pub fn new() -> World {
        World {
            objects: HashMap::new(),
            next_id: 0,
        }
    }
    pub fn add_object(&mut self, body: Box<dyn Corpus>, trans: Transform) {
        self.objects
            .insert(self.next_id, Object::new(self.next_id, body, trans));
        self.next_id += 1;
    }
    pub fn update(&mut self) {
        let senses = self
            .objects
            .iter()
            .map(|(_, obj)| {
                obj.body
                    .sensors()
                    .iter()
                    .map(|s| s.sense(&obj, &self))
                    .collect::<Vec<Sense>>()
            })
            .collect::<Vec<_>>();

        let actuators = self
            .objects
            .iter_mut()
            .zip(senses.iter())
            .map(|((_, obj), senses)| obj.body.update(senses))
            .collect::<Vec<_>>();

        let actions = self
            .objects
            .iter()
            .zip(actuators.iter())
            .map(|((_, obj), actuators)| {
                actuators
                    .iter()
                    .map(|a| a.actuate(&obj, &self))
                    .flatten()
                    .collect::<Vec<_>>()
            })
            .flatten()
            .collect::<Vec<_>>();

        for act in actions {
            match act {
                Action::Delete(id) => {
                    self.objects.remove(&id).unwrap();
                }
                Action::Create(body, trans) => {
                    self.add_object(trans, body);
                }
                Action::Update(id, f) => f(self.objects.get_mut(&id).unwrap()),
            }
        }
    }
}
