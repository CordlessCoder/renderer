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
pub mod prelude {
    use num_traits::AsPrimitive;

    pub use super::buf::{Buffer, Rgba};
    pub use super::vec::{CompleteVector, IntoVector, Vec2, Vec3, Vec4, Vector};
    pub use super::{CreateRay, Ray};
    pub type Vec2i = Vec2<i32>;
    pub type Vec3i = Vec3<i32>;
    pub type Vec4i = Vec4<i32>;
    pub type Vec2u = Vec2<u32>;
    pub type Vec3u = Vec3<u32>;
    pub type Vec4u = Vec4<u32>;
    pub type Vec2f = Vec2<f32>;
    pub type Vec3f = Vec3<f32>;
    pub type Vec4f = Vec4<f32>;
    pub type Ray2i = Ray<Vec2i>;
    pub type Ray3i = Ray<Vec3i>;
    pub type Ray4i = Ray<Vec4i>;
    pub type Ray2u = Ray<Vec2u>;
    pub type Ray3u = Ray<Vec3u>;
    pub type Ray4u = Ray<Vec4u>;
    pub type Ray2f = Ray<Vec2f>;
    pub type Ray3f = Ray<Vec3f>;
    pub type Ray4f = Ray<Vec4f>;
    pub fn vec2i(x: impl AsPrimitive<i32>, y: impl AsPrimitive<i32>) -> Vec2i {
        Vec2i::fromprim(x, y)
    }
    pub fn vec3i(
        x: impl AsPrimitive<i32>,
        y: impl AsPrimitive<i32>,
        z: impl AsPrimitive<i32>,
    ) -> Vec3i {
        Vec3i::fromprim(x, y, z)
    }
    pub fn vec4i(
        x: impl AsPrimitive<i32>,
        y: impl AsPrimitive<i32>,
        z: impl AsPrimitive<i32>,
        w: impl AsPrimitive<i32>,
    ) -> Vec4i {
        Vec4i::fromprim(x, y, z, w)
    }
    pub fn vec2u(x: impl AsPrimitive<u32>, y: impl AsPrimitive<u32>) -> Vec2u {
        Vec2u::fromprim(x, y)
    }
    pub fn vec3u(
        x: impl AsPrimitive<u32>,
        y: impl AsPrimitive<u32>,
        z: impl AsPrimitive<u32>,
    ) -> Vec3u {
        Vec3u::fromprim(x, y, z)
    }
    pub fn vec4u(
        x: impl AsPrimitive<u32>,
        y: impl AsPrimitive<u32>,
        z: impl AsPrimitive<u32>,
        w: impl AsPrimitive<u32>,
    ) -> Vec4u {
        Vec4u::fromprim(x, y, z, w)
    }
    pub fn vec2f(x: impl AsPrimitive<f32>, y: impl AsPrimitive<f32>) -> Vec2f {
        Vec2f::fromprim(x, y)
    }
    pub fn vec3f(
        x: impl AsPrimitive<f32>,
        y: impl AsPrimitive<f32>,
        z: impl AsPrimitive<f32>,
    ) -> Vec3f {
        Vec3f::fromprim(x, y, z)
    }
    pub fn vec4f(
        x: impl AsPrimitive<f32>,
        y: impl AsPrimitive<f32>,
        z: impl AsPrimitive<f32>,
        w: impl AsPrimitive<f32>,
    ) -> Vec4f {
        Vec4f::fromprim(x, y, z, w)
    }
}
