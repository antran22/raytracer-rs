use camera::Camera;
use object::{HittableList, Sphere};
use std::{fs::File, rc::Rc};
use vec3::Point;

mod camera;
mod interval;
mod object;
mod ray;
mod vec3;

const SPHERE1: Sphere = Sphere {
    center: Point::val(1.0, 0.0, -1.0),
    radius: 0.5,
};

const SPHERE2: Sphere = Sphere {
    center: Point::val(0.0, -100.5, -2.0),
    radius: 100.0,
};


const SPHERE3: Sphere = Sphere {
    center: Point::val(-10.0, 0.0, -10.0),
    radius: 10.0,
};

fn main() {
    let mut world = HittableList::empty();
    world.add(Rc::new(SPHERE1));
    world.add(Rc::new(SPHERE2));
    world.add(Rc::new(SPHERE3));

    let mut camera = Camera {
        aspect_ratio: 16.0 / 9.0,
        image_width: 600,
        viewport_height: 2.0,
        position: Point::val(0.0, 0.0, 0.0),
        focal_length: 0.5,
        ..Default::default()
    };

    camera.initialize();

    let mut file = File::create("./output/image.ppm").expect("cannot open file");

    camera.render(&world, &mut file);
}
