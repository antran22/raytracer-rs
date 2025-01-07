use object::{Hittable, HittableList, Sphere};
use ray::Ray;
use std::{
    io::{self, Write},
    rc::Rc,
};
use vec3::{Color, Point, Vec3};

mod object;
mod ray;
mod vec3;
mod interval;

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const WHITE: Color = Color::val(1.0, 1.0, 1.0);
const BLUE: Color = Color::val(0.5, 0.7, 1.0);
const SPHERE1: Sphere = Sphere {
    center: Point::val(1.0, 0.0, -1.0),
    radius: 0.5,
};

const SPHERE2: Sphere = Sphere {
    center: Point::val(0.0, -100.5, -2.0),
    radius: 100.0,
};

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

fn main() {
    let mut objects = HittableList::empty();
    objects.add(Rc::new(SPHERE1));
    objects.add(Rc::new(SPHERE2));

    let image_width: i32 = 400;
    let image_height: i32 = (400 as f64 / ASPECT_RATIO) as i32;

    // Camera

    let focal_length = 1.0;
    let viewport_height: f64 = 2.0;
    let viewport_width: f64 = viewport_height * (image_width as f64 / image_height as f64);
    let camera_center = Point::zero();

    // Calculate the vectors across the horizontal and down the vertical view port edges
    let viewport_u = Vec3::val(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::val(0.0, -viewport_height, 0.0);

    // Calculate the horizontal & vertical delta vectors from pixel to pixel
    let pixel_delta_u = viewport_u / (image_width as f64);
    let pixel_delta_v = viewport_v / (image_height as f64);

    // Calculate the location of the upper left pixel
    let viewport_upper_left =
        camera_center - Vec3::val(0.0, 0.0, focal_length) - (viewport_u / 2.0) - (viewport_v / 2.0);
    let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

    print!("P3\n{image_width} {image_height}\n255\n");
    let stdout = &mut io::stdout();
    for j in 0..image_height {
        eprint!("\rScanlines remaining: {}", image_height - j);
        io::stderr().flush().expect("Unable to flush stderr");
        for i in 0..image_width {
            let pixel_center =
                pixel00_loc + pixel_delta_u * (i as f64) + pixel_delta_v * (j as f64);
            let ray_direction = pixel_center - camera_center;
            let r = Ray {
                origin: camera_center,
                dir: ray_direction,
            };

            let color = ray_color(&objects, &r);
            color.print_color(stdout).expect("cannot printout color");
        }
    }
    eprintln!("\nDone");
}
