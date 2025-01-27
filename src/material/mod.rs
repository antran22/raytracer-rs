mod material;

mod dielectric;
mod lambertian;
mod light;
mod metal;

pub use dielectric::Dielectric;
pub use lambertian::Lambertian;
pub use light::DiffuseLight;
pub use material::{Material, MaterialInteractResult};
pub use metal::Metal;
