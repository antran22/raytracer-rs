use std::sync::Arc;

use image::{ImageReader, RgbImage};

use crate::{
    interval::Interval,
    vec3::{Color, Point},
};

use super::Texture;

pub struct ImageTexture {
    img: Arc<RgbImage>,
}

impl ImageTexture {
    pub fn new(img_path: &str) -> Self {
        let res = ImageReader::open(img_path)
            .expect("unable to read image at path")
            .decode()
            .expect("unable to parse image");
        let res = res.to_rgb8();

        Self { img: Arc::new(res) }
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, _p: &Point) -> Color {
        let img = self.img.clone();
        if img.height() == 0 {
            return Color::new(0.0, 1.0, 1.0);
        }

        let u = Interval::ONE.clamp(u);
        let v = 1.0 - Interval::ONE.clamp(v);

        let i = (u * (img.width() as f64)) as u32;
        let j = (v * (img.height() as f64)) as u32;

        let pixel = img.get_pixel(i, j);

        let color_scale = 1.0 / 255.0;
        Color::new(
            pixel[0] as f64 * color_scale,
            pixel[1] as f64 * color_scale,
            pixel[2] as f64 * color_scale,
        )
    }
}
