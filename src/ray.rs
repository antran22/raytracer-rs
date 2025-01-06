use crate::{
    sphere::Sphere,
    vec3::{Point, Vec3},
};

pub struct Ray {
    pub origin: Point,
    pub dir: Vec3,
}

impl Ray {
    pub fn at(&self, t: f64) -> Point {
        return self.origin + self.dir * t;
    }

    pub fn hit_sphere(&self, s: &Sphere) -> bool {
        let oc = s.center - self.origin;
        let a = self.dir.dot(&self.dir);
        let b = -2.0 * self.dir.dot(&oc);
        let c = oc.dot(&oc) - s.radius * s.radius;

        let discriminant = b * b - 4.0 * a * c;
        discriminant >= 0.0
    }
}
