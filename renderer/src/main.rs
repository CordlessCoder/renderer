use num_traits::Float;
use rayon::{
    iter::{IndexedParallelIterator, ParallelIterator},
    slice::ParallelSliceMut,
};
use winit::{
    dpi::PhysicalSize,
    event::{Event, WindowEvent},
    raw_window_handle::HasWindowHandle,
    window::Window,
};
mod winit_app;
// use rayon::{
//     iter::{IndexedParallelIterator, ParallelIterator},
//     slice::ParallelSliceMut,
// };
use std::{
    f32::consts::PI,
    num::NonZeroU32,
    ops::{Add, Mul, RangeBounds},
    rc::Rc,
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
    fn hit(&self, ray: &Ray3f, t_range: impl RangeBounds<f32> + Clone) -> Option<Hit>;
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
    fn hit(&self, ray: &Ray3f, t_range: impl RangeBounds<f32>) -> Option<Hit> {
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

        if !t_range.contains(&root) {
            root = (h + sqrtd) / a;
            if !t_range.contains(&root) {
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

impl<T: Object> Object for [T] {
    fn hit(&self, ray: &Ray3f, t_range: impl RangeBounds<f32> + Clone) -> Option<Hit> {
        self.iter()
            .flat_map(|o| o.hit(ray, t_range.clone()))
            .min_by(|a, b| a.t.total_cmp(&b.t))
    }
}

fn ray_color(ray: &Ray3f, t_range: impl RangeBounds<f32> + Clone) -> Colorf32 {
    fn environment(ray: &Ray3f) -> Color<f32> {
        let a = ray.direction().unit().z.mul_add(0.5, 0.5);
        // if a < 0.5 {
        //     return Color::new(0.1, 0.1, 0.1);
        // }
        lerp(Color::new(1.0, 1.0, 1.0) * 0.0, SHADE, a)
        // Color::from(*ray.direction())
    }

    const ILLUMINATED: Color<f32> = Color::new(0.953125, 0.910156, 0.605469);
    const SHADE: Color<f32> = Color::new(0.529, 0.808, 0.922);
    const SHADE_FACTOR: f32 = 0.0;
    // let sun_ray = vec3f(2, 2, 8).to(vec3f(0, 0, 0)).direction().unit();
    //
    // // let sphere = Sphere::new(vec3f(0, 3, 0), 0.5);
    // // Rgba::white()
    // // // lerp(Color::black(), Color::white(), ray.at(1.).z * 0.5 + 0.5).into_rgba()
    let objects = [
        Sphere::new(vec3f(0, 3, 0), 0.4),
        Sphere::new(vec3f(0.2, 1, 0), 0.1),
    ];
    // // let mut rec = HitRecord::default();
    if let Some(hit) = objects.hit(ray, t_range.clone()) {
        // let normal = hit.normal;
        //     // let sun_angle = angle_between(&normal, &sun_ray);
        //     // let factor = ((sun_angle / PI - SHADE_FACTOR) * (1. / (1. - SHADE_FACTOR))).clamp(0.0, 1.0);
        //
        //     let reflected = ray.direction().reflect(normal);
        //     let reflected_ray = Ray3f::new(hit.point, reflected);
        //     let fresnel = normal.angle_to(ray.direction()).mul_add(-2. / PI, 2.);
        //
        //     let external = ray_color(ray, t_range.clone());
        //     let sun_angle = normal.angle_to(&sun_ray);
        //     let factor = ((sun_angle / PI - SHADE_FACTOR) * (1. / (1. - SHADE_FACTOR))).clamp(0.0, 1.0);
        //     // let color = lerp(SHADE * SHADE_FACTOR, ILLUMINATED, factor);
        //     // let color = Color::black();
        //     // let color = Color::white() * 0.4;
        //     // let color = lerp(Color::white() * 0.2, external, lerp(0.4, fresnel, fresnel));
        //
        //     // let color = Color::splat((Color::splat(fresnel) > Color::splat(0.1)) as u8 as f32);
        //     let color = external;
        //     return color;
        //     // return Color::from(hit.normal * -1.).into_rgba();
        return Color::from(hit.normal.zyx().map(|n| *n += 1.) * 0.5);
    };
    environment(ray)
    // Rgba::black()
}

fn main() {
    rayon::ThreadPoolBuilder::new()
        // .num_threads(8)
        .build_global()
        .unwrap();
    const WIDTH: usize = 1024;
    const HEIGHT: usize = 512;
    const DYNAMIC_SIZE: bool = true;
    const WIN_WIDTH: usize = 1024;
    const WIN_HEIGHT: usize = 512;

    let mut app = winit_app::WinitAppBuilder::with_init(
        |event_loop| {
            let window = event_loop
                .create_window(Window::default_attributes())
                .unwrap();
            Rc::new(window)
        },
        |event_loop, win| {
            let context = softbuffer::Context::new(win.clone()).unwrap();
            softbuffer::Surface::new(&context, win.clone()).unwrap()
        },
    )
    .with_event_handler(|window, surface, event, event_loop| {
        event_loop.set_control_flow(winit::event_loop::ControlFlow::Wait);

        let surface = surface.unwrap();

        match event {
            Event::WindowEvent {
                window_id,
                event: WindowEvent::RedrawRequested,
            } if window_id == window.id() => {
                let PhysicalSize { width, height } = window.inner_size();
                surface
                    .resize(
                        NonZeroU32::new(width).unwrap(),
                        NonZeroU32::new(height).unwrap(),
                    )
                    .unwrap();

                let mut buf = surface.buffer_mut().unwrap();
                let pixels = buf
                    .chunks_exact_mut(width as usize)
                    .enumerate()
                    .flat_map(|(y, row)| row.iter_mut().enumerate().map(move |(x, p)| (x, y, p)));

                let aspect = width as f32 / height as f32;
                let viewport = {
                    let height = 0.5;
                    vec2f(aspect * height, height)
                };
                let focal_len: f32 = 1.0;

                let camera_origin = vec3f(0, 0, 0);

                for (x, y, pixel) in pixels {}
            }
            _ => (),
        }
        todo!();
    });

    // while window.is_open() && !window.is_key_down(Key::Escape) {
    //     if DYNAMIC_SIZE {
    //         let (width, height) = window.get_size();
    //         let (width, height) = (
    //             width / scale_to_int(SCALE) as usize,
    //             height / scale_to_int(SCALE) as usize,
    //         );
    //         buf.resize(width, height);
    //     }
    //
    //     let camera_origin = 'cam: {
    //         let Some(mut pos) = window
    //             .get_mouse_pos(minifb::MouseMode::Discard)
    //             .map(<(f32, f32) as Into<Vec2<f32>>>::into)
    //         else {
    //             break 'cam vec3f(0, 0, 0);
    //         };
    //         pos.x /= buf.width() as f32;
    //         pos.y /= buf.height() as f32;
    //         pos -= vec2f(0.5, 0.5);
    //         pos.x *= -2.0;
    //         pos.y *= 2.0;
    //         vec3f(pos.x, 0, pos.y)
    //         // vec3f(0, 0, 0)
    //     };
    //     let viewport_pixel_pos = {
    //         let delta = viewport.x / buf.width() as f32;
    //         let origin = vec3f(-viewport.x / 2.0, focal_len, viewport.y / 2.0) + camera_origin;
    //         move |x: usize, y: usize| origin + vec3f(delta * x as f32, 0, -delta * y as f32)
    //     };
    //     let dim = buf.dimensions();
    //     let start = Instant::now();
    //     buf.inner_buf_mut()
    //         .par_chunks_exact_mut(dim.x)
    //         .enumerate()
    //         // Run scanlines in parallel
    //         .for_each(|(y, line)| {
    //             line.iter_mut().enumerate().for_each(move |(x, p)| {
    //                 let ray_target = viewport_pixel_pos(x, y);
    //                 let ray = camera_origin.to(ray_target);
    //                 *p = ray_color(&ray, ..1000.).into_rgba();
    //             });
    //         });
    //     // // Run pixels in parallel
    //     // .flat_map(|(y, line)| line.par_iter_mut().enumerate().map(move |(x, p)| (x, y, p)))
    //     // .for_each(|(x, y, p)| {
    //     //     let ray_target = viewport_pixel_pos(x, y);
    //     //     let ray = camera_origin.to(ray_target);
    //     //     *p = ray_color(&ray);
    //     // });
    //
    //     // Run pixels on one thread
    //     // buf.iter_pos_mut().for_each(|(x, y, p)| {
    //     //     let ray_target = viewport_pixel_pos(x, y);
    //     //     let ray = camera_origin.to(ray_target);
    //     //     // *p = Color::splat(ray.direction().z * 0.5 + 0.5).into_rgba()
    //     //     *p = ray_color(&ray, ..1000.).into_rgba();
    //     // });
    //
    //     let took = start.elapsed();
    //
    //     window
    //         .update_with_buffer(buf.as_rgba(), buf.width(), buf.height())
    //         .unwrap();
    //     println!(
    //         "Rendering took {took:?}, Would allow for {:.1}fps",
    //         1.0 / took.as_secs_f64()
    //     );
    // }
}
