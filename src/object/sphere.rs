use std::{f64::consts::PI, sync::Arc};

use crate::{
    interval::Interval,
    material::Material,
    ray::Ray,
    vec3::{Point, Vec3},
};

use super::{
    hittable::{HitRecord, Hittable},
    Aabb,
};

pub struct Sphere {
    center: Ray,
    radius: f64,
    material: Arc<dyn Material + Send + Sync>,
    bbox: Aabb,
}

impl Sphere {
    pub fn stationary(
        center: Point,
        radius: f64,
        material: Arc<dyn Material + Send + Sync>,
    ) -> Self {
        let rvec = Vec3::all(radius);
        Self {
            center: Ray {
                origin: center,
                dir: Vec3::ZERO,
                time: 0.0,
            },
            radius,
            material,
            bbox: Aabb::between_points(&(center - rvec), &(center + rvec)),
        }
    }

    pub fn moving(
        center1: Point,
        center2: Point,
        radius: f64,
        material: Arc<dyn Material + Send + Sync>,
    ) -> Self {
        let rvec = Vec3::all(radius);

        let box1 = Aabb::between_points(&(center1 - rvec), &(center1 + rvec));
        let box2 = Aabb::between_points(&(center2 - rvec), &(center2 + rvec));

        Self {
            center: Ray {
                origin: center1,
                dir: center2 - center1,
                time: 0.0,
            },
            radius,
            material,
            bbox: Aabb::join(&box1, &box2),
        }
    }

    pub fn get_sphere_uv(point: &Point) -> (f64, f64) {
        let theta = f64::acos(-point.y);
        let phi = f64::atan2(-point.z, point.x) + PI;

        let u = phi / (2.0 * PI);
        let v = theta / PI;

        (u, v)
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        let current_center = self.center.at(ray.time);
        let oc = current_center - ray.origin;
        let a = ray.dir.length_squared();
        let h = ray.dir.dot(&oc);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = h * h - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();
        let mut t = (h - sqrtd) / a;
        if !ray_t.surrounds(t) {
            t = (h + sqrtd) / a;

            if !ray_t.surrounds(t) {
                return None;
            }
        }

        let point = ray.at(t);
        let outward_normal = (point - current_center) / self.radius;

        let (u, v) = Self::get_sphere_uv(&outward_normal);
        Some(HitRecord::new(
            ray,
            t,
            outward_normal,
            u,
            v,
            self.material.clone(),
        ))
    }

    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }
}
