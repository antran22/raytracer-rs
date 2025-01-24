use image::Rgb;

use crate::interval::Interval;
use crate::utils::linear_to_gamma;

pub use super::vec3::Vec3 as Color;
const COLOR_INTENSITY: Interval = Interval::new(0.0, 0.9999);
impl Color {
    pub fn to_rgb(&self) -> Rgb<u8> {
        let r = linear_to_gamma(self.x);
        let g = linear_to_gamma(self.y);
        let b = linear_to_gamma(self.z);

        let rbyte = (COLOR_INTENSITY.clamp(r) * 256.0) as u8;
        let gbyte = (COLOR_INTENSITY.clamp(g) * 256.0) as u8;
        let bbyte = (COLOR_INTENSITY.clamp(b) * 256.0) as u8;

        Rgb([rbyte, gbyte, bbyte])
    }

    pub const BLACK: Self = Self::zero();
    pub const WHITE: Self = Self::new(1.0, 1.0, 1.0);
    pub const RED: Self = Self::new(1.0, 0.0, 0.0);
    pub const BLUE: Self = Self::new(0.2, 0.2, 1.0);
    pub const GREEN: Self = Self::new(0.2, 1.0, 0.2);
    pub const TEAL: Self = Self::new(0.2, 0.8, 0.8);
    pub const ORANGE: Self = Self::new(1.0, 0.5, 0.0);
}
