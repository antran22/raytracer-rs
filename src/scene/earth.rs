use std::sync::Arc;

use crate::{
    camera::{Camera, CameraOption, OutputQuality},
    material::Lambertian,
    object::{HittableList, Sphere},
    texture::{CheckeredTexture, ImageTexture},
    vec3::{Color, Point, Vec3},
};

pub fn construct_earth_scene(quality: OutputQuality) -> (HittableList, Camera) {
    let mut world = HittableList::empty();

    let earth_texture = ImageTexture::new("earthmap.jpg");
    let mat_earth = Lambertian::new(Arc::new(earth_texture));
    world.add(Sphere::stationary(
        Point::new(0.0, 3.0, 0.0),
        2.0,
        Arc::new(mat_earth),
    ));

    let moon_texture = ImageTexture::new("moonmap.jpg");
    let mat_moon = Lambertian::new(Arc::new(moon_texture));
    world.add(Sphere::stationary(
        Point::new(0.0, 2.0, 3.0),
        1.0,
        Arc::new(mat_moon),
    ));

    let mat_ground = Lambertian::new_solid_color(Color::all(0.2));
    world.add(Sphere::stationary(
        Point::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(mat_ground),
    ));

    let camera: Camera = Camera::new(CameraOption {
        vfov: 20.0,
        look_from: Point::new(12.0, 3.0, 12.0),
        look_at: Point::new(0.0, 3.0, 0.0),
        vup: Vec3::new(0.0, 1.0, 0.0),
        defocus_angle: 0.0,
        focus_distance: 10.0,
        quality,
    });

    (world, camera)
}
