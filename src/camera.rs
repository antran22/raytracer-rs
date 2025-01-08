use std::io::{self, Write};

use crate::{
    interval::Interval,
    material::ScatterResult,
    object::Hittable,
    ray::Ray,
    utils::rand_double,
    vec3::{Color, Point, Vec3},
};

const WHITE: Color = Color::val(1.0, 1.0, 1.0);
const BLUE: Color = Color::val(0.5, 0.7, 1.0);
const BLACK: Color = Color::zero();

const RAY_INTERVAL: Interval = Interval {
    min: 0.001,
    max: f64::INFINITY,
};

pub struct CameraOption {
    pub image_width: i32,
    pub image_height: i32,

    pub vfov: f64,

    pub look_from: Point,
    pub look_at: Point,
    pub vup: Vec3,

    pub samples_per_pixel: i32,
    pub max_depth: i32,
}

pub struct Camera {
    image_width: i32,
    image_height: i32,

    // Basis vector
    position: Point,

    pixel00_loc: Point,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,

    samples_per_pixel: i32,
    pixel_samples_scale: f64,
    max_depth: i32,
}

impl Camera {
    pub fn new(opt: CameraOption) -> Camera {
        let h = (opt.vfov.to_radians() / 2.0).tan();

        let w = (opt.look_from - opt.look_at).to_unit();
        let u = Vec3::cross(&opt.vup, &w);
        let v = Vec3::cross(&w, &u);
        
        let focal_length = (opt.look_from - opt.look_at).length();
        
        let viewport_height = 2.0 * h * focal_length;
        let viewport_width = viewport_height * (opt.image_width as f64 / opt.image_height as f64);

        let viewport_u = viewport_width * u;
        let viewport_v = -viewport_height * v;

        let pixel_delta_u = viewport_u / (opt.image_width as f64);
        let pixel_delta_v = viewport_v / (opt.image_height as f64);

        let viewport_upper_left =
            opt.look_from - focal_length * w - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

        let pixel_samples_scale = 1.0 / opt.samples_per_pixel as f64;

        Camera {
            image_width: opt.image_width,
            image_height: opt.image_height,

            position: opt.look_from,

            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,

            samples_per_pixel: opt.samples_per_pixel,
            pixel_samples_scale,
            max_depth: opt.max_depth,
        }
    }

    fn ray_color(object: &dyn Hittable, ray: &Ray, depth: i32) -> Color {
        if depth <= 0 {
            return BLACK;
        }
        if let Some(record) = object.hit(ray, &RAY_INTERVAL) {
            if let Some(scatter_result) = record.material.scatter(ray, &record) {
                let ScatterResult {
                    attenuation,
                    ray: scattered_ray,
                } = scatter_result;

                return attenuation * Camera::ray_color(object, &scattered_ray, depth - 1);
            }

            return Color::BLACK;
        }
        let unit_dir = ray.dir.to_unit();
        let a = 0.5 * (unit_dir.y + 1.0);
        return (1.0 - a) * WHITE + a * BLUE;
    }

    fn get_ray(&self, i: i32, j: i32) -> Ray {
        let offset = Camera::sample_square();
        let pixel_sample = self.pixel00_loc
            + (i as f64 + offset.x) * self.pixel_delta_u
            + (j as f64 + offset.y) * self.pixel_delta_v;

        Ray {
            origin: self.position,
            dir: pixel_sample - self.position,
        }
    }

    fn sample_square() -> Vec3 {
        Vec3 {
            x: rand_double() - 0.5,
            y: rand_double() - 0.5,
            z: 0.0,
        }
    }

    pub fn render(&self, world: &dyn Hittable, output: &mut dyn Write) {
        write!(
            output,
            "P3\n{0} {1}\n255\n",
            self.image_width, self.image_height
        )
        .expect("cannot printout header");

        for j in 0..self.image_height {
            eprint!("\rScanlines remaining: {}        ", self.image_height - j);
            io::stderr().flush().expect("Unable to flush stderr");
            for i in 0..self.image_width {
                let mut color = Color::zero();
                for _sample in 0..self.samples_per_pixel {
                    let ray = self.get_ray(i, j);
                    color += Camera::ray_color(world, &ray, self.max_depth);
                }
                color *= self.pixel_samples_scale;
                color.print_color(output).expect("cannot printout color");
            }
        }

        eprintln!("\nDone");
    }
}
