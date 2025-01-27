use std::sync::Arc;

use crate::{
    object::HitRecord,
    ray::Ray,
    texture::{SolidColorTexture, Texture},
    vec3::Color,
};

use super::{Material, MaterialInteractResult};

pub struct DiffuseLight {
    texture: Arc<dyn Texture + Send + Sync>,
}

impl DiffuseLight {
    pub fn new(texture: Arc<dyn Texture + Send + Sync>) -> Self {
        Self { texture }
    }

    pub fn new_from_color(color: Color) -> Self {
        Self::new(Arc::new(SolidColorTexture::new(color)))
    }
}

impl Material for DiffuseLight {
    fn interact(&self, _r_in: &Ray, hit_record: &HitRecord) -> MaterialInteractResult {
        let color = self
            .texture
            .value(hit_record.u, hit_record.v, &hit_record.point);
        MaterialInteractResult::Emitted { color }
    }
}
