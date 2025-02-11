use std::sync::Arc;

use crate::{
    camera::{Camera, CameraOption, OutputQuality},
    material::{DiffuseLight, Lambertian},
    object::{HittableList, Quad, Transformable},
    vec3::{Color, Point, Vec3},
};

pub fn construct_cornell_box(quality: OutputQuality) -> (HittableList, Camera) {
    let mut world = HittableList::empty();

    let red = Arc::new(Lambertian::new_solid_color(Color::new(0.65, 0.05, 0.05)));
    let white = Arc::new(Lambertian::new_solid_color(Color::new(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::new_solid_color(Color::new(0.12, 0.45, 0.15)));
    let light = Arc::new(DiffuseLight::new_from_color(Color::new(15.0, 15.0, 15.0)));

    world.add(Quad::new(
        Point::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        green,
    ));
    world.add(Quad::new(
        Point::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        red,
    ));
    world.add(Quad::new(
        Point::new(343.0, 554.0, 332.0),
        Vec3::new(-130.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -105.0),
        light,
    ));
    world.add(Quad::new(
        Point::all(0.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        white.clone(),
    ));
    world.add(Quad::new(
        Point::all(555.0),
        Vec3::new(-555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -555.0),
        white.clone(),
    ));
    world.add(Quad::new(
        Point::new(0.0, 0.0, 555.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        white.clone(),
    ));

    world.add(
        HittableList::rectangular_box(
            &Point::new(0.0, 0.0, 0.0),
            &Point::new(165.0, 330.0, 165.0),
            white.clone(),
        )
        .rotate_y(15.0)
        .translate(Vec3::new(265.0, 0.0, 295.0)),
    );

    world.add(
        HittableList::rectangular_box(
            &Point::new(0.0, 0.0, 0.0),
            &Point::new(165.0, 165.0, 165.0),
            white.clone(),
        )
        .rotate_y(-18.0)
        .translate(Vec3::new(130.0, 0.0, 65.0)),
    );

    let camera: Camera = Camera::new(CameraOption {
        bg_color: Color::BLACK,
        vfov: 40.0,
        look_from: Point::new(278.0, 278.0, -800.0),
        look_at: Point::new(278.0, 278.0, 0.0),
        vup: Vec3::new(0.0, 1.0, 0.0),
        defocus_angle: 0.0,
        focus_distance: 10.0,
        quality,
    });

    (world, camera)
}
