use args::{Args, Mode, Scene};
use camera::OutputQuality;
use clap::Parser;
use image::ImageBuffer;
use object::BVHTree;
use rayon::prelude::*;
use std::{sync::mpsc, thread, time::Instant};
use vec3::Color;

mod args;
mod camera;
mod interval;
mod material;
mod object;
mod ray;
mod scene;
mod texture;
mod utils;
mod vec3;

fn main() {
    let args = Args::parse();
    let start = Instant::now();

    let (image_width, samples, max_depth) = match args.mode {
        Mode::Slow => (1200, 50, 50),
        Mode::Fast => (400, 10, 50),
    };
    let image_height = (image_width as f64 / (16.0 / 9.0)) as u32;

    let quality = OutputQuality {
        image_width,
        image_height,
        samples_per_pixel: samples,
        max_depth,
    };

    let (world, camera) = match args.scene {
        Scene::Complex => scene::construct_complex_scene(0.1, quality),
        Scene::CheckeredSphere => scene::construct_checkered_sphere_scene(quality),
        Scene::Earth => scene::construct_earth_scene(quality),
        Scene::Perlin => scene::construct_perlin_spheres(quality),
        Scene::Quads => scene::construct_quads_scene(quality),
        Scene::SimpleLight => scene::construct_simple_light(quality),
    };

    let world = BVHTree::from_list(world.objects());

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
            let x = i % image_width;
            let y = i / image_width;

            let color = camera.project_ray(x, y, &world);
            tx.send((x, y, color)).expect("cannot notify progress");
        });

    drop(tx);

    handle.join().expect("cannot join");

    let end = Instant::now();
    let duration = end.duration_since(start);
    eprintln!("\nExecution duration: {}s", duration.as_secs_f64());
}
