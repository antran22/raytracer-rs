use crate::vec3::{Point, Vec3};

pub struct Ray {
    origin: Point,
    dir: Vec3
}

impl Ray {
    pub fn at(&self, t: f64) -> Point {
        return self.origin + self.dir * t;
    }
}