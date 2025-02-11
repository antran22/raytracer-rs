use std::sync::Arc;

use super::{Aabb, HitRecord, Hittable, Quad};
use crate::{
    interval::Interval,
    material::Material,
    ray::Ray,
    vec3::{Point, Vec3},
};
pub type HittableVec = Vec<Arc<dyn Hittable + Send + Sync>>;

pub struct HittableList {
    objects: HittableVec,
    bbox: Aabb,
}

impl HittableList {
    pub fn empty() -> Self {
        Self {
            objects: Vec::new(),
            bbox: Aabb::ZERO,
        }
    }

    pub fn objects(&self) -> &HittableVec {
        &self.objects
    }

    pub fn add<T: Hittable + Send + Sync + 'static>(&mut self, obj: T) {
        self.bbox = Aabb::join(&(obj.bounding_box()), &self.bbox);
        self.objects.push(Arc::new(obj));
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn rectangular_box(a: &Point, b: &Point, mat: Arc<dyn Material + Send + Sync>) -> Self {
        let mut sides = Self::empty();
        let min = Point::new(f64::min(a.x, b.x), f64::min(a.y, b.y), f64::min(a.z, b.z));
        let max = Point::new(f64::max(a.x, b.x), f64::max(a.y, b.y), f64::max(a.z, b.z));

        let dx = Vec3::new(max.x - min.x, 0.0, 0.0);
        let dy = Vec3::new(0.0, max.y - min.y, 0.0);
        let dz = Vec3::new(0.0, 0.0, max.z - min.z);

        sides.add(Quad::new(
            Point::new(min.x, min.y, max.z),
            dx,
            dy,
            mat.clone(),
        )); // front
        sides.add(Quad::new(
            Point::new(max.x, min.y, max.z),
            -dz,
            dy,
            mat.clone(),
        )); // right
        sides.add(Quad::new(
            Point::new(max.x, min.y, min.z),
            -dx,
            dy,
            mat.clone(),
        )); // back
        sides.add(Quad::new(
            Point::new(min.x, min.y, min.z),
            dz,
            dy,
            mat.clone(),
        )); // left
        sides.add(Quad::new(
            Point::new(min.x, max.y, max.z),
            dx,
            -dz,
            mat.clone(),
        )); // top
        sides.add(Quad::new(
            Point::new(min.x, min.y, min.z),
            dx,
            dz,
            mat.clone(),
        )); // bottom

        sides
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

    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }
}
