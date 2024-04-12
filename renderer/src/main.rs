#![allow(unused, unreachable_code)]
use minifb::{Key, Window, WindowOptions};
use std::time::Duration;

use renderer::prelude::*;

fn main() {
    const WIDTH: usize = 32;
    const HEIGHT: usize = 32;
    const WIN_WIDTH: usize = 256;
    const WIN_HEIGHT: usize = 256;
    const FIXED_SIZE: bool = true;
    const REFRESH_RATE: u64 = 60;

    // Screen coordinates: 0..width Left to Right, 0..height Top to Bottom

    let mut window = Window::new(
        "renderer_swallow",
        WIN_WIDTH,
        WIN_HEIGHT,
        WindowOptions {
            borderless: true,
            resize: true,
            scale: minifb::Scale::FitScreen,
            ..WindowOptions::default()
        },
    )
    .unwrap();
    window.limit_update_rate(Some(Duration::from_nanos(1_000_000_000 / REFRESH_RATE)));

    let mut buf = Buffer::new(WIDTH, HEIGHT, Pixel::black());

    while window.is_open() && !window.is_key_down(Key::Escape) {
        if !FIXED_SIZE {
            let (width, height) = window.get_size();
            buf.resize(width, height);
        }
        let (width, height) = (buf.width(), buf.height());
        buf.iter_pos_mut().for_each(|(x, y, p)| {
            let uv = (x as f32 / width as f32, y as f32 / height as f32);
            p.r = (uv.0 * (u8::MAX as f32)) as u8;
            p.g = (uv.1 * (u8::MAX as f32)) as u8;
            p.b = 0;
        });

        window
            .update_with_buffer(buf.as_rgba(), width, height)
            .unwrap();
    }
}
