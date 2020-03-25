use super::vector::Vector;

struct Color(f64);

struct Ray {
    pub pos: Vector,
    pub dir: Vector,
}

struct Intersection {
    pub dist: f64,
    pub col: Color,
}

trait Object {
    fn intersects(&self, r: Ray) -> Option<Intersection>;
}

struct Circle {
    pub pos: Vector,
    pub r: f64,
}

impl Object for Circle {
    fn intersects(&self, r: Ray) -> Option<Intersection> {
        let ray_to_circ = self.pos - r.pos;
        let proj = ray_to_circ.dot(r.dir);
        if proj > 0.0 {
            let d2 = ray_to_circ.dot(ray_to_circ) - proj * proj;
            let r2 = self.r * self.r;
            if d2 <= r2 {
                let dist = proj - (r2 - d2).sqrt();
                if dist >= 0.0 {
                    Some(Intersection {
                        dist: dist,
                        col: Color(1.0),
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
