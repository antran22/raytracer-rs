use crate::{object::HitRecord, ray::Ray, vec3::Color};

pub struct ScatterResult {
    pub attenuation: Color,
    pub ray: Ray,
}
pub trait Material {
    fn scatter(&self, r_in: &Ray, hit_record: &HitRecord) -> Option<ScatterResult>;
}
