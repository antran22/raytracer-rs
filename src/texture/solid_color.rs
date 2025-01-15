use crate::vec3::{Color, Point};

use super::Texture;

pub struct SolidColor {
    albedo: Color,
}

impl SolidColor {
    pub fn new(color: Color) -> Self {
        Self { albedo: color }
    }
}

impl Texture for SolidColor {
    fn value(&self, _u: f64, _v: f64, _p: &Point) -> Color {
        self.albedo
    }
}
