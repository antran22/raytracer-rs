mod material;

mod dielectric;
mod lambertian;
mod metal;

pub use dielectric::Dielectric;
pub use lambertian::Lambertian;
pub use material::{Material, MaterialInteractResult};
pub use metal::Metal;
