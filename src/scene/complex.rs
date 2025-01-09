use std::sync::Arc;

use crate::{
    material::{Dielectric, Lambertian, Material, Metal},
    object::{HittableList, Sphere},
    utils::{rand_double, rand_range_double},
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
pub fn construct_complex_scene() -> HittableList {
    let mut world = HittableList::empty();

    let mat_ground = Lambertian::new(Color::new(0.3, 0.3, 0.3));
    world.add(Sphere::stationary(
        Point::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(mat_ground),
    ));

    // place marble
    for a in -11..12 {
        for b in -11..12 {
            let marble_position = sample_marble_position(a, b);
            let should_be_moving = rand_double() > 0.8;

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

            if should_be_moving {
                let end_position = marble_position + Vec3::new(0.0, rand_range_double(0.0..0.5), 0.0);
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

    let mat2 = Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Sphere::stationary(Point::new(-4.0, 1.0, 0.0), 1.0, mat2));

    let mat3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Sphere::stationary(Point::new(4.0, 1.0, 0.0), 1.0, mat3));

    world
}
