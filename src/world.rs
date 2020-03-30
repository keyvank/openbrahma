use super::shape::{Intersection, Ray, Shape, Transform};
use super::vector::Vector;

#[derive(Debug)]
pub enum Sense {}

#[derive(Debug)]
pub enum Action {
    Move(Transform),
}

pub trait Updatable {
    fn shape(&self) -> &dyn Shape;
    fn update(&mut self, senses: Vec<Sense>) -> Vec<Action>;
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
    fn see(&mut self, r: Ray, fov: f64, res: i32) -> Vec<Option<Intersection>> {
        let mut view = Vec::new();
        for i in -(res / 2)..(res / 2) {
            let ray = Ray {
                pos: r.pos,
                ang: r.ang + fov * (i as f64) / (res as f64),
            };
            view.push(
                self.objects
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
        view
    }
    pub fn update(&mut self) {
        for obj in self.objects.iter_mut() {
            for act in obj.body.update(Vec::new()) {
                match act {
                    Action::Move(t) => {}
                }
            }
        }
    }
}
