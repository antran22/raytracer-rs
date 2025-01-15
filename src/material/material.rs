use crate::{object::HitRecord, ray::Ray, vec3::Color};

pub enum MaterialInteractResult {
    Scatter { attenuation: Color, ray: Ray },
    Emitted { color: Color },
    None,
}

pub trait Material {
    fn interact(&self, r_in: &Ray, hit_record: &HitRecord) -> MaterialInteractResult;
}
