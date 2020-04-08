use crate::geometry::{Intersection, Ray};
use crate::{Object, World};

#[derive(Debug)]
pub enum Sense {
    Vision(Vec<Option<Intersection>>),
}

pub trait Sensor {
    fn sense(&self, u: &Object, w: &World) -> Sense;
}

pub struct Eye {
    pub ray: Ray,
    pub fov: f64,
    pub res: usize,
}
impl Sensor for Eye {
    fn sense(&self, u: &Object, w: &World) -> Sense {
        let ray = self.ray.transform(u.trans);
        let mut view = Vec::new();
        for i in -(self.res as i32 / 2)..(self.res as i32 / 2) {
            let ray = Ray {
                pos: ray.pos,
                ang: ray.ang + self.fov * (i as f64) / (self.res as f64),
            };
            view.push(
                w.objects
                    .iter()
                    .map(|(_, obj)| obj.body.shape().intersects(&obj.trans, &ray))
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
