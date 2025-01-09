use crate::{
    object::HitRecord,
    ray::Ray,
    utils::rand_double,
    vec3::{Color, Vec3},
};

use super::{Material, ScatterResult};

pub struct Dielectric {
    refraction_index: f64,
}
impl Dielectric {
    pub fn new(refraction_index: f64) -> Self {
        Self { refraction_index }
    }

    fn reflectance(cosine: f64, refraction_ratio: f64) -> f64 {
        let mut r0 = (1.0 - refraction_ratio) / (1.0 + refraction_ratio);
        r0 *= r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, hit_record: &HitRecord) -> Option<ScatterResult> {
        let HitRecord { point, normal, .. } = hit_record;
        let ri = if hit_record.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_dir = r_in.dir.to_unit();
        let cos_theta = f64::min(Vec3::dot(&-unit_dir, normal), 1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let dir = if ri * sin_theta > 1.0 || Dielectric::reflectance(cos_theta, ri) > rand_double()
        {
            unit_dir.reflect(normal)
        } else {
            unit_dir.refract(normal, ri)
        };

        Some(ScatterResult {
            attenuation: Color::WHITE,
            ray: Ray {
                origin: *point,
                time: r_in.time,
                dir,
            },
        })
    }
}
