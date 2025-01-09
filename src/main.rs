use camera::{Camera, CameraOption};
use image::ImageBuffer;
use material::{Dielectric, Lambertian, Material, Metal};
use object::{HittableList, Sphere};
use rayon::prelude::*;
use std::{any::Any, rc::Rc, sync::{mpsc, Arc}, thread, time::Instant};
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

fn construct_world() -> HittableList {
    let mut world = HittableList::empty();

    let mat_ground = Lambertian::new(Color::val(0.3, 0.3, 0.3));
    world.add(Sphere::new(
        Point::val(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(mat_ground),
    ));

    // place marble
    for a in -11..12 {
        for b in -11..12 {
            let marble_position = sample_marble_position(a, b);

            let sphere_material: Arc<dyn Material + Send + Sync> = match rand_double() {
                0.0..=0.8 => {
                    // lambertian
                    let albedo = Color::rand() * Color::rand();
                    Arc::new(Lambertian::new(albedo))
                }
                0.8..=0.95 => {
                    // metal
                    let albedo = Color::rand_range(0.5..1.0);
                    let fuzz = rand_range_double(0.0..0.5);
                    Arc::new(Metal::new(albedo, fuzz))
                }
                _ => {
                    // glass
                    let ref_idx = rand_range_double(1.33..1.5);
                    Arc::new(Dielectric::new(ref_idx))
                }
            };

            world.add(Sphere {
                center: marble_position,
                radius: 0.2,
                material: sphere_material,
            })
        }
    }

    let mat1 = Arc::new(Dielectric::new(1.5));
    world.add(Sphere {
        center: Point::val(0.0, 1.0, 0.0),
        radius: 1.0,
        material: mat1,
    });

    let mat2 = Arc::new(Lambertian::new(Color::val(0.4, 0.2, 0.1)));
    world.add(Sphere {
        center: Point::val(-4.0, 1.0, 0.0),
        radius: 1.0,
        material: mat2,
    });

    let mat3 = Arc::new(Metal::new(Color::val(0.7, 0.6, 0.5), 0.0));
    world.add(Sphere {
        center: Point::val(4.0, 1.0, 0.0),
        radius: 1.0,
        material: mat3,
    });

    world
}

fn main() {
    let start = Instant::now();
    let image_width = 400;
    let image_height = (image_width as f64 / (16.0 / 9.0)) as u32;
    let world = Arc::new(construct_world());
    let camera: Camera = Camera::new(CameraOption {
        image_width,
        image_height,
        vfov: 20.0,
        samples_per_pixel: 10,
        max_depth: 50,
        look_from: Point::val(13.0, 2.0, 3.0),
        look_at: Point::val(0.0, 0.0, 0.0),
        vup: Vec3::val(0.0, 1.0, 0.0),
        defocus_angle: 0.6,
        focus_distance: 10.0,
    });

    let (tx, rx) = mpsc::channel::<(u32, u32, Color)>();

    let total_pixel = image_height * image_width;

    let handle = thread::spawn(move || {
        let mut img = ImageBuffer::new(image_width, image_height);
        for (idx, (x, y, color)) in rx.iter().enumerate() {
            eprint!("\rProcessed: {}/{} pixels", idx + 1, total_pixel);
            img.put_pixel(x, y, color.to_rgb());
        }
        img.save("./output/image.png").expect("cannot write image");
    });

    // Create iterator over all pixels
    (0..total_pixel)
        .into_par_iter() // Convert to parallel iterator
        .for_each(|i| {
            let x = i % image_width as u32;
            let y = i / image_width as u32;
            
            let color = camera.project_ray::<HittableList>(x, y, &world);
            tx.send((x, y, color)).expect("cannot notify progress");
        });
    
    drop(tx);
    
    handle.join().expect("cannot join");

    let end = Instant::now();
    let duration = end.duration_since(start);
    eprintln!("\nExecution duration: {}s", duration.as_secs_f64());

}
