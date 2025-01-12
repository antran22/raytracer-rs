use camera::{Camera, CameraOption};
use image::ImageBuffer;
use object::{BVHTree, HittableList};
use rayon::prelude::*;
use std::{
    sync::{mpsc, Arc},
    thread,
    time::Instant,
};
use vec3::{Color, Point, Vec3};

mod camera;
mod interval;
mod material;
mod object;
mod ray;
mod scene;
mod utils;
mod vec3;

fn main() {
    let start = Instant::now();

    let deep_render = false;
    let (image_width, samples, max_depth) = if deep_render {
        (1200, 50, 50)
    } else {
        (400, 10, 10)
    };
    let image_height = (image_width as f64 / (16.0 / 9.0)) as u32;
    let world = scene::construct_complex_scene(0.0);
    let world = BVHTree::from_list(world.objects());
    let camera: Camera = Camera::new(CameraOption {
        image_width,
        image_height,
        vfov: 20.0,
        samples_per_pixel: samples,
        max_depth,
        look_from: Point::new(13.0, 2.0, 3.0),
        look_at: Point::new(0.0, 0.0, 0.0),
        vup: Vec3::new(0.0, 1.0, 0.0),
        defocus_angle: 0.0,
        focus_distance: 10.0,
    });

    let (tx, rx) = mpsc::channel::<(u32, u32, Color)>();

    let total_pixel = image_height * image_width;

    let handle = thread::spawn(move || {
        let mut img = ImageBuffer::new(image_width, image_height);
        for (idx, (x, y, color)) in rx.iter().enumerate() {
            if idx % 1000 == 999 {
                let duration = Instant::now().duration_since(start);
                eprint!(
                    "\rProcessed: {}/{} pixels. Elapsed: {}s             ",
                    idx + 1,
                    total_pixel,
                    duration.as_secs_f64()
                );
            }
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

            let color = camera.project_ray(x, y, &world);
            tx.send((x, y, color)).expect("cannot notify progress");
        });

    drop(tx);

    handle.join().expect("cannot join");

    let end = Instant::now();
    let duration = end.duration_since(start);
    eprintln!("\nExecution duration: {}s", duration.as_secs_f64());
}
