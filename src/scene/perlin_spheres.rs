use std::sync::Arc;

use crate::{
    camera::{Camera, CameraOption, OutputQuality},
    material::Lambertian,
    object::{HittableList, Sphere},
    texture::NoiseTexture,
    vec3::{Point, Vec3},
};

pub fn construct_perlin_spheres(quality: OutputQuality) -> (HittableList, Camera) {
    let mut world = HittableList::empty();

    let perlin_texture = NoiseTexture::new_perlin(4.0);
    let mat_perlin = Arc::new(Lambertian::new(Arc::new(perlin_texture)));
    world.add(Sphere::stationary(
        Point::new(0.0, -1000.0, 0.0),
        1000.0,
        mat_perlin.clone(),
    ));

    world.add(Sphere::stationary(
        Point::new(0.0, 2.0, 0.0),
        2.0,
        mat_perlin.clone(),
    ));

    let camera: Camera = Camera::new(CameraOption {
        vfov: 20.0,
        look_from: Point::new(12.0, 2.0, 3.0),
        look_at: Point::new(0.0, 0.0, 0.0),
        vup: Vec3::new(0.0, 1.0, 0.0),
        defocus_angle: 0.0,
        focus_distance: 10.0,
        quality,
    });

    (world, camera)
}
