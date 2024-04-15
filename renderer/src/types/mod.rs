pub mod buf;
pub mod color;
pub mod vec;

use self::vec::{CompleteVector, Vector};

// An origin point with a vector for the direction
pub struct Ray<Vec> {
    origin: Vec,
    direction: Vec,
}

impl<Vec: CompleteVector<Vec>> Ray<Vec> {
    pub const fn new(origin: Vec, direction: Vec) -> Self {
        Self { origin, direction }
    }
    pub const fn origin(&self) -> &Vec {
        &self.origin
    }
    pub const fn direction(&self) -> &Vec {
        &self.direction
    }
    pub fn at(&self, t: <Vec as Vector>::Num) -> Vec {
        self.direction().clone() * t + self.origin().clone()
    }
}

pub trait CreateRay: Sized {
    fn to(self, target: Self) -> Ray<Self>;
}

impl<V: CompleteVector<V>> CreateRay for V {
    fn to(self, target: Self) -> Ray<Self> {
        let dir = target - self.clone();
        Ray::new(self, dir)
    }
}
