use crate::{
    object::HitRecord,
    ray::Ray,
    vec3::{Color, Vec3},
};

use super::{Material, MaterialInteractResult};

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Self {
            albedo,
            fuzz: fuzz.clamp(0.0, 1.0),
        }
    }
}

impl Material for Metal {
    fn interact(&self, r_in: &Ray, hit_record: &HitRecord) -> MaterialInteractResult {
        let mut reflected_dir = r_in.dir.reflect(&hit_record.normal);
        if self.fuzz > 0.0 {
            reflected_dir = reflected_dir.to_unit() + self.fuzz * Vec3::rand_unit();
        }
        if reflected_dir.dot(&hit_record.normal) <= 0.0 {
            return MaterialInteractResult::None;
        }
        let scattered_ray = Ray {
            dir: reflected_dir,
            origin: hit_record.point,
            time: r_in.time,
        };
        MaterialInteractResult::Scatter {
            attenuation: self.albedo,
            ray: scattered_ray,
        }
    }
}
