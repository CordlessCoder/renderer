pub mod buf;
pub mod vec;

use bytemuck::Zeroable;
use vec::Vec3;

/// The triangle is considered as front-facing when the points appear to be in counter-clockwise
/// order when viewed.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Zeroable)]
#[repr(C)]
pub struct Triangle {
    pub points: [Vec3; 3],
}
