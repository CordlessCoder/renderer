use minifb::{Key, Window, WindowOptions};
use num_traits::Float;
use rayon::{
    iter::{IndexedParallelIterator, ParallelIterator},
    slice::ParallelSliceMut,
};
use std::{
    ops::{Add, Mul},
    time::{Duration, Instant},
};

use renderer_types::{color::Color, prelude::*};

fn lerp<B: Mul<T, Output = B> + Add<B, Output = B>, T: Float>(start: B, end: B, factor: T) -> B {
    start * (T::one() - factor) + (end * factor)
}

fn ray_color(ray: &Ray3f) -> Rgba {
    fn sphere_hit(center: Vec3f, radius: f32, ray: &Ray3f) -> Option<f32> {
        let oc = center - *ray.origin();
        let a = ray.direction().len_squared();
        let h = ray.direction().dot(oc);
        let c = oc.len_squared() - radius * radius;
        let discriminant = h * h - a * c;

        (discriminant >= 0.).then(|| (h - discriminant.sqrt()) / a)
    }
    let center = vec3f(0, 3, 0);
    if let Some(t) = sphere_hit(center, 0.5, ray) {
        // dbg!(t);
        let mut n = ray.at(t) - center;
        n = n.unit();
        // let dot = n.dot(*ray.direction());
        // let angle =
        //     1. - ((dot / (n.len_squared() * ray.direction().len_squared()).sqrt()).acos() /
        //        std::f32::consts::PI);
        n *= 0.5;
        n += Vec3f::splat(0.5);
        // dbg!(n.y);
        // let color = Color::from(n);
        // n.y *= 1.;
        let color = Color::from(n.yyy());
        // let color = lerp(Color::black(), Color::white(), angle);
        return color.into_rgba();
    }
    let a = (ray.direction().unit().z + 1.0) * 0.5;
    lerp(Color::new(1.0, 1.0, 1.0), Color::new(0.5, 0.7, 1.0), a).into_rgba()
    // lerp(Color::black(), Color::white(), ray.at(1.).z * 0.5 + 0.5).into_rgba()
}

fn main() {
    const WIDTH: usize = 1024;
    const HEIGHT: usize = 512;
    const DYNAMIC_SIZE: bool = true;
    const SCALE: minifb::Scale = minifb::Scale::X1;
    const WIN_WIDTH: usize = 1024;
    const WIN_HEIGHT: usize = 512;
    const REFRESH_RATE: u64 = 240;

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
        if DYNAMIC_SIZE {
            let (width, height) = window.get_size();
            buf.resize(width, height);
        }
        let aspect = buf.width() as f32 / buf.height() as f32;
        let viewport = {
            let height = 0.5;
            vec2f(aspect * height, height)
        };
        let focal_len: f32 = 1.0;

        let camera_origin = {
            // let mut pos = window
            //     .get_mouse_pos(minifb::MouseMode::Discard)
            //     .unwrap_or((0., 0.))
            //     .into_vector();
            // pos.x /= buf.width() as f32;
            // pos.y /= buf.height() as f32;
            // pos -= vec2f(0.5, 0.5);
            // vec3f(pos.x, 0, pos.y)
            vec3f(0, 0, 0)
        };
        let viewport_pixel_pos = {
            let delta = viewport.x / buf.width() as f32;
            let origin = vec3f(-viewport.x / 2.0, focal_len, viewport.y / 2.0);
            move |x: usize, y: usize| origin + vec3f(delta * x as f32, 0, -delta * y as f32)
        };
        let dim = buf.dimensions();
        let start = Instant::now();
        buf.inner_buf_mut()
            .par_chunks_exact_mut(dim.x)
            .enumerate()
            // Run scanlines in parallel
            .for_each(|(y, line)| {
                line.iter_mut().enumerate().for_each(move |(x, p)| {
                    let ray_target = viewport_pixel_pos(x, y);
                    let ray = camera_origin.to(ray_target);
                    *p = ray_color(&ray);
                });
            });
        // // Run pixels in parallel
        // .flat_map(|(y, line)| line.par_iter_mut().enumerate().map(move |(x, p)| (x, y, p)))
        // .for_each(|(x, y, p)| {
        //     let ray_target = viewport_pixel_pos(x, y);
        //     let ray = camera_origin.to(ray_target);
        //     *p = ray_color(&ray);
        // });

        // // Run pixels on one thread
        // buf.iter_pos_mut().for_each(|(x, y, p)| {
        //     let ray_target = viewport_pixel_pos(x, y);
        //     let ray = camera_origin.to(ray_target);
        //     // *p = Color::splat(ray.direction().z * 0.5 + 0.5).into_rgba()
        //     *p = ray_color(&ray);
        // });

        window
            .update_with_buffer(buf.as_rgba(), buf.width(), buf.height())
            .unwrap();
        let took = start.elapsed();
        println!(
            "Rendering+display took {took:?}, {:.1}fps",
            1.0 / took.as_secs_f64()
        );
    }
}
