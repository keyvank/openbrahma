mod shape;
mod vector;

pub use shape::*;
pub use vector::*;

#[derive(Debug, Clone, Copy)]
pub struct Color(pub f64);

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub pos: Vector,
    pub ang: f64,
}

use std::ops::Add;
#[derive(Debug, Clone, Copy)]
pub struct Transform {
    pub pos: Vector,
    pub rot: f64,
}
impl Add<Transform> for Transform {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            pos: self.pos + other.pos,
            rot: self.rot + other.rot,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Intersection {
    pub dist: f64,
    pub col: Color,
}
