use crate::{object::HitRecord, ray::Ray, vec3::Color};

use super::{Material, ScatterResult};

pub struct Metal {
    albedo: Color,
}

impl Metal {
    pub const fn new(albedo: &Color) -> Self {
        Self { albedo: *albedo }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, hit_record: &HitRecord) -> Option<ScatterResult> {
        let reflected_dir = r_in.dir.reflect(&hit_record.normal);
        let scattered_ray = Ray {
            dir: reflected_dir,
            origin: hit_record.point,
        };
        Some(ScatterResult {
            attenuation: self.albedo,
            ray: scattered_ray,
        })
    }
}
