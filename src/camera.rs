use std::io::{self, Write};

use crate::{
    interval,
    object::Hittable,
    ray::Ray,
    vec3::{Color, Point, Vec3},
};

const WHITE: Color = Color::val(1.0, 1.0, 1.0);
const BLUE: Color = Color::val(0.5, 0.7, 1.0);

#[derive(Default)]
pub struct Camera {
    pub aspect_ratio: f64,

    pub image_width: i32,
    pub image_height: i32,

    pub viewport_height: f64,
    pub viewport_width: f64,
    pub position: Point,
    pub focal_length: f64,

    pub pixel00_loc: Point,
    pub pixel_delta_u: Vec3,
    pub pixel_delta_v: Vec3,
}
impl Camera {
    pub fn initialize(&mut self) {
        self.image_height = i32::max((self.image_width as f64 / self.aspect_ratio) as i32, 1);
        self.viewport_width =
            self.viewport_height * (self.image_width as f64 / self.image_height as f64);

        let viewport_u = Vec3::val(self.viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::val(0.0, -self.viewport_height, 0.0);

        self.pixel_delta_u = viewport_u / (self.image_width as f64);
        self.pixel_delta_v = viewport_v / (self.image_height as f64);

        let viewport_upper_left = self.position
            - Vec3::val(0.0, 0.0, self.focal_length)
            - (viewport_u / 2.0)
            - (viewport_v / 2.0);
        self.pixel00_loc = viewport_upper_left + (self.pixel_delta_u + self.pixel_delta_v) * 0.5;
    }

    fn ray_color(object: &dyn Hittable, ray: &Ray) -> Color {
        match object.hit(ray, &interval::POSITIVE) {
            None => {
                let unit_dir = ray.dir.unit();
                let a = 0.5 * (unit_dir.y + 1.0);
                (1.0 - a) * WHITE + a * BLUE
            }
            Some(record) => {
                let normal = record.normal;
                Color::val(normal.x + 1.0, normal.y + 1.0, normal.z + 1.0) * 0.5
            }
        }
    }

    pub fn render(&self, world: &dyn Hittable, output: &mut dyn Write) {
        write!(
            output,
            "P3\n{0} {1}\n255\n",
            self.image_width, self.image_height
        ).expect("cannot printout header");

        for j in 0..self.image_height {
            eprint!("\rScanlines remaining: {}", self.image_height - j);
            io::stderr().flush().expect("Unable to flush stderr");
            for i in 0..self.image_width {
                let pixel_center = self.pixel00_loc
                    + self.pixel_delta_u * (i as f64)
                    + self.pixel_delta_v * (j as f64);
                let ray_direction = pixel_center - self.position;
                let r = Ray {
                    origin: self.position,
                    dir: ray_direction,
                };

                let color = Camera::ray_color(world, &r);
                color.print_color(output).expect("cannot printout color");
            }
        }

        eprintln!("\nDone");
    }
}
