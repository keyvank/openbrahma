use serde::{Deserialize, Serialize};

mod shape;
mod vector;

pub use shape::*;
pub use vector::*;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Color(pub f32, pub f32, pub f32);
impl Color {
    pub fn red() -> Color {
        Color(1.0, 0.0, 0.0)
    }
    pub fn green() -> Color {
        Color(0.0, 1.0, 0.0)
    }
    pub fn blue() -> Color {
        Color(0.0, 0.0, 1.0)
    }
    pub fn white() -> Color {
        Color(1.0, 1.0, 1.0)
    }
    pub fn black() -> Color {
        Color(0.0, 0.0, 0.0)
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
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
