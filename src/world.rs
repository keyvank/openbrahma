use super::vector::Vector;

struct Color(f64);

struct Ray {
    pub pos: Vector,
    pub dir: Vector,
}

struct Intersection {
    pub dist: f64,
    pub norm: Vector,
    pub col: Color,
}

trait Object {
    fn intersects(r: Ray) -> Option<Intersection>;
}

struct Circle {
    pub pos: Vector,
    pub r: f64,
}

impl Object for Circle {
    fn intersects(r: Ray) -> Option<Intersection> {
        None
    }
}
