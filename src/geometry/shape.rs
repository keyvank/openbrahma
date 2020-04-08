use super::vector::Vector;
use super::{Color, Intersection, Ray, Transform};

pub enum Shape {
    Circle { r: f64, col: Color },
}

pub struct BoundingCircle {
    pub r: f64,
    pub col: Color,
}

impl Shape {
    pub fn intersects(&self, t: &Transform, ray: &Ray) -> Option<Intersection> {
        match self {
            Shape::Circle { r, col } => {
                let ray_to_circ = t.trans - ray.pos;
                let dir = Vector(ray.ang.cos(), ray.ang.sin());
                let proj = ray_to_circ.dot(dir);
                if proj > 0.0 {
                    let d2 = ray_to_circ.dot(ray_to_circ) - proj * proj;
                    let r2 = r * r;
                    if d2 <= r2 {
                        let dist = proj - (r2 - d2).sqrt();
                        if dist >= 0.0 {
                            Some(Intersection {
                                dist: dist,
                                col: *col,
                            })
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
        }
    }

    pub fn bounding_circle(&self) -> BoundingCircle {
        match self {
            Shape::Circle { r, col } => BoundingCircle { r: *r, col: *col },
        }
    }
}
