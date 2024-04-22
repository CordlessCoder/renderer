use minifb::{Key, Window, WindowOptions};
use num_traits::Float;
use rayon::{
    iter::{IndexedParallelIterator, ParallelIterator},
    slice::ParallelSliceMut,
};
use std::{
    f32::consts::PI,
    ops::{Add, Mul},
    time::{Duration, Instant},
};

use renderer_types::{color::Color, prelude::*};

fn lerp<B, T: Float>(start: B, end: B, factor: T) -> B
where
    B: Mul<T, Output = B> + Add<B, Output = B>,
{
    start * (T::one() - factor) + (end * factor)
}

#[derive(Debug, PartialEq, Clone, Copy, Default)]
pub struct Hit {
    pub point: Vec3f,
    pub normal: Vec3f,
    pub t: f32,
    pub front_face: bool,
}

impl Hit {
    pub fn new(point: Vec3f, t: f32, ray: &Ray3f, outward_normal: Vec3f) -> Self {
        let mut h = Self {
            point,
            t,
            ..Default::default()
        };
        h.set_normal(ray, outward_normal);
        h
    }
    /// Sets the hit record normal vector.
    /// NOTE: the parameter `outward_normal` is assumed to have unit length.
    pub fn set_normal(&mut self, ray: &Ray3f, outward_normal: Vec3f) {
        self.front_face = ray.direction().dot(outward_normal) < 0.;
        self.normal = self
            .front_face
            .then_some(outward_normal)
            .unwrap_or_else(|| -outward_normal)
    }
}

pub trait Object {
    fn hit(&self, ray: &Ray3f, t_range: (f32, f32)) -> Option<Hit>;
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Sphere {
    center: Vec3f,
    radius: f32,
}

impl Sphere {
    pub fn new(center: Vec3f, radius: f32) -> Self {
        Self {
            center,
            radius: radius.max(0.0),
        }
    }
}

impl Object for Sphere {
    fn hit(&self, ray: &Ray3f, (t_min, t_max): (f32, f32)) -> Option<Hit> {
        let oc = self.center - *ray.origin();
        let a = ray.direction().len_squared();
        let h = ray.direction().dot(oc);
        let c = oc.len_squared() - self.radius * self.radius;

        let discriminant = h * h - a * c;
        if discriminant < 0. {
            return None;
        }
        let sqrtd = discriminant.sqrt();
        // Find the nearest root that lies in the acceptable range.
        let mut root = (h - sqrtd) / a;
        if root <= t_min || t_max <= root {
            root = (h + sqrtd) / a;
            if root <= t_min || t_max <= root {
                return None;
            }
        }

        let point = ray.at(root);
        Some(Hit::new(
            point,
            root,
            ray,
            (point - self.center) / self.radius,
        ))
    }
}

fn ray_color(ray: &Ray3f) -> Rgba {
    // fn environment(ray: &Ray3f) -> Color<f32> {
    //     let a = ray.direction().unit().z.mul_add(0.5, 0.5);
    //     if a < 0.5 {
    //         return Color::new(0.1, 0.1, 0.1);
    //     }
    //     lerp(Color::new(1.0, 1.0, 1.0) * 0.0, SHADE, a)
    // }

    const ILLUMINATED: Color<f32> = Color::new(0.953125, 0.910156, 0.605469);
    const SHADE: Color<f32> = Color::new(0.529, 0.808, 0.922);
    const SHADE_FACTOR: f32 = 0.0;
    let sun_ray = vec3f(2, 2, 8).to(vec3f(0, 0, 0)).direction().unit();

    let sphere = Sphere::new(vec3f(0, 3, 0), 0.5);
    // // lerp(Color::black(), Color::white(), ray.at(1.).z * 0.5 + 0.5).into_rgba()
    // let sphere = Sphere::new(vec3f(0, 3, 0), 0.5);
    // let mut rec = HitRecord::default();
    // if sphere.hit(ray, (0.0, 10000.0), &mut rec) {
    //     let surface = Colorf32::from_rgb(206, 210, 215) * 0.7;
    //     let normal = rec.normal;
    //     // let sun_angle = angle_between(&normal, &sun_ray);
    //     // let factor = ((sun_angle / PI - SHADE_FACTOR) * (1. / (1. - SHADE_FACTOR))).clamp(0.0, 1.0);
    //
    //     let reflected = *ray.direction() - normal * (2. * normal.dot(*ray.direction()));
    //     let reflected = ray.direction().reflect(normal);
    //     let reflected_ray = Ray3f::new(rec.point, reflected);
    //     let fresnel = 1. - angle_between(&normal, ray.direction()) / PI;
    //
    //     let environment = environment(&reflected_ray);
    //     let sun_angle = angle_between(&normal, &sun_ray);
    //     let factor = ((sun_angle / PI - SHADE_FACTOR) * (1. / (1. - SHADE_FACTOR))).clamp(0.0, 1.0);
    //     let surface = lerp(surface, ILLUMINATED, factor);
    //     // normal *= 0.5;
    //     // normal += Vec3f::splat(0.5);
    //     // // normal.y *= -1.;
    //     // // // dbg!(n.y);
    //     // let color = Color::from(normal.xzy());
    //     // // n.y *= 1.;
    //     // let color = Color::from(normal.zzz());
    //     let color = lerp(environment, ILLUMINATED, factor) * surface;
    //     // let color = lerp(surface, environment, fresnel);
    //     // let color = lerp(Colorf32::black(), Colorf32::white(), fresnel);
    //     return color.into_rgba();
    // };
    // environment(ray).into_rgba()
}

fn main() {
    const fn scale_to_int(scale: minifb::Scale) -> u8 {
        use minifb::Scale::*;
        match scale {
            X1 => 1,
            X2 => 2,
            X4 => 4,
            X8 => 8,
            X16 => 16,
            X32 => 32,
            FitScreen => 1,
        }
    }
    const WIDTH: usize = 1024;
    const HEIGHT: usize = 512;
    const DYNAMIC_SIZE: bool = true;
    const SCALE: minifb::Scale = minifb::Scale::X4;
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
            let (width, height) = (
                width / scale_to_int(SCALE) as usize,
                height / scale_to_int(SCALE) as usize,
            );
            buf.resize(width, height);
        }
        let aspect = buf.width() as f32 / buf.height() as f32;
        let viewport = {
            let height = 0.5;
            vec2f(aspect * height, height)
        };
        let focal_len: f32 = 1.0;

        let camera_origin = 'cam: {
            let Some(mut pos) = window
                .get_mouse_pos(minifb::MouseMode::Discard)
                .map(|pos| pos.into_vector())
            else {
                break 'cam vec3f(0, 0, 0);
            };
            pos.x /= buf.width() as f32;
            pos.y /= buf.height() as f32;
            pos -= vec2f(0.5, 0.5);
            pos.x *= -2.0;
            pos.y *= 2.0;
            vec3f(pos.x, 0, pos.y)
            // vec3f(0, 0, 0)
        };
        let viewport_pixel_pos = {
            let delta = viewport.x / buf.width() as f32;
            let origin = vec3f(-viewport.x / 2.0, focal_len, viewport.y / 2.0) + camera_origin;
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
