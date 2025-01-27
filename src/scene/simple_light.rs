use std::sync::Arc;

use crate::{
    camera::{Camera, CameraOption, OutputQuality},
    material::{DiffuseLight, Lambertian},
    object::{HittableList, Quad, Sphere},
    texture::NoiseTexture,
    vec3::{Color, Point, Vec3},
};

pub fn construct_simple_light(quality: OutputQuality) -> (HittableList, Camera) {
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

    let diff_light = Arc::new(DiffuseLight::new_from_color(Color::all(4.0)));
    world.add(Quad::new(
        Point::new(3.0, 1.0, -2.0),
        Vec3::new(2.0, 0.0, 0.0),
        Vec3::new(0.0, 2.0, 0.0),
        diff_light,
    ));

    let camera: Camera = Camera::new(CameraOption {
        bg_color: Color::BLACK,
        vfov: 20.0,
        look_from: Point::new(26.0, 3.0, 6.0),
        look_at: Point::new(0.0, 2.0, 0.0),
        vup: Vec3::new(0.0, 1.0, 0.0),
        defocus_angle: 0.0,
        focus_distance: 10.0,
        quality,
    });

    (world, camera)
}
