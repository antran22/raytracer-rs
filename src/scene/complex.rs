use std::sync::Arc;

use crate::{
    camera::{Camera, CameraOption, OutputQuality},
    material::{Dielectric, Lambertian, Material, Metal},
    object::{HittableList, Sphere},
    texture::CheckeredTexture,
    utils::{rand_double, rand_range},
    vec3::{Color, Point, Vec3},
};

const MARBLE_GROUND_CENTER: Point = Point::new(4.0, 0.2, 0.0);
fn sample_marble_position(a: i32, b: i32) -> Vec3 {
    loop {
        let marble_position = Point::new(
            a as f64 + 0.9 * rand_double(),
            0.2,
            b as f64 + 0.9 * rand_double(),
        );
        if marble_position.dist(MARBLE_GROUND_CENTER) > 0.9 {
            return marble_position;
        }
    }
}
pub fn construct_complex_scene(
    moving_probability: f64,
    output_quality: OutputQuality,
) -> (HittableList, Camera) {
    let mut world = HittableList::empty();

    let mat_ground = Lambertian::new(Arc::new(CheckeredTexture::new_from_colors(
        0.3,
        Color::new(0.2, 0.3, 0.1),
        Color::all(0.9),
    )));
    world.add(Sphere::stationary(
        Point::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(mat_ground),
    ));

    // place marble
    for a in -11..12 {
        for b in -11..12 {
            let marble_position = sample_marble_position(a, b);
            let should_be_moving = rand_double() <= moving_probability;

            let sphere_material: Arc<dyn Material + Send + Sync> = match rand_double() {
                0.0..=0.8 => {
                    // lambertian
                    let albedo = Color::rand() * Color::rand();
                    Arc::new(Lambertian::new_solid_color(albedo))
                }
                0.8..=0.95 => {
                    // metal
                    let albedo = Color::rand_range(0.5..1.0);
                    let fuzz = rand_range(0.0..0.5);
                    Arc::new(Metal::new(albedo, fuzz))
                }
                _ => {
                    // glass
                    let ref_idx = rand_range(1.33..1.5);
                    Arc::new(Dielectric::new(ref_idx))
                }
            };

            if should_be_moving {
                let end_position = marble_position + Vec3::new(0.0, rand_range(0.0..0.5), 0.0);
                world.add(Sphere::moving(
                    marble_position,
                    end_position,
                    0.2,
                    sphere_material,
                ))
            } else {
                world.add(Sphere::stationary(marble_position, 0.2, sphere_material));
            }
        }
    }

    let mat1 = Arc::new(Dielectric::new(1.5));
    world.add(Sphere::stationary(Point::new(0.0, 1.0, 0.0), 1.0, mat1));
    let mat1 = Arc::new(Dielectric::new(1.0 / 1.5));
    world.add(Sphere::stationary(Point::new(0.0, 1.0, 0.0), 0.9, mat1));

    let mat2 = Arc::new(Lambertian::new_solid_color(Color::new(0.4, 0.2, 0.1)));
    world.add(Sphere::stationary(Point::new(-4.0, 1.0, 0.0), 1.0, mat2));

    let mat3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Sphere::stationary(Point::new(4.0, 1.0, 0.0), 1.0, mat3));

    let camera: Camera = Camera::new(CameraOption {
        bg_color: Color::new(0.7, 0.8, 1.0),
        vfov: 20.0,
        look_from: Point::new(13.0, 2.0, 3.0),
        look_at: Point::new(0.0, 0.0, 0.0),
        vup: Vec3::new(0.0, 1.0, 0.0),
        defocus_angle: 0.0,
        focus_distance: 10.0,
        quality: output_quality,
    });

    (world, camera)
}
