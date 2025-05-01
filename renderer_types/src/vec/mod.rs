use std::ops::{Add, AddAssign, Div, Mul, MulAssign, Neg, Sub, SubAssign};

use num_traits::{real::Real, Float, MulAdd, One};
use renderer_macros::swizzle;

mod vec2;
mod vec3;
mod vec4;
pub use vec2::*;
pub use vec3::*;
pub use vec4::*;

#[allow(clippy::len_without_is_empty)]
pub trait Vector {
    type Num;

    /// Returns a vector with every component set to the value
    fn splat(val: Self::Num) -> Self
    where
        Self::Num: Clone;
    /// \|vec\|^2
    fn len_squared(&self) -> Self::Num
    where
        Self::Num: Real;
    /// \|vec\|
    fn len(&self) -> Self::Num
    where
        Self::Num: Real,
    {
        self.len_squared().sqrt()
    }
    /// Returns a normalized vector
    ///
    /// Modifies the vector such as vec.len() = 1, but the direction is kept constant
    ///
    /// Returns a zero vector when provided with a zero vector
    fn unit(self) -> Self
    where
        Self::Num: Real;

    /// Returns a new vector reflected against normal
    ///
    /// Normal needs to be a unit vector
    fn reflect(self, normal: Self) -> Self
    where
        Self: Sized + Mul<Self::Num, Output = Self> + Sub<Self, Output = Self> + Clone,
        Self::Num: Float
            + Sized
            + One
            + MulAdd<Self::Num, Output = Self::Num>
            + Mul<Self::Num, Output = Self::Num>,
    {
        self.clone() - normal.clone() * ((Self::Num::one() + Self::Num::one()) * self.dot(normal))
    }
    fn dot(self, rhs: Self) -> Self::Num
    where
        Self::Num: MulAdd<Self::Num, Output = Self::Num> + Mul<Self::Num, Output = Self::Num>;

    /// Returns the angle between the vectors in radians
    fn angle_to(&self, rhs: &Self) -> Self::Num
    where
        Self: Sized + Clone,
        Self::Num: Float + MulAdd<Self::Num, Output = Self::Num>,
    {
        num_traits::Float::acos(self.clone().dot(rhs.clone()) / (self.len() * rhs.len()))
    }

    fn map(mut self, cb: impl FnMut(&mut Self::Num)) -> Self
    where
        Self: Sized,
    {
        self.components().for_each(cb);
        self
    }
    fn components(&mut self) -> impl Iterator<Item = &mut Self::Num>;
}

pub trait CompleteVector<Rhs>:
    Sized
    + Clone
    + Vector
    + Neg
    + Add<Rhs, Output = Self>
    + Sub<Rhs, Output = Self>
    + Sub<Rhs, Output = Self>
    + Mul<<Self as Vector>::Num, Output = Self>
    + Div<<Self as Vector>::Num, Output = Self>
    + AddAssign<Rhs>
    + SubAssign<Rhs>
    + MulAssign<<Self as Vector>::Num>
    + Div<<Self as Vector>::Num, Output = Self>
{
}

pub trait IntoVector<T> {
    type Vector;

    fn into_vector(self) -> Self::Vector;
}
impl<T> IntoVector<()> for (T, T) {
    type Vector = Vec2<T>;

    fn into_vector(self) -> Self::Vector {
        let (x, y) = self;
        Self::Vector { x, y }
    }
}
impl<T> IntoVector<()> for (T, T, T) {
    type Vector = Vec3<T>;

    fn into_vector(self) -> Self::Vector {
        let (x, y, z) = self;
        Self::Vector { x, y, z }
    }
}
impl<T> IntoVector<()> for (T, T, T, T) {
    type Vector = Vec4<T>;

    fn into_vector(self) -> Self::Vector {
        let (x, y, z, w) = self;
        Self::Vector { x, y, z, w }
    }
}

macro_rules! implement_swizzle {
    ($type:ident, $combined:ident, $x:ident, $y:ident) => {
        pub fn $combined(&self) -> Vec2<$type> {
            Vec2 {
                x: self.$x.clone(),
                y: self.$y.clone(),
            }
        }
    };
    ($type:ident, $combined:ident, $x:ident, $y:ident, $z:ident) => {
        pub fn $combined(&self) -> Vec3<$type> {
            Vec3 {
                x: self.$x.clone(),
                y: self.$y.clone(),
                z: self.$z.clone(),
            }
        }
    };
    ($type:ident, $combined:ident, $x:ident, $y:ident, $z:ident, $w:ident) => {
        pub fn $combined(&self) -> Vec4<$type> {
            Vec4 {
                x: self.$x.clone(),
                y: self.$y.clone(),
                z: self.$z.clone(),
                w: self.$w.clone(),
            }
        }
    };
}
impl<T: Clone> Vec2<T> {
    // These provide methods such as .xy to extract a group of values into a new vec
    swizzle!([x, y] 2 prefix "implement_swizzle!(T," suffix ");");
    swizzle!([x, y] 3 prefix "implement_swizzle!(T," suffix ");");
    swizzle!([x, y] 4 prefix "implement_swizzle!(T," suffix ");");
}
impl<T: Clone> Vec3<T> {
    // These provide methods such as .xyz to extract a group of values into a new vec
    swizzle!([x, y, z] 2 prefix "implement_swizzle!(T," suffix ");");
    swizzle!([x, y, z] 3 prefix "implement_swizzle!(T," suffix ");");
    swizzle!([x, y, z] 4 prefix "implement_swizzle!(T," suffix ");");
}
impl<T: Clone> Vec4<T> {
    // These provide methods such as .xyzw to extract a group of values into a new vec
    swizzle!([x, y, z, w] 2 prefix "implement_swizzle!(T," suffix ");");
    swizzle!([x, y, z, w] 3 prefix "implement_swizzle!(T," suffix ");");
    swizzle!([x, y, z, w] 4 prefix "implement_swizzle!(T," suffix ");");
}
