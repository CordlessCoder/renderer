pub mod buf;
use bytemuck::Zeroable;
use renderer_macros::swizzle;

macro_rules! implement_swizzle {
    ($combined:ident, $x:ident, $y:ident) => {
        pub fn $combined(&self) -> Vec2 {
            Vec2 {
                x: self.$x,
                y: self.$y,
            }
        }
    };
    ($combined:ident, $x:ident, $y:ident, $z:ident) => {
        pub fn $combined(&self) -> Vec3 {
            Vec3 {
                x: self.$x,
                y: self.$y,
                z: self.$z,
            }
        }
    };
}

pub type FLT = f32;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Zeroable)]
#[repr(C)]
pub struct Vec2 {
    pub x: FLT,
    pub y: FLT,
}

impl Vec2 {
    pub fn new(x: FLT, y: FLT) -> Self {
        Vec2 { x, y }
    }

    swizzle!([x, y] 2 => implement_swizzle!);
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Zeroable)]
#[repr(C)]
pub struct Vec3 {
    pub x: FLT,
    pub y: FLT,
    pub z: FLT,
}

impl Vec3 {
    pub fn new(x: FLT, y: FLT, z: FLT) -> Self {
        Vec3 { x, y, z }
    }
    swizzle!([x, y, z] 2 => implement_swizzle!);
    swizzle!([x, y, z] 3 => implement_swizzle!);
}

/// The triangle is considered as front-facing when the points appear to be in counter-clockwise
/// order when viewed.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Zeroable)]
#[repr(C)]
pub struct Triangle {
    pub points: [Vec3; 3],
}
