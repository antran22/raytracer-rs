use std::sync::Arc;

use crate::{
    camera::{Camera, CameraOption, OutputQuality},
    material::Lambertian,
    object::{HittableList, Quad},
    vec3::{Color, Point, Vec3},
};

pub fn construct_quads_scene(quality: OutputQuality) -> (HittableList, Camera) {
    let mut world = HittableList::empty();

    let mat_red = Arc::new(Lambertian::new_solid_color(Color::RED));
    let mat_blue = Arc::new(Lambertian::new_solid_color(Color::BLUE));
    let mat_green = Arc::new(Lambertian::new_solid_color(Color::GREEN));
    let mat_orange = Arc::new(Lambertian::new_solid_color(Color::ORANGE));
    let mat_teal = Arc::new(Lambertian::new_solid_color(Color::TEAL));

    world.add(Quad::new(
        Point::new(-3.0, -2.0, 5.0),
        Vec3::new(0.0, 0.0, -4.0),
        Vec3::new(0.0, 4.0, 0.0),
        mat_red,
    ));

    world.add(Quad::new(
        Point::new(-2.0, -2.0, 0.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 4.0, 0.0),
        mat_green,
    ));

    world.add(Quad::new(
        Point::new(3.0, -2.0, 1.0),
        Vec3::new(0.0, 0.0, 4.0),
        Vec3::new(0.0, 4.0, 0.0),
        mat_blue,
    ));

    world.add(Quad::new(
        Point::new(-2.0, 3.0, 1.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 4.0),
        mat_orange,
    ));

    world.add(Quad::new(
        Point::new(-2.0, -3.0, 5.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -4.0),
        mat_teal,
    ));

    let camera: Camera = Camera::new(CameraOption {
        vfov: 80.0,
        look_from: Point::new(0.0, 0.0, 9.0),
        look_at: Point::new(0.0, 0.0, 0.0),
        vup: Vec3::new(0.0, 1.0, 0.0),
        defocus_angle: 0.0,
        focus_distance: 10.0,
        quality,
    });

    (world, camera)
}
