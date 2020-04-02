use super::vector::Vector;

#[derive(Debug, Clone)]
pub struct Color(pub f64);

#[derive(Debug, Clone)]
pub struct Ray {
    pub pos: Vector,
    pub ang: f64,
}

#[derive(Debug, Clone)]
pub struct Transform {
    pub pos: Vector,
    pub rot: f64,
}

#[derive(Debug, Clone)]
pub struct Intersection {
    pub dist: f64,
    pub col: Color,
}

pub trait Shape {
    fn intersects(&self, t: &Transform, r: &Ray) -> Option<Intersection>;
}

#[derive(Debug, Clone)]
pub struct Circle {
    pub r: f64,
    pub col: Color,
}

impl Shape for Circle {
    fn intersects(&self, t: &Transform, r: &Ray) -> Option<Intersection> {
        let ray_to_circ = t.pos - r.pos;
        let dir = Vector(r.ang.cos(), r.ang.sin());
        let proj = ray_to_circ.dot(dir);
        if proj > 0.0 {
            let d2 = ray_to_circ.dot(ray_to_circ) - proj * proj;
            let r2 = self.r * self.r;
            if d2 <= r2 {
                let dist = proj - (r2 - d2).sqrt();
                if dist >= 0.0 {
                    Some(Intersection {
                        dist: dist,
                        col: self.col.clone(),
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
