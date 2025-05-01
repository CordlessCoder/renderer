use bytemuck::Zeroable;

use crate::prelude::*;

#[derive(bytemuck::Pod, Zeroable, Clone, Copy)]
// Required for compatibility with the u32 type
#[repr(align(4))]
#[repr(C)]
#[cfg(target_endian = "little")]
pub struct Rgba {
    pub b: u8,
    pub g: u8,
    pub r: u8,
    pub a: u8,
}
#[derive(bytemuck::Pod, Zeroable, Clone, Copy)]
// Required for compatibility with the u32 type
#[repr(align(4))]
#[repr(C)]
#[cfg(target_endian = "big")]
pub struct Rgba {
    pub a: u8,
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl From<u32> for Rgba {
    fn from(value: u32) -> Self {
        bytemuck::cast(value)
    }
}

impl Rgba {
    pub const fn black() -> Self {
        Rgba::new(0, 0, 0, 255)
    }
    pub const fn white() -> Self {
        Rgba::new(255, 255, 255, 255)
    }
    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Rgba { r, g, b, a }
    }
    pub const fn from_u32_rgba(val: u32) -> Self {
        let [r, g, b, a] = val.to_le_bytes();
        Self { r, g, b, a }
    }
}

pub struct Buffer {
    pixels: Vec<Rgba>,
    width: usize,
    height: usize,
}

impl Buffer {
    pub fn new(width: usize, height: usize, fill: Rgba) -> Self {
        Self::new_with(width, height, |_, _| fill)
    }
    pub fn new_with<F: FnMut(usize, usize) -> Rgba>(
        width: usize,
        height: usize,
        mut cb: F,
    ) -> Self {
        Self {
            width,
            height,
            pixels: (0..height)
                .flat_map(|y| (0..width).map(move |x| (x, y)))
                .map(|(x, y)| cb(x, y))
                .collect(),
        }
    }
    pub fn as_rgba(&self) -> &[u32] {
        bytemuck::cast_slice(&self.pixels)
    }
    pub fn as_rgba_mut(&mut self) -> &mut [u32] {
        bytemuck::cast_slice_mut(&mut self.pixels)
    }
    pub fn inner_buf(&self) -> &[Rgba] {
        &self.pixels
    }
    pub fn inner_buf_mut(&mut self) -> &mut [Rgba] {
        &mut self.pixels
    }
    pub fn dimensions(&self) -> Vec2<usize> {
        Vec2::new(self.width, self.height)
    }
    pub fn width(&self) -> usize {
        self.width
    }
    pub fn height(&self) -> usize {
        self.height
    }
    pub fn get(&self, x: usize, y: usize) -> Option<Rgba> {
        if x >= self.width || y >= self.height {
            return None;
        }
        Some(unsafe { *self.get_unchecked(x, y) })
    }
    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut Rgba> {
        if x >= self.width || y >= self.height {
            return None;
        }
        Some(unsafe { self.get_unchecked_mut(x, y) })
    }
    /// # Safety
    ///
    /// Requires x < width, y < height
    pub unsafe fn get_unchecked(&self, x: usize, y: usize) -> &Rgba {
        self.pixels.get_unchecked(y * self.width + x)
    }
    /// # Safety
    ///
    /// Requires x < width, y < height
    pub unsafe fn get_unchecked_mut(&mut self, x: usize, y: usize) -> &mut Rgba {
        self.pixels.get_unchecked_mut(y * self.width + x)
    }
    pub fn set(&mut self, x: usize, y: usize, value: Rgba) {
        let Some(pixel) = self.get_mut(x, y) else {
            panic!(
                "Attempted to set pixel {x}, {y} in a {width}x{height} buffer",
                width = self.width(),
                height = self.height()
            )
        };
        *pixel = value;
    }
    pub fn iter(&self) -> impl Iterator<Item = &Rgba> {
        self.pixels.iter()
    }
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Rgba> {
        self.pixels.iter_mut()
    }
    pub fn iter_pos(&self) -> impl Iterator<Item = (usize, usize, &Rgba)> {
        self.pixels
            .chunks_exact(self.width)
            .enumerate()
            .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, pixel)| (x, y, pixel)))
    }
    pub fn iter_pos_mut(&mut self) -> impl Iterator<Item = (usize, usize, &mut Rgba)> {
        self.pixels
            .chunks_exact_mut(self.width)
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter_mut()
                    .enumerate()
                    .map(move |(x, pixel)| (x, y, pixel))
            })
    }
    pub fn resize_and_fill<F: FnMut(usize, usize) -> Rgba>(
        &mut self,
        width: usize,
        height: usize,
        mut cb: F,
    ) {
        if self.width == width && self.height == height {
            return;
        }
        self.pixels.clear();
        let iter = (0..height)
            .flat_map(|y| (0..width).map(move |x| (x, y)))
            .map(|(x, y)| cb(x, y));
        self.pixels.extend(iter);
        self.width = width;
        self.height = height;
    }
    pub fn resize(&mut self, width: usize, height: usize) {
        self.resize_and_fill(width, height, |_, _| Rgba::black())
    }
}
