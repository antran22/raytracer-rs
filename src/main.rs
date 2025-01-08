use camera::{Camera, CameraOption};
use material::{Dielectric, Lambertian, Material, Metal};
use object::{HittableList, Sphere};
use std::{fs::File, rc::Rc};
use utils::{rand_double, rand_range_double};
use vec3::{Color, Point, Vec3};

mod camera;
mod interval;
mod material;
mod object;
mod ray;
mod utils;
mod vec3;

const MARBLE_GROUND_CENTER: Point = Point::val(4.0, 0.2, 0.0);

fn sample_marble_position(a: i32, b: i32) -> Vec3 {
    loop {
        let marble_position = Point::val(
            a as f64 + 0.9 * rand_double(),
            0.2,
            b as f64 + 0.9 * rand_double(),
        );
        if marble_position.dist(MARBLE_GROUND_CENTER) > 0.9 {
            return marble_position;
        }
    }
}

fn main() {
    let mut world = HittableList::empty();

    let mat_ground = Lambertian::new(Color::val(0.3, 0.3, 0.3));
    world.add(Rc::new(Sphere {
        center: Point::val(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: Rc::new(mat_ground),
    }));

    // place marble
    for a in -11..12 {
        for b in -11..12 {
            let marble_position = sample_marble_position(a, b);

            let sphere_material: Rc<dyn Material> = match rand_double() {
                0.0..=0.8 => {
                    // lambertian
                    let albedo = Color::rand() * Color::rand();
                    Rc::new(Lambertian::new(albedo))
                }
                0.8..=0.95 => {
                    // metal
                    let albedo = Color::rand_range(0.5..1.0);
                    let fuzz = rand_range_double(0.0..0.5);
                    Rc::new(Metal::new(albedo, fuzz))
                }
                _ => {
                    // glass
                    let ref_idx = rand_range_double(1.33..1.5);
                    Rc::new(Dielectric::new(ref_idx))
                }
            };

            world.add(Rc::new(Sphere {
                center: marble_position,
                radius: 0.2,
                material: sphere_material,
            }))
        }
    }

    let mat1 = Rc::new(Dielectric::new(1.5));
    world.add(Rc::new(Sphere {
        center: Point::val(0.0, 1.0, 0.0),
        radius: 1.0,
        material: mat1,
    }));

    let mat2 = Rc::new(Lambertian::new(Color::val(0.4, 0.2, 0.1)));
    world.add(Rc::new(Sphere {
        center: Point::val(-4.0, 1.0, 0.0),
        radius: 1.0,
        material: mat2,
    }));

    let mat3 = Rc::new(Metal::new(Color::val(0.7, 0.6, 0.5), 0.0));
    world.add(Rc::new(Sphere {
        center: Point::val(4.0, 1.0, 0.0),
        radius: 1.0,
        material: mat3,
    }));

    let image_width = 1200;
    let camera = Camera::new(CameraOption {
        image_width: image_width,
        image_height: (image_width as f64 / (16.0 / 9.0)) as i32,
        vfov: 20.0,
        samples_per_pixel: 50,
        max_depth: 50,
        look_from: Point::val(13.0, 2.0, 3.0),
        look_at: Point::val(0.0, 0.0, 0.0),
        vup: Vec3::val(0.0, 1.0, 0.0),
        defocus_angle: 0.6,
        focus_distance: 10.0,
    });

    let mut file = File::create("./output/image.ppm").expect("cannot open file");

    camera.render(&world, &mut file);
}
