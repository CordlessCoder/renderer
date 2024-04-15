mod types;
pub use types::*;
pub mod prelude {
    use num_traits::AsPrimitive;

    pub use super::buf::{Buffer, Pixel};
    pub use super::vec::{IntoVector, Vec2, Vec3, Vec4, Vector};
    pub type Vec2i = Vec2<i32>;
    pub type Vec3i = Vec3<i32>;
    pub type Vec4i = Vec4<i32>;
    pub type Vec2u = Vec2<u32>;
    pub type Vec3u = Vec3<u32>;
    pub type Vec4u = Vec4<u32>;
    pub type Vec2f = Vec2<f32>;
    pub type Vec3f = Vec3<f32>;
    pub type Vec4f = Vec4<f32>;
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
