use super::shape::{Intersection, Ray};
use super::world::World;

#[derive(Debug)]
pub enum Sense {
    Vision(Vec<Option<Intersection>>),
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
