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
impl Ray {
    pub fn transform(&self, t: Transform) -> Self {
        Ray {
            pos: self.pos.rotate(t.rot) + t.trans,
            ang: self.ang + t.rot,
        }
    }
}

use std::ops::Add;
#[derive(Debug, Clone, Copy)]
pub struct Transform {
    pub trans: Vector, // Translate
    pub rot: f64,      // Rotate
}
impl Add<Transform> for Transform {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            trans: self.trans + other.trans,
            rot: self.rot + other.rot,
        }
    }
}
impl Transform {
    pub fn transform(&self, t: Self) -> Self {
        Self {
            trans: self.trans.rotate(t.rot) + t.trans,
            rot: self.rot + t.rot,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Intersection {
    pub dist: f64,
    pub col: Color,
}
