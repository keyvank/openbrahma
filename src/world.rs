use super::shape::{Intersection, Ray, Shape, Transform};
use super::vector::Vector;

#[derive(Debug)]
pub enum Sense {
    Vision(Vec<Option<Intersection>>),
}

#[derive(Debug)]
pub enum Action {
    Move(Transform),
}

pub trait Sensor {
    fn sense(&self, w: &World) -> Sense;
}

pub struct Eye {
    ray: Ray,
    fov: f64,
    res: usize,
}
impl Sensor for Eye {
    fn sense(&self, w: &World) -> Sense {
        let mut view = Vec::new();
        for i in -(self.res as i32 / 2)..(self.res as i32 / 2) {
            let ray = Ray {
                pos: self.ray.pos,
                ang: self.ray.ang + self.fov * (i as f64) / (self.res as f64),
            };
            view.push(
                w.objects
                    .iter()
                    .map(|obj| obj.body.shape().intersects(&obj.trans, &ray))
                    .filter_map(|opt| opt)
                    .min_by(|a, b| {
                        a.dist
                            .partial_cmp(&b.dist)
                            .unwrap_or(std::cmp::Ordering::Equal)
                    }),
            );
        }
        Sense::Vision(view)
    }
}

pub trait Updatable {
    fn shape(&self) -> &dyn Shape;
    fn sensors(&self) -> Vec<Box<dyn Sensor>>;
    fn update(&mut self, senses: &Vec<Sense>) -> Vec<Action>;
}

pub struct Object {
    body: Box<dyn Updatable>,
    trans: Transform,
}

impl Object {
    pub fn new(body: Box<dyn Updatable>, trans: Transform) -> Object {
        Object { body, trans }
    }
}

pub struct World {
    objects: Vec<Object>,
}

impl World {
    pub fn new() -> World {
        World {
            objects: Vec::new(),
        }
    }
    pub fn add_object(&mut self, o: Object) {
        self.objects.push(o);
    }
    pub fn update(&mut self) {
        let senses = self
            .objects
            .iter()
            .map(|obj| {
                obj.body
                    .sensors()
                    .iter()
                    .map(|s| s.sense(&self))
                    .collect::<Vec<Sense>>()
            })
            .collect::<Vec<_>>();

        self.objects
            .iter_mut()
            .zip(senses.iter())
            .for_each(|(obj, senses)| {
                for act in obj.body.update(senses) {
                    match act {
                        Action::Move(t) => {}
                    }
                }
            });
    }
}
