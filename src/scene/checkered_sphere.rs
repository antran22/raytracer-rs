use std::sync::Arc;

use crate::{
    camera::{Camera, CameraOption, OutputQuality},
    material::Lambertian,
    object::{HittableList, Sphere},
    texture::CheckeredTexture,
    vec3::{Color, Point, Vec3},
};

pub fn construct_checkered_sphere_scene(quality: OutputQuality) -> (HittableList, Camera) {
    let mut world = HittableList::empty();

    let mat_ground = Lambertian::new(Arc::new(CheckeredTexture::new_from_colors(
        0.01,
        Color::new(0.2, 0.3, 0.1),
        Color::all(0.9),
    )));

    let mat_ground = Arc::new(mat_ground);

    world.add(Sphere::stationary(
        Point::new(0.0, -10.0, 0.0),
        10.0,
        mat_ground.clone(),
    ));

    world.add(Sphere::stationary(
        Point::new(0.0, 10.0, 0.0),
        10.0,
        mat_ground.clone(),
    ));

    let camera: Camera = Camera::new(CameraOption {
        vfov: 20.0,
        look_from: Point::new(13.0, 2.0, 3.0),
        look_at: Point::new(0.0, 0.0, 0.0),
        vup: Vec3::new(0.0, 1.0, 0.0),
        defocus_angle: 0.0,
        focus_distance: 10.0,
        quality,
    });

    (world, camera)
}
