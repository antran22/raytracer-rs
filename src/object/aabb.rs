// Define Axis-Aligned Bounding Box struct
use crate::{
    interval::Interval,
    ray::Ray,
    vec3::{Point, Vec3},
};
use std::ops::{Add, Index};

#[derive(Clone, Copy, Debug)]
pub struct Aabb {
    pub x: Interval,
    pub y: Interval,
    pub z: Interval,
}

impl Aabb {
    pub const ZERO: Aabb = Aabb::new_static(Interval::ZERO, Interval::ZERO, Interval::ZERO);
    pub const EMPTY: Aabb = Aabb::new_static(Interval::EMPTY, Interval::EMPTY, Interval::EMPTY);
    pub const UNIVERSE: Aabb =
        Aabb::new_static(Interval::UNIVERSE, Interval::UNIVERSE, Interval::UNIVERSE);

    pub const fn new_static(x: Interval, y: Interval, z: Interval) -> Self {
        Self { x, y, z }
    }

    pub fn new(x: Interval, y: Interval, z: Interval) -> Self {
        let [x, y, z] = [x, y, z].map(|i| i.pad(0.001));
        Self { x, y, z }
    }

    pub fn join(box1: &Self, box2: &Self) -> Self {
        Self::new(
            Interval::join(&box1.x, &box2.x),
            Interval::join(&box1.y, &box2.y),
            Interval::join(&box1.z, &box2.z),
        )
    }

    pub fn between_points(a: &Point, b: &Point) -> Self {
        Self::new(
            Interval::new_checked(a.x, b.x),
            Interval::new_checked(a.y, b.y),
            Interval::new_checked(a.z, b.z),
        )
    }

    pub fn longest_axis(&self) -> i32 {
        let x_size = self.x.size();
        let y_size = self.y.size();
        let z_size = self.z.size();

        if x_size > y_size {
            if x_size > z_size {
                0
            } else {
                2
            }
        } else if y_size > z_size {
            1
        } else {
            2
        }
    }

    pub fn hit(&self, r: &Ray) -> Option<Interval> {
        let Ray { dir, origin, .. } = r;
        let (mut ray_min, mut ray_max) = (f64::NEG_INFINITY, f64::INFINITY);
        for axis_index in 0..3 {
            let ax = self[axis_index];
            let axis_dir_determinant = dir[axis_index];
            let inverse_det = 1.0 / axis_dir_determinant;
            // instead of calculating t0 and t1 by division, we calculate the inverse then multiply. This will be faster to compute.
            let t0 = (ax.min - origin[axis_index]) * inverse_det;
            let t1 = (ax.max - origin[axis_index]) * inverse_det;

            if t0 < t1 {
                ray_min = f64::max(ray_min, t0);
                ray_max = f64::min(ray_max, t1);
            } else {
                ray_min = f64::max(ray_min, t1);
                ray_max = f64::min(ray_max, t0);
            }

            if ray_max <= ray_min {
                return None;
            }
        }
        Some(Interval::new(ray_min, ray_max))
    }
}

impl Index<usize> for Aabb {
    type Output = Interval;

    fn index(&self, i: usize) -> &Self::Output {
        match i {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Index out of bounds for AABB"),
        }
    }
}

impl Add<&Vec3> for &Aabb {
    type Output = Aabb;

    fn add(self, rhs: &Vec3) -> Self::Output {
        Aabb::new(&self.x + rhs.x, &self.y + rhs.y, &self.z + rhs.z)
    }
}

impl Add<&Aabb> for &Vec3 {
    type Output = Aabb;

    fn add(self, rhs: &Aabb) -> Self::Output {
        rhs + self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{interval::Interval, vec3::Vec3};

    #[test]
    fn test_aabb_hit() {
        // Define a simple AABB
        let aabb = Aabb::new(
            Interval::new(1.0, 3.0),
            Interval::new(1.0, 3.0),
            Interval::new(1.0, 3.0),
        );

        // Define a Ray that should intersect the AABB
        let ray = Ray {
            origin: Point::new(0.0, 0.0, 0.0),
            dir: Vec3::new(1.0, 1.0, 1.0),
            time: 0.0,
        };

        // The ray should hit the AABB
        match aabb.hit(&ray) {
            Some(interval) => {
                assert!(interval.min <= interval.max);
                println!(
                    "Ray hits the AABB with intersection interval: {:?}",
                    interval
                );
            }
            None => panic!("Expected the ray to hit the AABB!"),
        }
    }
}
