use num_traits::Float;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use super::buf::Rgba;

#[derive(Debug, Clone, Copy, Default)]
pub struct Color<T: Float> {
    pub r: T,
    pub g: T,
    pub b: T,
}

impl<T: Float> Color<T> {
    pub fn new(r: T, g: T, b: T) -> Self {
        let in_range = |v| v >= T::zero() && v <= T::one();
        debug_assert!(in_range(r));
        debug_assert!(in_range(g));
        debug_assert!(in_range(b));
        Self { r, g, b }
    }
    pub fn clamp(mut self) -> Self {
        let clamp = |v: T| v.min(T::one()).max(T::zero());
        self.r = clamp(self.r);
        self.g = clamp(self.g);
        self.b = clamp(self.b);
        self
    }
}

impl<T: Float> Color<T> {
    pub fn into_rgba(self) -> Rgba {
        let max = T::from(256).unwrap();
        let [r, g, b] = [self.r, self.g, self.b]
            .map(|v| v * max)
            .map(|c| c.to_u8().unwrap_or(255));
        Rgba::new(r, g, b, 255)
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
