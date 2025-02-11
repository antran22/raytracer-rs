use std::sync::Arc;

use crate::{
    interval::Interval,
    object::{Aabb, HitRecord, Hittable},
    ray::Ray,
    vec3::Vec3,
};

pub struct Translate<H: Hittable + Send + Sync> {
    offset: Vec3,
    object: H,
    bbox: Aabb,
}

impl<H: Hittable + Send + Sync> Translate<H> {
    pub fn new(offset: Vec3, object: H) -> Self {
        Self {
            offset,
            bbox: object.bounding_box() + &offset,
            object,
        }
    }
}

impl<H: Hittable + Send + Sync> Hittable for Translate<H> {
    fn hit(&self, ray: &Ray, interval: &Interval) -> Option<HitRecord> {
        let offset_ray = Ray {
            origin: ray.origin - self.offset,
            dir: ray.dir,
            time: ray.time,
        };

        if let Some(mut hit) = self.object.hit(&offset_ray, interval) {
            hit.point += self.offset;
            return Some(hit);
        }

        None
    }

    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }
}
