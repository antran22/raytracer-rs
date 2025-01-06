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

    pub fn sphere_normal(&self, s: &Sphere) -> Option<Vec3> {
        let oc = s.center - self.origin;
        let a = self.dir.length_squared();
        let h = self.dir.dot(&oc);
        let c = oc.length_squared() - s.radius * s.radius;

        let discriminant = h * h - a * c;

        if discriminant < 0.0 {
            None
        } else {
            let t = (h - discriminant.sqrt()) / a;
            let n = (self.at(t) - s.center).unit();
            Some(n)
        }
    }
}
