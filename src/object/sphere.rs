use crate::{ray::Ray, vec3::Point};

use super::hittable::{HitRecord, Hittable};

pub struct Sphere {
    pub center: Point,
    pub radius: f64,
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord> {
        let oc = self.center - ray.origin;
        let a = ray.dir.length_squared();
        let h = ray.dir.dot(&oc);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = h * h - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();
        let mut t = (h - sqrtd) / a;
        if t <= ray_tmin || ray_tmax <= t {
            t = (h + sqrtd) / a;

            if t <= ray_tmin || ray_tmax <= t {
                return None;
            }
        }

        let point = ray.at(t);
        let outward_normal = (point - self.center) / self.radius;
        Some(HitRecord::new(ray, t, outward_normal))
    }
}
