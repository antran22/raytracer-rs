mod material;

mod lambertian;
mod metal;
mod dielectric;

pub use material::{Material, ScatterResult};
pub use lambertian::Lambertian;
pub use metal::Metal;
pub use dielectric::Dielectric;
