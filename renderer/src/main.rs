#![allow(unused, unreachable_code)]
use minifb::{Key, Window, WindowOptions};
use num_traits::Float;
use std::{sync::atomic::AtomicU32, time::Duration};

use renderer::prelude::*;

fn main() {
    const WIDTH: usize = 32;
    const HEIGHT: usize = 32;
    const WIN_WIDTH: usize = 256;
    const WIN_HEIGHT: usize = 256;
    const FIXED_SIZE: bool = false;
    const REFRESH_RATE: u64 = 60;

    // Screen coordinates: 0..width Left to Right, 0..height Top to Bottom

    let mut window = Window::new(
        "renderer",
        WIN_WIDTH,
        WIN_HEIGHT,
        WindowOptions {
            // borderless: true,
            title: true,
            resize: !FIXED_SIZE,
            scale: minifb::Scale::FitScreen,
            // transparency: true,
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
        let min = width.min(height);
        let max = width.max(height);
        let maxlen = max as f32 / 2.;
        let minlen = min as f32 / 2.;
        let feather = 3.;
        let per_pixel = feather / max as f32;
        buf.iter_pos_mut().for_each(|(x, y, p)| {
            let mid = vec2f(width, height) / 2.0;
            let pos = vec2f(x, y);
            let pos = pos - mid;
            let x = pos.len() / minlen;
            let diff = (x - 1.0).clamp(0.0, 1.0);
            let x = if diff == 0. {
                255
            } else {
                ((per_pixel - diff).clamp(0.0, 1.0) / feather * max as f32 * 256.) as u8
            };
            *p = Pixel::new(x, x, x, 255);
        });

        window
            .update_with_buffer(buf.as_rgba(), width, height)
            .unwrap();
    }
}
