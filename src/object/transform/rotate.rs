use std::sync::Arc;

use crate::{
    interval::Interval,
    object::{Aabb, HitRecord, Hittable},
    ray::Ray,
    vec3::{Point, Vec3},
};

#[derive(Debug)]
pub struct YRotate<H: Hittable + Send + Sync> {
    sin_theta: f64,
    cos_theta: f64,
    object: H,
    bbox: Aabb,
}

impl<H: Hittable + Send + Sync> YRotate<H> {
    pub fn new(angle: f64, object: H) -> Self {
        let theta = angle.to_radians();
        let sin_theta = theta.sin();
        let cos_theta = theta.cos();
        let orig_bbox = object.bounding_box();

        let mut p_min = Point::INFINITY;
        let mut p_max = Point::NEG_INFINITY;

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = orig_bbox.x[i];
                    let y = orig_bbox.y[j];
                    let z = orig_bbox.z[k];

                    let newx = cos_theta * x + sin_theta * z;
                    let newz = -sin_theta * x + cos_theta * z;

                    let tester = Vec3::new(newx, y, newz);

                    for c in 0..3 {
                        p_min[c] = p_min[c].min(tester[c]);
                        p_max[c] = p_max[c].max(tester[c]);
                    }
                }
            }
        }

        let new_bbox = Aabb::between_points(&p_min, &p_max);

        Self {
            sin_theta,
            cos_theta,
            bbox: new_bbox,
            object,
        }
    }
}

impl<H: Hittable + Send + Sync> Hittable for YRotate<H> {
    fn hit(&self, ray: &Ray, interval: &Interval) -> Option<HitRecord> {
        let Ray { origin, dir, time } = ray;
        let Self {
            sin_theta,
            cos_theta,
            ..
        } = self;

        let rotated_origin = Point::new(
            cos_theta * origin.x - sin_theta * origin.z,
            origin.y,
            sin_theta * origin.x + cos_theta * origin.z,
        );

        let rotated_dir = Vec3::new(
            cos_theta * dir.x - sin_theta * dir.z,
            dir.y,
            sin_theta * dir.x + cos_theta * dir.z,
        );

        let rotated_ray = Ray {
            origin: rotated_origin,
            dir: rotated_dir,
            time: *time,
        };

        if let Some(mut hit) = self.object.hit(&rotated_ray, interval) {
            hit.point = Point::new(
                cos_theta * hit.point.x + sin_theta * hit.point.z,
                hit.point.y,
                -sin_theta * hit.point.x + cos_theta * hit.point.z,
            );

            hit.normal = Vec3::new(
                cos_theta * hit.normal.x + sin_theta * hit.normal.z,
                hit.normal.y,
                -sin_theta * hit.normal.x + cos_theta * hit.normal.z,
            );
            return Some(hit);
        }

        None
    }

    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }
}
