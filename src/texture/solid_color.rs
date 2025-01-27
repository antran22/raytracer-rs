use crate::vec3::{Color, Point};

use super::Texture;

pub struct SolidColorTexture {
    albedo: Color,
}

impl SolidColorTexture {
    pub fn new(color: Color) -> Self {
        Self { albedo: color }
    }
}

impl Texture for SolidColorTexture {
    fn value(&self, _u: f64, _v: f64, _p: &Point) -> Color {
        self.albedo
    }
}
