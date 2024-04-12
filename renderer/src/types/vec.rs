use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use bytemuck::Zeroable;
use num_traits::{real::Real, AsPrimitive};
use renderer_macros::swizzle;

pub trait BasicVector {
    type Num;
    /// Returns |vec|^2
    fn len_squared(&self) -> Self::Num;
    fn len(&self) -> Self::Num;
    fn normalized(self) -> Self;
    fn dot(self, rhs: Self) -> Self::Num;
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

#[derive(Debug, Clone, PartialEq, PartialOrd, Zeroable)]
#[repr(C)]
pub struct Vec2<T = f32> {
    pub x: T,
    pub y: T,
}
impl<T: Copy> Copy for Vec2<T> {}
impl<T> Vec2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
    pub fn splat(val: T) -> Self
    where
        T: Clone,
    {
        Self {
            x: val.clone(),
            y: val,
        }
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
impl<T: Clone> Vec2<T> {
    // These provide methods such as .xy to extract a group of values into a new vec
    swizzle!([x, y] 2 prefix "implement_swizzle!(T, " suffix ");");
    swizzle!([x, y] 3 prefix "implement_swizzle!(T, " suffix ");");
    swizzle!([x, y] 4 prefix "implement_swizzle!(T, " suffix ");");
}
impl<T, T1: Into<T>, T2: Into<T>> From<(T1, T2)> for Vec2<T> {
    fn from((x, y): (T1, T2)) -> Self {
        Self::new(x.into(), y.into())
    }
}
impl<T> BasicVector for Vec2<T>
where
    T: Mul<T, Output = T> + Add<T, Output = T> + Real,
{
    type Num = T;

    fn len_squared(&self) -> Self::Num {
        let Self { x, y } = *self;
        x.mul_add(x, y * y)
    }
    fn len(&self) -> Self::Num {
        self.len_squared().sqrt()
    }
    fn normalized(self) -> Self {
        let len = self.len();
        if len.is_zero() {
            return self;
        }
        self / len
    }
    fn dot(self, rhs: Self) -> Self::Num {
        self.x.mul_add(rhs.x, self.y * rhs.y)
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

#[derive(Debug, Clone, PartialEq, PartialOrd, Zeroable)]
#[repr(C)]
pub struct Vec3<T = f32> {
    pub x: T,
    pub y: T,
    pub z: T,
}
impl<T: Copy> Copy for Vec3<T> {}

impl<T> Vec3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
}
impl<T: Clone> Vec3<T> {
    pub fn splat(val: T) -> Self {
        Self {
            x: val.clone(),
            y: val.clone(),
            z: val,
        }
    }
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
    swizzle!([x, y, z] 2 prefix "implement_swizzle!(T, " suffix ");");
    swizzle!([x, y, z] 3 prefix "implement_swizzle!(T, " suffix ");");
    swizzle!([x, y, z] 4 prefix "implement_swizzle!(T, " suffix ");");
}
impl<T, T1: Into<T>, T2: Into<T>, T3: Into<T>> From<(T1, T2, T3)> for Vec3<T> {
    fn from((x, y, z): (T1, T2, T3)) -> Self {
        Self::new(x.into(), y.into(), z.into())
    }
}
impl<T: Mul<T, Output = T> + Real> BasicVector for Vec3<T> {
    type Num = T;

    fn len_squared(&self) -> Self::Num {
        let Self { x, y, z } = *self;
        x.mul_add(x, y.mul_add(y, z * z))
    }
    fn len(&self) -> Self::Num {
        self.len_squared().sqrt()
    }
    fn normalized(self) -> Self {
        let len = self.len();
        if len.is_zero() {
            return self;
        }
        self / len
    }
    fn dot(self, rhs: Self) -> Self::Num {
        self.x.mul_add(rhs.x, self.y.mul_add(rhs.y, self.z * rhs.z))
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
    pub fn new(x: T, y: T, z: T, w: T) -> Self {
        Self { x, y, z, w }
    }
    pub fn splat(val: impl AsPrimitive<T>) -> Self
    where
        T: Copy + 'static,
    {
        let val = val.as_();
        Self {
            x: val,
            y: val,
            z: val,
            w: val,
        }
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
impl<T: Clone> Vec4<T> {
    swizzle!([x, y, z, w] 2 prefix "implement_swizzle!(T, " suffix ");");
    swizzle!([x, y, z, w] 3 prefix "implement_swizzle!(T, " suffix ");");
    swizzle!([x, y, z, w] 4 prefix "implement_swizzle!(T, " suffix ");");
}
impl<T, T1: Into<T>, T2: Into<T>, T3: Into<T>, T4: Into<T>> From<(T1, T2, T3, T4)> for Vec4<T> {
    fn from((x, y, z, w): (T1, T2, T3, T4)) -> Self {
        Self::new(x.into(), y.into(), z.into(), w.into())
    }
}
impl<T: Mul<T, Output = T> + Real> BasicVector for Vec4<T> {
    type Num = T;

    fn len_squared(&self) -> Self::Num {
        let Self { x, y, z, w } = *self;
        x.mul_add(x, y.mul_add(y, z.mul_add(z, w * w)))
    }
    fn len(&self) -> Self::Num {
        self.len_squared().sqrt()
    }
    fn normalized(self) -> Self {
        let len = self.len();
        if len.is_zero() {
            return self;
        }
        self / len
    }
    fn dot(self, rhs: Self) -> Self::Num {
        self.x.mul_add(
            rhs.x,
            self.y.mul_add(rhs.y, self.z.mul_add(rhs.z, self.w * rhs.w)),
        )
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
