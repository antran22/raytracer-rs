use crate::{
    object::HitRecord,
    ray::Ray,
    vec3::{Color, Vec3},
};

use super::{Material, ScatterResult};

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub const fn new(albedo: Color) -> Self {
        Self { albedo: albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, hit_record: &HitRecord) -> Option<super::ScatterResult> {
        let _ = r_in;
        let mut scatter_dir = hit_record.normal + Vec3::rand_unit();
        if scatter_dir.is_near_zero() {
            scatter_dir = hit_record.normal
        }

        let scattered = Ray {
            origin: hit_record.point,
            dir: scatter_dir,
        };
        Some(ScatterResult {
            attenuation: self.albedo,
            ray: scattered,
        })
    }
}
