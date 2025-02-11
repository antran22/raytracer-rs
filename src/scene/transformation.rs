use std::sync::Arc;

use crate::{
    camera::{Camera, CameraOption, OutputQuality},
    material::Lambertian,
    object::{HittableList, Sphere, Transformable},
    texture::{ImageTexture, SolidColorTexture},
    vec3::{Color, Point, Vec3},
};

pub fn construct_transformation_debug_scene(quality: OutputQuality) -> (HittableList, Camera) {
    let mut world = HittableList::empty();

    let mat_red = Arc::new(Lambertian::new(Arc::new(SolidColorTexture::new(
        Color::RED,
    ))));

    let mat_blue = Arc::new(Lambertian::new(Arc::new(SolidColorTexture::new(
        Color::BLUE,
    ))));

    world.add(Sphere::stationary(
        Point::new(0.0, 3.0, 0.0),
        2.0,
        mat_red.clone(),
    ));

    world.add(
        Sphere::stationary(Point::new(0.0, 3.0, 0.0), 2.0, mat_blue.clone())
            .translate(Vec3::new(2.0, 3.0, 0.0)),
    );

    let mat_ground = Arc::new(Lambertian::new_solid_color(Color::all(0.2)));
    world.add(Sphere::stationary(
        Point::new(0.0, -1000.0, 0.0),
        1000.0,
        mat_ground,
    ));

    world.add(HittableList::rectangular_box(
        &Vec3::new(-8.0, 5.0, 0.0),
        &Vec3::new(-2.0, 8.0, 3.0),
        mat_red.clone(),
    ));

    world.add(
        HittableList::rectangular_box(
            &Vec3::new(-8.0, 5.0, 0.0),
            &Vec3::new(-2.0, 8.0, 3.0),
            mat_blue.clone(),
        )
        .rotate_y(60.0),
    );

    let camera: Camera = Camera::new(CameraOption {
        bg_color: Color::new(0.7, 0.8, 1.0),
        vfov: 30.0,
        look_from: Point::new(22.0, 12.0, 26.0),
        look_at: Point::new(0.0, 3.0, 0.0),
        vup: Vec3::new(0.0, 1.0, 0.0),
        defocus_angle: 0.0,
        focus_distance: 10.0,
        quality,
    });

    (world, camera)
}
