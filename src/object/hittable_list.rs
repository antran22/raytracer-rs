use std::rc::Rc;

use crate::{interval::Interval, ray::Ray};

use super::{HitRecord, Hittable};

pub struct HittableList {
    pub objects: Vec<Rc<dyn Hittable>>,
}

impl HittableList {
    pub fn empty() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn new(objects: Vec<Rc<dyn Hittable>>) -> Self {
        Self { objects: objects }
    }
    pub fn add(&mut self, obj: Rc<dyn Hittable>) {
        self.objects.push(obj);
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
