use minifb::{Key, Window, WindowOptions};
use num_traits::Float;
use std::{
    ops::{Add, Mul},
    time::Duration,
};

use renderer::{color::Color, prelude::*};

fn lerp<B: Mul<T, Output = B> + Add<B, Output = B>, T: Float>(start: B, end: B, factor: T) -> B {
    start * (T::one() - factor) + end * factor
}

fn ray_color(ray: &Ray3f) -> Rgba {
    let a = (ray.direction().unit().z + 1.0) * 0.5;
    lerp(Color::new(1.0, 1.0, 1.0), Color::new(0.5, 0.7, 1.0), a).into_rgba()
}

fn main() {
    const WIDTH: usize = 512;
    const HEIGHT: usize = 1024;
    const DYNAMIC_SIZE: bool = false;
    const SCALE: minifb::Scale = minifb::Scale::X1;
    const WIN_WIDTH: usize = 512;
    const WIN_HEIGHT: usize = 1024;
    const REFRESH_RATE: u64 = 60;

    let mut window = Window::new(
        "renderer",
        WIN_WIDTH,
        WIN_HEIGHT,
        WindowOptions {
            // borderless: true,
            title: true,
            resize: DYNAMIC_SIZE,
            scale: SCALE,
            // transparency: true,
            ..WindowOptions::default()
        },
    )
    .unwrap();
    window.limit_update_rate(Some(Duration::from_nanos(1_000_000_000 / REFRESH_RATE)));

    let mut buf = Buffer::new(WIDTH, HEIGHT, Rgba::black());

    while window.is_open() && !window.is_key_down(Key::Escape) {
        if !DYNAMIC_SIZE {
            let (width, height) = window.get_size();
            buf.resize(width, height);
        }
        let aspect = buf.width() as f32 / buf.height() as f32;
        let viewport = {
            let height = 2f32;
            vec2f(height * aspect, height)
        };
        let focal_len = 1f32;
        let camera_origin = vec3f(0, 0, 0);
        let viewport_pixel_pos = {
            let delta = viewport.x / buf.width() as f32;
            let origin = vec3f(-viewport.x / 2.0, focal_len, viewport.y / 2.0);
            move |x: usize, y: usize| origin + vec3f(delta * x as f32, 0, -delta * y as f32)
        };
        buf.iter_pos_mut().for_each(|(x, y, p)| {
            let ray_target = viewport_pixel_pos(x, y);
            let ray = camera_origin.to(ray_target);
            *p = ray_color(&ray);
        });

        window
            .update_with_buffer(buf.as_rgba(), buf.width(), buf.height())
            .unwrap();
    }
}
