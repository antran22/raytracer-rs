use std::{cmp::Ordering, sync::Arc};

use crate::{interval::Interval, ray::Ray};

use super::{aabb::Aabb, hittable_list::HittableVec, Hittable};

// Implementation of the Bounding Volume Hierarchy system
pub struct BVHTree {
    left: Arc<dyn Hittable + Send + Sync>,
    right: Arc<dyn Hittable + Send + Sync>,
    bbox: Aabb,
}

impl BVHTree {
    fn new(left: Arc<dyn Hittable + Send + Sync>, right: Arc<dyn Hittable + Send + Sync>) -> Self {
        Self {
            bbox: Aabb::join(&left.bounding_box(), &right.bounding_box()),
            left,
            right,
        }
    }

    fn leaf(hittable: Arc<dyn Hittable + Send + Sync>) -> Self {
        Self {
            bbox: hittable.bounding_box().clone(),
            left: hittable.clone(),
            right: hittable.clone(),
        }
    }

    pub fn from_list(objects: &HittableVec) -> Self {
        let length = objects.len();
        return match length {
            1 => Self::leaf(objects[0].clone()),
            2 => Self::new(objects[0].clone(), objects[1].clone()),
            _ => {
                let mut bbox = Aabb::EMPTY;
                for obj in objects {
                    bbox = Aabb::join(obj.bounding_box(), &bbox)
                }
                let axis = bbox.longest_axis() as usize;

                let mut sortable_objects = objects.clone();
                sortable_objects.sort_by(|a, b| {
                    let a_interval = a.bounding_box()[axis];
                    let b_interval = b.bounding_box()[axis];

                    if a_interval.min < b_interval.min {
                        return Ordering::Less;
                    }
                    if a_interval.min > b_interval.min {
                        return Ordering::Greater;
                    }
                    return Ordering::Equal;
                });
                let mid = length / 2;
                let left_tree = Arc::new(Self::from_list(&sortable_objects[0..mid].to_vec()));
                let right_tree = Arc::new(Self::from_list(&sortable_objects[mid..].to_vec()));
                Self {
                    left: left_tree,
                    right: right_tree,
                    bbox,
                }
            }
        };
    }
}

impl Hittable for BVHTree {
    fn hit(&self, ray: &Ray, interval: &Interval) -> Option<super::HitRecord> {
        if self.bbox.hit(ray).is_none() {
            return None;
        }

        let left_hit = self.left.hit(ray, interval);

        if let Some(left_hit_record) = left_hit {
            let right_interval = Interval::new(interval.min, left_hit_record.t);
            let right_hit = self.right.hit(ray, &right_interval);
            return right_hit.or(Some(left_hit_record));
        } else {
            return self.right.hit(ray, interval);
        }
    }

    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }
}
