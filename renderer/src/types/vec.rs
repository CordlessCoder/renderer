use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use bytemuck::Zeroable;
use num_traits::AsPrimitive;
use renderer_macros::swizzle;

pub type FLT = f32;

pub trait BasicVector {
    fn len_squared(&self) -> FLT;
    fn len(&self) -> FLT {
        self.len_squared().sqrt()
    }
    fn normalized(self) -> Self;
    fn dot(self, rhs: Self) -> FLT;
}

macro_rules! implement_swizzle {
    ($combined:ident, $x:ident, $y:ident) => {
        pub fn $combined(self) -> Vec2 {
            Vec2 {
                x: self.$x,
                y: self.$y,
            }
        }
    };
    ($combined:ident, $x:ident, $y:ident, $z:ident) => {
        pub fn $combined(self) -> Vec3 {
            Vec3 {
                x: self.$x,
                y: self.$y,
                z: self.$z,
            }
        }
    };
    ($combined:ident, $x:ident, $y:ident, $z:ident, $w:ident) => {
        pub fn $combined(self) -> Vec4 {
            Vec4 {
                x: self.$x,
                y: self.$y,
                z: self.$z,
                w: self.$w,
            }
        }
    };
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Zeroable)]
#[repr(C)]
pub struct Vec2 {
    pub x: FLT,
    pub y: FLT,
}
impl Vec2 {
    pub fn new(x: impl AsPrimitive<FLT>, y: impl AsPrimitive<FLT>) -> Self {
        Vec2 {
            x: x.as_(),
            y: y.as_(),
        }
    }
    pub fn splat(val: impl AsPrimitive<FLT>) -> Self {
        let val = val.as_();
        Vec2 { x: val, y: val }
    }
    swizzle!([x, y] 2 => implement_swizzle!);
}
impl<T1: AsPrimitive<FLT>, T2: AsPrimitive<FLT>> From<(T1, T2)> for Vec2 {
    fn from((x, y): (T1, T2)) -> Self {
        Self::new(x, y)
    }
}
impl BasicVector for Vec2 {
    fn len_squared(&self) -> FLT {
        self.x * self.x + self.y * self.y
    }
    fn normalized(self) -> Self {
        let len = self.len();
        if len == 0.0 {
            return self;
        }
        let Self { x, y } = self;
        Self {
            x: x / len,
            y: y / len,
        }
    }
    fn dot(self, rhs: Self) -> FLT {
        self.x * rhs.x + self.y * rhs.y
    }
}
impl Neg for Vec2 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        let Self { x, y } = self;
        Self { x: -x, y: -y }
    }
}
impl<V: Into<Vec2>> Add<V> for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: V) -> Self::Output {
        let rhs = rhs.into();
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
impl<V: Into<Vec2>> AddAssign<V> for Vec2 {
    fn add_assign(&mut self, rhs: V) {
        let rhs = rhs.into();
        let Self { x, y } = self;
        *x += rhs.x;
        *y += rhs.y;
    }
}
impl<V: Into<Vec2>> Sub<V> for Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: V) -> Self::Output {
        let rhs = rhs.into();
        Vec2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
impl<V: Into<Vec2>> SubAssign<V> for Vec2 {
    fn sub_assign(&mut self, rhs: V) {
        let rhs = rhs.into();
        let Self { x, y } = self;
        *x -= rhs.x;
        *y -= rhs.y;
    }
}
impl<T: AsPrimitive<FLT>> Mul<T> for Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: T) -> Self::Output {
        let rhs = rhs.as_();
        Vec2 {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}
impl<T: AsPrimitive<FLT>> MulAssign<T> for Vec2 {
    fn mul_assign(&mut self, rhs: T) {
        let Self { x, y } = self;
        let rhs = rhs.as_();
        *x *= rhs;
        *y *= rhs;
    }
}
impl<T: AsPrimitive<FLT>> Div<T> for Vec2 {
    type Output = Vec2;

    fn div(self, rhs: T) -> Self::Output {
        let rhs = rhs.as_();
        Vec2 {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}
impl<T: AsPrimitive<FLT>> DivAssign<T> for Vec2 {
    fn div_assign(&mut self, rhs: T) {
        let Self { x, y } = self;
        let rhs = rhs.as_();
        *x /= rhs;
        *y /= rhs;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Zeroable)]
#[repr(C)]
pub struct Vec3 {
    pub x: FLT,
    pub y: FLT,
    pub z: FLT,
}

impl Vec3 {
    pub fn new(
        x: impl AsPrimitive<FLT>,
        y: impl AsPrimitive<FLT>,
        z: impl AsPrimitive<FLT>,
    ) -> Self {
        Vec3 {
            x: x.as_(),
            y: y.as_(),
            z: z.as_(),
        }
    }
    pub fn splat(val: impl AsPrimitive<FLT>) -> Self {
        let val = val.as_();
        Vec3 {
            x: val,
            y: val,
            z: val,
        }
    }
    swizzle!([x, y, z] 2 => implement_swizzle!);
    swizzle!([x, y, z] 3 => implement_swizzle!);
    pub fn cross(self, rhs: Self) -> Self {
        Vec3 {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }
}
impl<T1: AsPrimitive<FLT>, T2: AsPrimitive<FLT>, T3: AsPrimitive<FLT>> From<(T1, T2, T3)> for Vec3 {
    fn from((x, y, z): (T1, T2, T3)) -> Self {
        Self::new(x, y, z)
    }
}
impl BasicVector for Vec3 {
    fn len_squared(&self) -> FLT {
        let xy = self.xy().len();
        Vec2::new(xy, self.z).len_squared()
    }
    fn normalized(self) -> Self {
        let len = self.len();
        if len == 0.0 {
            return self;
        }
        let Self { x, y, z } = self;
        Self {
            x: x / len,
            y: y / len,
            z: z / len,
        }
    }
    fn dot(self, rhs: Self) -> FLT {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}
impl Neg for Vec3 {
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
impl<V: Into<Vec3>> Add<V> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: V) -> Self::Output {
        let rhs = rhs.into();
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}
impl<V: Into<Vec3>> AddAssign<V> for Vec3 {
    fn add_assign(&mut self, rhs: V) {
        let rhs = rhs.into();
        let Self { x, y, z } = self;
        *x += rhs.x;
        *y += rhs.y;
        *z += rhs.z;
    }
}
impl<V: Into<Vec3>> Sub<V> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: V) -> Self::Output {
        let rhs = rhs.into();
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}
impl<V: Into<Vec3>> SubAssign<V> for Vec3 {
    fn sub_assign(&mut self, rhs: V) {
        let rhs = rhs.into();
        let Self { x, y, z } = self;
        *x -= rhs.x;
        *y -= rhs.y;
        *z -= rhs.z;
    }
}
impl<T: AsPrimitive<FLT>> Mul<T> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: T) -> Self::Output {
        let rhs = rhs.as_();
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}
impl<T: AsPrimitive<FLT>> MulAssign<T> for Vec3 {
    fn mul_assign(&mut self, rhs: T) {
        let Self { x, y, z } = self;
        let rhs = rhs.as_();
        *x *= rhs;
        *y *= rhs;
        *z *= rhs;
    }
}
impl<T: AsPrimitive<FLT>> Div<T> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: T) -> Self::Output {
        let rhs = rhs.as_();
        Vec3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}
impl<T: AsPrimitive<FLT>> DivAssign<T> for Vec3 {
    fn div_assign(&mut self, rhs: T) {
        let Self { x, y, z } = self;
        let rhs = rhs.as_();
        *x /= rhs;
        *y /= rhs;
        *z /= rhs;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Zeroable)]
#[repr(C)]
pub struct Vec4 {
    pub x: FLT,
    pub y: FLT,
    pub z: FLT,
    pub w: FLT,
}

impl Vec4 {
    pub fn new(
        x: impl AsPrimitive<FLT>,
        y: impl AsPrimitive<FLT>,
        z: impl AsPrimitive<FLT>,
        w: impl AsPrimitive<FLT>,
    ) -> Self {
        Vec4 {
            x: x.as_(),
            y: y.as_(),
            z: z.as_(),
            w: w.as_(),
        }
    }
    pub fn splat(val: impl AsPrimitive<FLT>) -> Self {
        let val = val.as_();
        Vec4 {
            x: val,
            y: val,
            z: val,
            w: val,
        }
    }
    swizzle!([x, y, z, w] 2 => implement_swizzle!);
    swizzle!([x, y, z, w] 3 => implement_swizzle!);
    swizzle!([x, y, z, w] 4 => implement_swizzle!);
}
impl<T1: AsPrimitive<FLT>, T2: AsPrimitive<FLT>, T3: AsPrimitive<FLT>, T4: AsPrimitive<FLT>>
    From<(T1, T2, T3, T4)> for Vec4
{
    fn from((x, y, z, w): (T1, T2, T3, T4)) -> Self {
        Self::new(x, y, z, w)
    }
}
impl BasicVector for Vec4 {
    fn len_squared(&self) -> FLT {
        let xy = self.xy().len();
        let zw = self.zw().len();
        Vec2::new(xy, zw).len_squared()
    }
    fn normalized(self) -> Self {
        let len = self.len();
        if len == 0.0 {
            return self;
        }
        let Self { x, y, z, w } = self;
        Self {
            x: x / len,
            y: y / len,
            z: z / len,
            w: w / len,
        }
    }
    fn dot(self, rhs: Self) -> FLT {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z + self.w * rhs.w
    }
}
impl Neg for Vec4 {
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
impl<V: Into<Vec4>> Add<V> for Vec4 {
    type Output = Vec4;

    fn add(self, rhs: V) -> Self::Output {
        let rhs = rhs.into();
        Vec4 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w,
        }
    }
}
impl<V: Into<Vec4>> AddAssign<V> for Vec4 {
    fn add_assign(&mut self, rhs: V) {
        let rhs = rhs.into();
        let Self { x, y, z, w } = self;
        *x += rhs.x;
        *y += rhs.y;
        *z += rhs.z;
        *w += rhs.w;
    }
}
impl<V: Into<Vec4>> Sub<V> for Vec4 {
    type Output = Vec4;

    fn sub(self, rhs: V) -> Self::Output {
        let rhs = rhs.into();
        Vec4 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w,
        }
    }
}
impl<V: Into<Vec4>> SubAssign<V> for Vec4 {
    fn sub_assign(&mut self, rhs: V) {
        let rhs = rhs.into();
        let Self { x, y, z, w } = self;
        *x -= rhs.x;
        *y -= rhs.y;
        *z -= rhs.z;
        *w -= rhs.w;
    }
}
impl<T: AsPrimitive<FLT>> Mul<T> for Vec4 {
    type Output = Vec4;

    fn mul(self, rhs: T) -> Self::Output {
        let rhs = rhs.as_();
        Vec4 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w * rhs,
        }
    }
}
impl<T: AsPrimitive<FLT>> MulAssign<T> for Vec4 {
    fn mul_assign(&mut self, rhs: T) {
        let Self { x, y, z, w } = self;
        let rhs = rhs.as_();
        *x *= rhs;
        *y *= rhs;
        *z *= rhs;
        *w *= rhs;
    }
}
impl<T: AsPrimitive<FLT>> Div<T> for Vec4 {
    type Output = Vec4;

    fn div(self, rhs: T) -> Self::Output {
        let rhs = rhs.as_();
        Vec4 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
            w: self.w / rhs,
        }
    }
}
impl<T: AsPrimitive<FLT>> DivAssign<T> for Vec4 {
    fn div_assign(&mut self, rhs: T) {
        let Self { x, y, z, w } = self;
        let rhs = rhs.as_();
        *x /= rhs;
        *y /= rhs;
        *z /= rhs;
        *w /= rhs;
    }
}
