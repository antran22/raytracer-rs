mod material;

mod lambertian;
mod metal;

pub use material::{Material, ScatterResult};
pub use lambertian::Lambertian;
pub use metal::Metal;
