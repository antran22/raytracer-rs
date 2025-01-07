use camera::{Camera, CameraOption};
use material::{Lambertian, Metal};
use object::{HittableList, Sphere};
use std::{fs::File, rc::Rc};
use vec3::{Color, Point};

mod camera;
mod interval;
mod material;
mod object;
mod ray;
mod utils;
mod vec3;

fn main() {
    let mut world = HittableList::empty();

    let mat_ground = Lambertian::new(&Color::val(0.8, 0.8, 0.0));
    let mat_center = Lambertian::new(&Color::val(0.1, 0.2, 0.5));
    let mat_left = Metal::new(&Color::val(0.8, 0.8, 0.8), 0.3);
    let mat_right = Metal::new(&Color::val(0.8, 0.6, 0.2), 1.0);

    world.add(Rc::new(Sphere {
        center: Point::val(0.0, -100.5, -1.0),
        radius: 100.0,
        material: Rc::new(mat_ground),
    }));
    world.add(Rc::new(Sphere {
        center: Point::val(0.0, 0.0, -1.2),
        radius: 0.5,
        material: Rc::new(mat_center),
    }));
    world.add(Rc::new(Sphere {
        center: Point::val(-1.0, 0.0, -1.0),
        radius: 0.5,
        material: Rc::new(mat_left),
    }));
    world.add(Rc::new(Sphere {
        center: Point::val(1.0, 0.0, -1.0),
        radius: 0.5,
        material: Rc::new(mat_right),
    }));

    let camera = Camera::new(CameraOption {
        image_width: 1000,
        image_height: (1000.0 / (16.0 / 9.0)) as i32,
        viewport_height: 2.0,
        position: Point::val(0.0, 0.0, 0.0),
        focal_length: 1.0,
        samples_per_pixel: 10,
        max_depth: 50,
    });

    let mut file = File::create("./output/image.ppm").expect("cannot open file");

    camera.render(&world, &mut file);
}
