use std::sync::{Arc, Mutex};

use crate::{interval::Interval, ray::Ray};

use super::{HitRecord, Hittable};

pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable + Send + Sync>>,
}

impl HittableList {
    pub fn empty() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn add<T: Hittable + Send + Sync + 'static>(&mut self, obj: T) {
        self.objects.push(Arc::new(obj));
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        let mut closest_hit_record: Option<HitRecord> = None;
        let mut closest_so_far = ray_t.max;

        for obj in &self.objects {
            if let Some(hit) = obj.hit(ray, &Interval::new(ray_t.min, closest_so_far)) {
                closest_so_far = hit.t;
                closest_hit_record = Some(hit);
            };
        }
        closest_hit_record
    }
}
