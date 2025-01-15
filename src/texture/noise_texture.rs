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

const MID_GREY: Color = Color::all(0.5);

impl<T: NoiseFunction> Texture for NoiseTexture<T> {
    fn value(&self, _u: f64, _v: f64, p: &Point) -> Color {
        MID_GREY * (1.0 + f64::sin(self.scale * p.z + 10.0 * self.noise_func.turbulence(p)))
    }
}
