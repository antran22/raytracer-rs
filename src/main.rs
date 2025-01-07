use camera::{Camera, CameraOption};
use object::{HittableList, Sphere};
use std::{fs::File, rc::Rc};
use vec3::Point;

mod camera;
mod interval;
mod object;
mod ray;
mod utils;
mod vec3;

const SPHERE1: Sphere = Sphere {
    center: Point::val(0.0, 0.0, -1.0),
    radius: 0.5,
};

const SPHERE2: Sphere = Sphere {
    center: Point::val(0.0, -100.5, -2.0),
    radius: 100.0,
};

fn main() {
    let mut world = HittableList::empty();
    world.add(Rc::new(SPHERE1));
    world.add(Rc::new(SPHERE2));

    let camera = Camera::new(CameraOption {
        image_width: 400,
        image_height: (400.0 / (16.0 / 9.0)) as i32,
        viewport_height: 2.0,
        position: Point::val(0.0, 0.0, 0.0),
        focal_length: 1.0,
        samples_per_pixel: 10,
        max_depth: 50,
    });

    let mut file = File::create("./output/image.ppm").expect("cannot open file");

    camera.render(&world, &mut file);
}
