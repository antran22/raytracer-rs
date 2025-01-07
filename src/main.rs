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

const MAT_GROUND: Lambertian = Lambertian::new(&Color::val(0.8, 0.8, 0.0));
const MAT_CENTER: Lambertian = Lambertian::new(&Color::val(0.1, 0.2, 0.5));
const MAT_LEFT: Metal = Metal::new(&Color::val(0.8, 0.8, 0.8));
const MAT_RIGHT: Metal = Metal::new(&Color::val(0.8, 0.6, 0.2));

fn main() {
    let mut world = HittableList::empty();

    world.add(Rc::new(Sphere {
        center: Point::val(0.0, -100.5, -1.0),
        radius: 100.0,
        material: Rc::new(MAT_GROUND),
    }));
    world.add(Rc::new(Sphere {
        center: Point::val(0.0, 0.0, -1.2),
        radius: 0.5,
        material: Rc::new(MAT_CENTER),
    }));
    world.add(Rc::new(Sphere {
        center: Point::val(-1.0, 0.0, -1.0),
        radius: 0.5,
        material: Rc::new(MAT_LEFT),
    }));
    world.add(Rc::new(Sphere {
        center: Point::val(1.0, 0.0, -1.0),
        radius: 0.5,
        material: Rc::new(MAT_RIGHT),
    }));


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
