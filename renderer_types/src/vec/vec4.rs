use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use bytemuck::Zeroable;
use num_traits::{real::Real, AsPrimitive, MulAdd, Zero};

use super::{CompleteVector, Vector};

impl<T: Zero> Zero for Vec4<T> {
    fn zero() -> Self {
        Self {
            x: T::zero(),
            y: T::zero(),
            z: T::zero(),
            w: T::zero(),
        }
    }
    fn is_zero(&self) -> bool {
        self.x.is_zero() && self.y.is_zero() && self.z.is_zero() && self.w.is_zero()
    }
}
impl<T: Zero> Default for Vec4<T> {
    fn default() -> Self {
        Self::zero()
    }
}
impl<
        V: Into<Vec4<T>>,
        T: Neg<Output = T>
            + Add<T, Output = T>
            + Sub<T, Output = T>
            + Mul<T, Output = T>
            + Div<T, Output = T>
            + AddAssign<T>
            + SubAssign<T>
            + MulAssign<T>
            + Clone,
    > CompleteVector<V> for Vec4<T>
{
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Zeroable)]
#[repr(C)]
pub struct Vec4<T = f32> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}
impl<T: Copy> Copy for Vec4<T> {}

impl<T> Vec4<T> {
    pub const fn new(x: T, y: T, z: T, w: T) -> Self {
        Self { x, y, z, w }
    }
    pub fn fromprim(
        x: impl AsPrimitive<T>,
        y: impl AsPrimitive<T>,
        z: impl AsPrimitive<T>,
        w: impl AsPrimitive<T>,
    ) -> Self
    where
        T: Copy + 'static,
    {
        Self {
            x: x.as_(),
            y: y.as_(),
            z: z.as_(),
            w: w.as_(),
        }
    }
    pub fn cast<U>(self) -> Vec4<U>
    where
        T: Into<U>,
    {
        let Self { x, y, z, w } = self;
        Vec4 {
            x: x.into(),
            y: y.into(),
            z: z.into(),
            w: w.into(),
        }
    }
}
impl<T, T1: Into<T>, T2: Into<T>, T3: Into<T>, T4: Into<T>> From<(T1, T2, T3, T4)> for Vec4<T> {
    fn from((x, y, z, w): (T1, T2, T3, T4)) -> Self {
        Self::new(x.into(), y.into(), z.into(), w.into())
    }
}
impl<T, U: Into<T>> From<[U; 4]> for Vec4<T> {
    fn from([x, y, z, w]: [U; 4]) -> Self {
        Self::new(x.into(), y.into(), z.into(), w.into())
    }
}
impl<T> Vector for Vec4<T> {
    type Num = T;

    fn splat(val: T) -> Self
    where
        T: Clone,
    {
        Self {
            x: val.clone(),
            y: val.clone(),
            z: val.clone(),
            w: val,
        }
    }
    fn len_squared(&self) -> Self::Num
    where
        Self::Num: Real,
    {
        let Self { x, y, z, w } = *self;
        x.mul_add(x, y.mul_add(y, z.mul_add(z, w * w)))
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
        self.x.mul_add(
            rhs.x,
            self.y.mul_add(rhs.y, self.z.mul_add(rhs.z, self.w * rhs.w)),
        )
    }
    fn components(&mut self) -> impl Iterator<Item = &mut Self::Num> {
        let Self { x, y, z, w } = self;
        [x, y, z, w].into_iter()
    }
}
impl<T: Neg<Output = T>> Neg for Vec4<T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        let Self { x, y, z, w } = self;
        Self {
            x: -x,
            y: -y,
            z: -z,
            w: -w,
        }
    }
}
impl<T: Add<T, Output = T>, V: Into<Vec4<T>>> Add<V> for Vec4<T> {
    type Output = Self;

    fn add(self, rhs: V) -> Self::Output {
        let rhs = rhs.into();
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w,
        }
    }
}
impl<T: AddAssign, V: Into<Vec4<T>>> AddAssign<V> for Vec4<T> {
    fn add_assign(&mut self, rhs: V) {
        let rhs = rhs.into();
        let Self { x, y, z, w } = self;
        *x += rhs.x;
        *y += rhs.y;
        *z += rhs.z;
        *w += rhs.w;
    }
}
impl<T: Sub<T, Output = T>, V: Into<Vec4<T>>> Sub<V> for Vec4<T> {
    type Output = Self;

    fn sub(self, rhs: V) -> Self::Output {
        let rhs = rhs.into();
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w,
        }
    }
}
impl<T: SubAssign, V: Into<Vec4<T>>> SubAssign<V> for Vec4<T> {
    fn sub_assign(&mut self, rhs: V) {
        let rhs = rhs.into();
        let Self { x, y, z, w } = self;
        *x -= rhs.x;
        *y -= rhs.y;
        *z -= rhs.z;
        *w -= rhs.w;
    }
}
impl<T: Mul<T, Output = T> + Clone> Mul<T> for Vec4<T> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self {
            x: self.x * rhs.clone(),
            y: self.y * rhs.clone(),
            z: self.z * rhs.clone(),
            w: self.w * rhs,
        }
    }
}
impl<T: MulAssign + Clone> MulAssign<T> for Vec4<T> {
    fn mul_assign(&mut self, rhs: T) {
        let Self { x, y, z, w } = self;
        *x *= rhs.clone();
        *y *= rhs.clone();
        *z *= rhs.clone();
        *w *= rhs;
    }
}
impl<T: Div<T, Output = T> + Clone> Div<T> for Vec4<T> {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Self {
            x: self.x / rhs.clone(),
            y: self.y / rhs.clone(),
            z: self.z / rhs.clone(),
            w: self.w / rhs,
        }
    }
}
impl<T: DivAssign + Clone> DivAssign<T> for Vec4<T> {
    fn div_assign(&mut self, rhs: T) {
        let Self { x, y, z, w } = self;
        *x /= rhs.clone();
        *y /= rhs.clone();
        *z /= rhs.clone();
        *w /= rhs;
    }
}
