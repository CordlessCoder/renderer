mod types;
pub use types::*;
pub mod prelude {
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
}
