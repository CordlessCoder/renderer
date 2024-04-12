mod types;
pub use types::*;
pub mod prelude {
    pub use super::buf::{Buffer, Pixel};
    pub use super::vec::{BasicVector, Vec2, Vec3, Vec4, FLT};
}
