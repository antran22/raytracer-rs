use std::sync::Arc;

use crate::{
    interval::Interval,
    material::Material,
    ray::Ray,
    vec3::{Point, Vec3},
};

use super::{Aabb, HitRecord, Hittable};

pub struct Quad {
    q: Point,
    u: Vec3,
    v: Vec3,
    normal: Vec3,
    d: f64,
    w: Vec3,
    bbox: Aabb,
    material: Arc<dyn Material + Send + Sync + 'static>,
}

impl Quad {
    pub fn new(q: Point, u: Vec3, v: Vec3, material: Arc<dyn Material + Send + Sync>) -> Self {
        let n = u.cross(&v);
        let normal = n.to_unit();
        let d = normal.dot(&q);
        let w = n / Vec3::dot(&n, &n);

        let bbox_diag1 = Aabb::between_points(&q, &(q + u + v));
        let bbox_diag2 = Aabb::between_points(&(q + u), &(q + v));

        Self {
            q,
            u,
            v,
            material,
            normal,
            d,
            w,
            bbox: Aabb::join(&bbox_diag1, &bbox_diag2),
        }
    }

    fn is_interior(a: f64, b: f64) -> bool {
        let unit = Interval::UNIT;
        unit.contains(a) && unit.contains(b)
    }
}

impl Hittable for Quad {
    fn hit(&self, ray: &Ray, hit_interval: &Interval) -> Option<HitRecord> {
        let denom = self.normal.dot(&ray.dir);
        if denom.abs() < 1e-8 {
            return None; // ray is parallel to quad
        }
        let t = (self.d - Vec3::dot(&self.normal, &ray.origin)) / denom;
        if !hit_interval.contains(t) {
            return None;
        }
        let intersection = ray.at(t);
        let planar_hitpt_vector = intersection - self.q;
        let alpha = Vec3::dot(&self.w, &Vec3::cross(&planar_hitpt_vector, &self.v));
        let beta = Vec3::dot(&self.w, &Vec3::cross(&self.u, &planar_hitpt_vector));
        if !Self::is_interior(alpha, beta) {
            return None;
        }

        Some(HitRecord::new(
            ray,
            t,
            self.normal,
            alpha,
            beta,
            self.material.clone(),
        ))
    }

    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }
}
