use rand::{
    distributions::uniform::{SampleRange, SampleUniform},
    Rng,
};

use crate::vec3::Vec3;

pub fn rand_double() -> f64 {
    rand::thread_rng().gen_range(0.0..1.0)
}

pub fn rand_range<T, R>(range: R) -> T
where
    T: SampleUniform,
    R: SampleRange<T>,
{
    rand::thread_rng().gen_range(range)
}

pub fn rand_vector_in_unit_disk() -> Vec3 {
    loop {
        let v = Vec3::new(rand_range(-1.0..1.0), rand_range(-1.0..1.0), 0.0);
        if v.length_squared() < 1.0 {
            return v;
        }
    }
}

pub fn linear_to_gamma(linear: f64) -> f64 {
    if linear > 0.0 {
        return linear.sqrt();
    }
    0.0
}
