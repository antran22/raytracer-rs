use std::sync::Arc;

use crate::vec3::{Color, Point};

use super::{solid_color::SolidColor, Texture};

pub struct CheckeredTexture {
    inv_scale: f64,
    even: Arc<dyn Texture + Send + Sync>,
    odd: Arc<dyn Texture + Send + Sync>,
}

impl CheckeredTexture {
    pub fn new(
        scale: f64,
        even: Arc<dyn Texture + Send + Sync>,
        odd: Arc<dyn Texture + Send + Sync>,
    ) -> Self {
        Self {
            inv_scale: 1.0 / scale,
            even,
            odd,
        }
    }

    pub fn new_from_colors(scale: f64, even: Color, odd: Color) -> Self {
        Self::new(
            scale,
            Arc::new(SolidColor::new(even)),
            Arc::new(SolidColor::new(odd)),
        )
    }
}

impl Texture for CheckeredTexture {
    fn value(&self, u: f64, v: f64, p: Point) -> Color {
        let u_int = (u * self.inv_scale).floor() as i32;
        let v_int = (v * self.inv_scale).floor() as i32;

        if (u_int + v_int) % 2 == 0 {
            self.even.value(u, v, p)
        } else {
            self.odd.value(u, v, p)
        }
    }
}
