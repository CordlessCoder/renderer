use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use super::{CompleteVector, Vector};
use bytemuck::Zeroable;
use num_traits::{real::Real, AsPrimitive, MulAdd, Zero};

impl<T: Zero> Zero for Vec3<T> {
    fn zero() -> Self {
        Self {
            x: T::zero(),
            y: T::zero(),
            z: T::zero(),
        }
    }
    fn is_zero(&self) -> bool {
        self.x.is_zero() && self.y.is_zero() && self.z.is_zero()
    }
}
impl<T: Zero> Default for Vec3<T> {
    fn default() -> Self {
        Self::zero()
    }
}
impl<
        V: Into<Vec3<T>>,
        T: Neg<Output = T>
            + Add<T, Output = T>
            + Sub<T, Output = T>
            + Mul<T, Output = T>
            + Div<T, Output = T>
            + AddAssign<T>
            + SubAssign<T>
            + MulAssign<T>
            + Clone,
    > CompleteVector<V> for Vec3<T>
{
}
#[derive(Debug, Clone, PartialEq, PartialOrd, Zeroable)]
#[repr(C)]
pub struct Vec3<T = f32> {
    pub x: T,
    pub y: T,
    pub z: T,
}
impl<T: Copy> Copy for Vec3<T> {}

impl<T> Vec3<T> {
    pub const fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
}
impl<T: Clone> Vec3<T> {
    pub fn fromprim(x: impl AsPrimitive<T>, y: impl AsPrimitive<T>, z: impl AsPrimitive<T>) -> Self
    where
        T: Copy + 'static,
    {
        Self {
            x: x.as_(),
            y: y.as_(),
            z: z.as_(),
        }
    }
    pub fn cast<U>(self) -> Vec3<U>
    where
        T: Into<U>,
    {
        let Self { x, y, z } = self;
        Vec3 {
            x: x.into(),
            y: y.into(),
            z: z.into(),
        }
    }
    pub fn cross(self, rhs: Self) -> Self
    where
        T: Mul<T, Output = T> + Sub<T, Output = T>,
    {
        Self {
            x: self.y.clone() * rhs.z.clone() - self.z.clone() * rhs.y.clone(),
            y: self.z * rhs.x.clone() - self.x.clone() * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }
}
impl<T, T1: Into<T>, T2: Into<T>, T3: Into<T>> From<(T1, T2, T3)> for Vec3<T> {
    fn from((x, y, z): (T1, T2, T3)) -> Self {
        Self::new(x.into(), y.into(), z.into())
    }
}
impl<T, U: Into<T>> From<[U; 3]> for Vec3<T> {
    fn from([x, y, z]: [U; 3]) -> Self {
        Self::new(x.into(), y.into(), z.into())
    }
}
impl<T> Vector for Vec3<T> {
    type Num = T;

    fn splat(val: T) -> Self
    where
        T: Clone,
    {
        Self {
            x: val.clone(),
            y: val.clone(),
            z: val,
        }
    }
    fn len_squared(&self) -> Self::Num
    where
        Self::Num: Real,
    {
        let Self { x, y, z } = *self;
        x.mul_add(x, y.mul_add(y, z * z))
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
        self.x.mul_add(rhs.x, self.y.mul_add(rhs.y, self.z * rhs.z))
    }
    fn components(&mut self) -> impl Iterator<Item = &mut Self::Num> {
        let Self { x, y, z } = self;
        [x, y, z].into_iter()
    }
}
impl<T: Neg<Output = T>> Neg for Vec3<T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        let Self { x, y, z } = self;
        Self {
            x: -x,
            y: -y,
            z: -z,
        }
    }
}
impl<T: Add<T, Output = T>, V: Into<Vec3<T>>> Add<V> for Vec3<T> {
    type Output = Self;

    fn add(self, rhs: V) -> Self::Output {
        let rhs = rhs.into();
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}
impl<T: AddAssign, V: Into<Vec3<T>>> AddAssign<V> for Vec3<T> {
    fn add_assign(&mut self, rhs: V) {
        let rhs = rhs.into();
        let Self { x, y, z } = self;
        *x += rhs.x;
        *y += rhs.y;
        *z += rhs.z;
    }
}
impl<T: Sub<T, Output = T>, V: Into<Vec3<T>>> Sub<V> for Vec3<T> {
    type Output = Self;

    fn sub(self, rhs: V) -> Self::Output {
        let rhs = rhs.into();
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}
impl<T: SubAssign, V: Into<Vec3<T>>> SubAssign<V> for Vec3<T> {
    fn sub_assign(&mut self, rhs: V) {
        let rhs = rhs.into();
        let Self { x, y, z } = self;
        *x -= rhs.x;
        *y -= rhs.y;
        *z -= rhs.z;
    }
}
impl<T: Mul<T, Output = T> + Clone> Mul<T> for Vec3<T> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self {
            x: self.x * rhs.clone(),
            y: self.y * rhs.clone(),
            z: self.z * rhs,
        }
    }
}
impl<T: MulAssign + Clone> MulAssign<T> for Vec3<T> {
    fn mul_assign(&mut self, rhs: T) {
        let Self { x, y, z } = self;
        *x *= rhs.clone();
        *y *= rhs.clone();
        *z *= rhs;
    }
}
impl<T: Div<T, Output = T> + Clone> Div<T> for Vec3<T> {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Self {
            x: self.x / rhs.clone(),
            y: self.y / rhs.clone(),
            z: self.z / rhs,
        }
    }
}
impl<T: DivAssign + Clone> DivAssign<T> for Vec3<T> {
    fn div_assign(&mut self, rhs: T) {
        let Self { x, y, z } = self;
        *x /= rhs.clone();
        *y /= rhs.clone();
        *z /= rhs;
    }
}
