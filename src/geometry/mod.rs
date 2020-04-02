mod shape;
mod vector;

pub use shape::*;
pub use vector::*;

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
