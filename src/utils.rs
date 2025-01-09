use std::ops::Range;

use rand::Rng;

use crate::vec3::Vec3;

pub fn rand_double() -> f64 {
    rand::thread_rng().gen_range(0.0..1.0)
}

pub fn rand_range_double(r: Range<f64>) -> f64 {
    rand::thread_rng().gen_range(r.clone())
}

pub fn rand_vector_on_hemisphere(normal: &Vec3) -> Vec3 {
    let u = Vec3::rand_unit();
    if u.dot(normal) > 0.0 {
        return u;
    }
    return -u;
}

pub fn rand_vector_in_unit_disk() -> Vec3 {
    loop {
        let v = Vec3::new(
            rand_range_double(-1.0..1.0),
            rand_range_double(-1.0..1.0),
            0.0,
        );
        if v.length_squared() < 1.0 {
            return v;
        }
    }
}

pub fn linear_to_gamma(linear: f64) -> f64 {
    if linear > 0.0 {
        return linear.sqrt();
    }
    return 0.0;
}
