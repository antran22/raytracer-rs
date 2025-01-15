use std::sync::Arc;

use crate::{
    object::HitRecord,
    ray::Ray,
    texture::{SolidColor, Texture},
    vec3::{Color, Vec3},
};

use super::{Material, ScatterResult};

#[derive(Clone)]
pub struct Lambertian {
    texture: Arc<dyn Texture + Sync + Send>,
}

impl Lambertian {
    pub fn new_solid_color(albedo: Color) -> Self {
        Self {
            texture: Arc::new(SolidColor::new(albedo)),
        }
    }
    pub fn new(texture: Arc<dyn Texture + Sync + Send>) -> Self {
        Self { texture }
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
            time: r_in.time,
            dir: scatter_dir,
        };
        let attenuation = self
            .texture
            .value(hit_record.u, hit_record.v, &hit_record.point);
        Some(ScatterResult {
            attenuation,
            ray: scattered,
        })
    }
}
