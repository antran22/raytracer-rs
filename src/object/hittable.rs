use std::sync::Arc;

use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{Point, Vec3};

use super::aabb::AABB;

#[derive(Clone)]
pub struct HitRecord {
    pub point: Point,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub u: f64,
    pub v: f64,
    pub material: Arc<dyn Material>,
}

impl HitRecord {
    pub fn new(
        ray: &Ray,
        t: f64,
        outward_normal: Vec3,
        u: f64,
        v: f64,
        material: Arc<dyn Material>,
    ) -> Self {
        let front_face = ray.dir.dot(&outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };

        let point = ray.at(t);

        Self {
            u,
            v,
            front_face,
            normal,
            point,
            t,
            material,
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, interval: &Interval) -> Option<HitRecord>;

    fn bounding_box(&self) -> &AABB;
}
