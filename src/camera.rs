use crate::{
    interval::Interval,
    material::MaterialInteractResult,
    object::Hittable,
    ray::Ray,
    utils::{rand_double, rand_vector_in_unit_disk},
    vec3::{Color, Point, Vec3},
};

const WHITE: Color = Color::new(1.0, 1.0, 1.0);
const BLUE: Color = Color::new(0.5, 0.7, 1.0);
const BLACK: Color = Color::zero();

const RAY_INTERVAL: Interval = Interval {
    min: 0.001,
    max: f64::INFINITY,
};

pub struct OutputQuality {
    pub image_width: u32,
    pub image_height: u32,
    pub samples_per_pixel: u32,
    pub max_depth: u32,
}

pub struct CameraOption {
    pub vfov: f64,

    pub look_from: Point,
    pub look_at: Point,
    pub vup: Vec3,

    pub defocus_angle: f64,
    pub focus_distance: f64,
    pub quality: OutputQuality,
    pub bg_color: Color,
}

#[derive(Clone, Copy)]
pub struct Camera {
    // Basis vector
    position: Point,

    pixel00_loc: Point,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,

    samples_per_pixel: u32,
    pixel_samples_scale: f64,
    max_depth: u32,

    defocus_angle: f64,

    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
    bg_color: Color,
}

impl Camera {
    pub fn new(opt: CameraOption) -> Camera {
        let OutputQuality {
            image_width,
            image_height,
            samples_per_pixel,
            max_depth,
        } = opt.quality;
        let h = (opt.vfov.to_radians() / 2.0).tan();

        let w = (opt.look_from - opt.look_at).to_unit();
        let u = Vec3::cross(&opt.vup, &w);
        let v = Vec3::cross(&w, &u);

        let viewport_height = 2.0 * h * opt.focus_distance;
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);

        let viewport_u = viewport_width * u;
        let viewport_v = -viewport_height * v;

        let pixel_delta_u = viewport_u / (image_width as f64);
        let pixel_delta_v = viewport_v / (image_height as f64);

        let viewport_upper_left =
            opt.look_from - opt.focus_distance * w - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

        let pixel_samples_scale = 1.0 / samples_per_pixel as f64;

        let defocus_radius = opt.focus_distance * (opt.defocus_angle / 2.0).to_radians().tan();
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;

        Camera {
            bg_color: opt.bg_color,
            position: opt.look_from,

            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,

            samples_per_pixel,
            pixel_samples_scale,
            max_depth,

            defocus_disk_u,
            defocus_disk_v,
            defocus_angle: opt.defocus_angle,
        }
    }

    fn ray_color<T: Hittable>(&self, object: &T, ray: &Ray, depth: u32) -> Color {
        if depth == 0 {
            return BLACK;
        }
        if let Some(record) = object.hit(ray, &RAY_INTERVAL) {
            return match record.material.interact(ray, &record) {
                MaterialInteractResult::Scatter { attenuation, ray } => {
                    attenuation * self.ray_color(object, &ray, depth - 1)
                }
                MaterialInteractResult::Emitted { color } => color,
                MaterialInteractResult::None => Color::BLACK,
            };
        }
        // when the ray doesn't hit anything, return the background color
        self.bg_color
    }

    pub fn project_ray<T: Hittable>(&self, i: u32, j: u32, world: &T) -> Color {
        let mut color = Color::zero();
        for _sample in 0..self.samples_per_pixel {
            let ray = self.get_ray(i, j);
            color += self.ray_color(world, &ray, self.max_depth);
        }
        color * self.pixel_samples_scale
    }

    fn get_ray(&self, i: u32, j: u32) -> Ray {
        let offset = Self::sample_square();
        let pixel_sample = self.pixel00_loc
            + (i as f64 + offset.x) * self.pixel_delta_u
            + (j as f64 + offset.y) * self.pixel_delta_v;
        let ray_origin = if self.defocus_angle <= 0.0 {
            self.position
        } else {
            self.sample_defocus_disk()
        };

        Ray {
            origin: ray_origin,
            dir: pixel_sample - ray_origin,
            time: rand_double(),
        }
    }

    fn sample_square() -> Vec3 {
        Vec3 {
            x: rand_double() - 0.5,
            y: rand_double() - 0.5,
            z: 0.0,
        }
    }

    fn sample_defocus_disk(&self) -> Point {
        let p = rand_vector_in_unit_disk();
        self.position + (p.x * self.defocus_disk_u) + (p.y * self.defocus_disk_v)
    }
}
