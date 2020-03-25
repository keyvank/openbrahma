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
        let pc = self.pos - r.pos;
        let tca = pc.dot(r.dir);
        if tca > 0.0 {
            let d2 = pc.dot(pc) - tca * tca;
            let r2 = self.r * self.r;
            if d2 <= r2 {
                let dist = tca - (r2 - d2).sqrt();
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
