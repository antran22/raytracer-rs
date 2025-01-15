use crate::{
    utils::{NoiseFunction, Perlin},
    vec3::{Color, Point},
};

use super::Texture;

pub struct NoiseTexture<T: NoiseFunction> {
    noise_func: T,
    scale: f64,
}

impl NoiseTexture<Perlin> {
    pub fn new_perlin(scale: f64) -> Self {
        Self {
            noise_func: Perlin::new(),
            scale,
        }
    }
}

impl<T: NoiseFunction> Texture for NoiseTexture<T> {
    fn value(&self, _u: f64, _v: f64, p: &Point) -> Color {
        let p = *p * self.scale;
        Color::WHITE * self.noise_func.noise(&p)
    }
}
