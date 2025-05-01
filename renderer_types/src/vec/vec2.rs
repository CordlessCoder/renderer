use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use super::{CompleteVector, Vector};
use bytemuck::Zeroable;
use num_traits::{real::Real, AsPrimitive, MulAdd, Zero};

impl<T: Zero> Zero for Vec2<T> {
    fn zero() -> Self {
        Self {
            x: T::zero(),
            y: T::zero(),
        }
    }
    fn is_zero(&self) -> bool {
        self.x.is_zero() && self.y.is_zero()
    }
}
impl<T: Zero> Default for Vec2<T> {
    fn default() -> Self {
        Self::zero()
    }
}
impl<
        V: Into<Vec2<T>>,
        T: Neg<Output = T>
            + Add<T, Output = T>
            + Sub<T, Output = T>
            + Mul<T, Output = T>
            + Div<T, Output = T>
            + AddAssign<T>
            + SubAssign<T>
            + MulAssign<T>
            + Clone,
    > CompleteVector<V> for Vec2<T>
{
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Zeroable)]
#[repr(C)]
pub struct Vec2<T = f32> {
    pub x: T,
    pub y: T,
}
impl<T: Copy> Copy for Vec2<T> {}
impl<T> Vec2<T> {
    pub const fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
    pub fn fromprim(x: impl AsPrimitive<T>, y: impl AsPrimitive<T>) -> Self
    where
        T: Copy + 'static,
    {
        Self {
            x: x.as_(),
            y: y.as_(),
        }
    }
    pub fn cast<U>(self) -> Vec2<U>
    where
        T: Into<U>,
    {
        let Self { x, y } = self;
        Vec2 {
            x: x.into(),
            y: y.into(),
        }
    }
}
impl<T, T1: Into<T>, T2: Into<T>> From<(T1, T2)> for Vec2<T> {
    fn from((x, y): (T1, T2)) -> Self {
        Self::new(x.into(), y.into())
    }
}
impl<T, U: Into<T>> From<[U; 2]> for Vec2<T> {
    fn from([x, y]: [U; 2]) -> Self {
        Self::new(x.into(), y.into())
    }
}
impl<T> Vector for Vec2<T> {
    type Num = T;

    fn splat(val: T) -> Self
    where
        T: Clone,
    {
        Self {
            x: val.clone(),
            y: val,
        }
    }
    fn len_squared(&self) -> Self::Num
    where
        Self::Num: Real,
    {
        let Self { x, y } = *self;
        x.mul_add(x, y * y)
    }
    fn unit(self) -> Self
    where
        Self::Num: Real,
    {
        let len = self.len();
        if len.is_zero() {
            return self;
        }
        self / len
    }
    fn dot(self, rhs: Self) -> Self::Num
    where
        Self::Num: MulAdd<Self::Num, Output = Self::Num> + Mul<Self::Num, Output = Self::Num>,
    {
        self.x.mul_add(rhs.x, self.y * rhs.y)
    }
    fn components(&mut self) -> impl Iterator<Item = &mut Self::Num> {
        let Self { x, y } = self;
        [x, y].into_iter()
    }
}
impl<T: std::ops::Neg<Output = T>> Neg for Vec2<T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        let Self { x, y } = self;
        Self { x: -x, y: -y }
    }
}
impl<T: Add<T, Output = T>, V: Into<Vec2<T>>> Add<V> for Vec2<T> {
    type Output = Self;

    fn add(self, rhs: V) -> Self::Output {
        let rhs = rhs.into();
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
impl<T: AddAssign, V: Into<Vec2<T>>> AddAssign<V> for Vec2<T> {
    fn add_assign(&mut self, rhs: V) {
        let rhs = rhs.into();
        let Self { x, y } = self;
        *x += rhs.x;
        *y += rhs.y;
    }
}
impl<T: Sub<T, Output = T>, V: Into<Vec2<T>>> Sub<V> for Vec2<T> {
    type Output = Self;

    fn sub(self, rhs: V) -> Self::Output {
        let rhs = rhs.into();
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
impl<T: SubAssign, V: Into<Vec2<T>>> SubAssign<V> for Vec2<T> {
    fn sub_assign(&mut self, rhs: V) {
        let rhs = rhs.into();
        let Self { x, y } = self;
        *x -= rhs.x;
        *y -= rhs.y;
    }
}
impl<T: Mul<T, Output = T> + Clone> Mul<T> for Vec2<T> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self {
            x: self.x * rhs.clone(),
            y: self.y * rhs,
        }
    }
}
impl<T: MulAssign + Clone> MulAssign<T> for Vec2<T> {
    fn mul_assign(&mut self, rhs: T) {
        let Self { x, y } = self;
        *x *= rhs.clone();
        *y *= rhs;
    }
}
impl<T: Clone + Div<T, Output = T>> Div<T> for Vec2<T> {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Self {
            x: self.x / rhs.clone(),
            y: self.y / rhs,
        }
    }
}
impl<T: Clone + DivAssign> DivAssign<T> for Vec2<T> {
    fn div_assign(&mut self, rhs: T) {
        let Self { x, y } = self;
        *x /= rhs.clone();
        *y /= rhs;
    }
}
