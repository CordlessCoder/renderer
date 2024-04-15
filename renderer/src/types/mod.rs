pub mod buf;
pub mod vec;

use std::ops::{Add, Mul};

use crate::prelude::*;

use self::vec::{CompleteVector, Vec3, Vector};

// An origin point with a unit vector for the direction
pub struct Ray<Vec> {
    origin: Vec,
    direction: Vec,
}

impl<Vec: CompleteVector<Vec>> Ray<Vec> {
    pub fn new(origin: Vec, direction: Vec) -> Self {
        Self { origin, direction }
    }
    pub fn origin(&self) -> &Vec {
        &self.origin
    }
    pub fn direction(&self) -> &Vec {
        &self.direction
    }
    pub fn at(&self, t: <Vec as Vector>::Num) -> Vec {
        self.direction().clone() * t + self.origin().clone()
    }
}
fn test() {
    Ray::new(vec2f(0., 0.), vec2f(1., 0.));
}
