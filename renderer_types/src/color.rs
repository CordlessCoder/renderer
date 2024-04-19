use num_traits::{AsPrimitive, Float};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use crate::vec::Vec3;

use super::buf::Rgba;

#[derive(Debug, Clone, Copy, Default)]
pub struct Color<T: Float = f32> {
    pub r: T,
    pub g: T,
    pub b: T,
}

impl<T: Float> Color<T> {
    pub fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Self::from(Rgba::new(r, g, b, 255))
    }
    pub const fn new(r: T, g: T, b: T) -> Self {
        Self { r, g, b }
    }
    pub fn fromprim(r: impl AsPrimitive<T>, g: impl AsPrimitive<T>, b: impl AsPrimitive<T>) -> Self
    where
        T: 'static,
    {
        Self::new(r.as_(), g.as_(), b.as_())
    }
    pub fn clamp(mut self) -> Self {
        let clamp = |v: T| v.min(T::one()).max(T::zero());
        self.r = clamp(self.r);
        self.g = clamp(self.g);
        self.b = clamp(self.b);
        self
    }
    pub const fn splat(v: T) -> Self {
        Self { r: v, g: v, b: v }
    }
    pub fn black() -> Self {
        Self::splat(T::zero())
    }
    pub fn white() -> Self {
        Self::splat(T::one())
    }
}

impl<T: Float> Color<T> {
    pub fn into_rgba(self) -> Rgba {
        let max = T::from(255).unwrap();
        let clamp = |v: T| {
            if !v.is_sign_positive() {
                return T::zero();
            }
            if v > T::one() {
                return T::one();
            }
            v
        };
        let [r, g, b] = [self.r, self.g, self.b]
            .map(clamp)
            .map(|v| v * max)
            .map(|c| c.to_u8().unwrap_or(255));
        Rgba::new(r, g, b, 255)
    }
}

impl<T: Float> From<Rgba> for Color<T> {
    fn from(value: Rgba) -> Self {
        let Rgba { b, g, r, a: _ } = value;
        let max = T::from(256).unwrap();
        let [r, g, b] = [r, g, b]
            .map(T::from)
            .map(|c| c.unwrap_or_else(|| T::zero()))
            .map(|v| v / max);
        Self { r, g, b }
    }
}

impl<T: Float> From<Vec3<T>> for Color<T> {
    fn from(value: Vec3<T>) -> Self {
        let Vec3 { x, y, z } = value;
        Self { r: x, g: y, b: z }
    }
}

impl<T: Float> Add<Self> for Color<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        }
    }
}
impl<T: Float> Sub<Self> for Color<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            r: self.r - rhs.r,
            g: self.g - rhs.g,
            b: self.b - rhs.b,
        }
    }
}
impl<T: Float> Div<T> for Color<T> {
    type Output = Self;

    fn div(mut self, rhs: T) -> Self::Output {
        self.r = self.r / rhs;
        self.g = self.g / rhs;
        self.b = self.b / rhs;
        self
    }
}
impl<T: Float> Mul<T> for Color<T> {
    type Output = Self;

    fn mul(mut self, rhs: T) -> Self::Output {
        self.r = self.r * rhs;
        self.g = self.g * rhs;
        self.b = self.b * rhs;
        self
    }
}
impl<T: Float> AddAssign<Self> for Color<T> {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}
impl<T: Float> SubAssign<Self> for Color<T> {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}
impl<T: Float> MulAssign<T> for Color<T> {
    fn mul_assign(&mut self, rhs: T) {
        *self = *self * rhs;
    }
}
impl<T: Float> DivAssign<T> for Color<T> {
    fn div_assign(&mut self, rhs: T) {
        *self = *self / rhs;
    }
}
impl<T: Float> Mul<Color<T>> for Color<T> {
    type Output = Self;

    fn mul(mut self, rhs: Color<T>) -> Self::Output {
        self.r = self.r * rhs.r;
        self.g = self.g * rhs.g;
        self.b = self.b * rhs.b;
        self
    }
}
impl<T: Float> MulAssign<Color<T>> for Color<T> {
    fn mul_assign(&mut self, rhs: Color<T>) {
        *self = *self * rhs;
    }
}
