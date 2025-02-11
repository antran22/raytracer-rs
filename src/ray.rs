use crate::vec3::{Point, Vec3};

pub struct Ray {
    pub origin: Point,
    pub dir: Vec3,
    pub time: f64,
}

impl Ray {
    pub fn at(&self, t: f64) -> Point {
        self.origin + self.dir * t
    }
}
