use serde::{Deserialize, Serialize};
use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Vector(pub f64, pub f64);

impl Vector {
    pub fn dot(self, other: Self) -> f64 {
        self.0 * other.0 + self.1 * other.1
    }
    pub fn len(self) -> f64 {
        self.dot(self).sqrt()
    }
    pub fn norm(self) -> Self {
        self / self.len()
    }
    pub fn rotate(self, ang: f64) -> Self {
        let cos = ang.cos();
        let sin = ang.sin();
        Self(self.0 * cos - self.1 * sin, self.0 * sin + self.1 * cos)
    }
    pub fn zero() -> Self {
        Vector(0.0, 0.0)
    }
    pub fn i() -> Self {
        Vector(1.0, 0.0)
    }
    pub fn j() -> Self {
        Vector(0.0, 1.0)
    }
}

impl Add<Vector> for Vector {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0, self.1 + other.1)
    }
}

impl Sub<Vector> for Vector {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self(self.0 - other.0, self.1 - other.1)
    }
}

impl Mul<f64> for Vector {
    type Output = Self;
    fn mul(self, other: f64) -> Self {
        Self(self.0 * other, self.1 * other)
    }
}

impl Div<f64> for Vector {
    type Output = Self;
    fn div(self, other: f64) -> Self {
        Self(self.0 / other, self.1 / other)
    }
}
