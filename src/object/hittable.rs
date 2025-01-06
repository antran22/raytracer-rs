use crate::ray::Ray;
use crate::vec3::{Point, Vec3};

pub struct HitRecord {
    pub point: Point,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(ray: &Ray, t: f64, outward_normal: Vec3) -> Self {
        let is_front_face = ray.dir.dot(&outward_normal) < 0.0;
        let normal = if is_front_face {
            outward_normal
        } else {
            -outward_normal
        };
        
        let point = ray.at(t);

        Self {
            front_face: is_front_face,
            normal: normal,
            point: point,
            t: t,
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord>;
}
