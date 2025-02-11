use crate::{object::Hittable, vec3::Vec3};

use super::{Translate, YRotate};

pub trait Transformable<T: Hittable + Send + Sync> {
    fn translate(self, offset: Vec3) -> Translate<T>;
    fn rotate_y(self, angle: f64) -> YRotate<T>;
}

impl<T> Transformable<T> for T
where
    T: Hittable + Send + Sync,
{
    fn translate(self, offset: Vec3) -> Translate<T> {
        Translate::new(offset, self)
    }

    fn rotate_y(self, angle: f64) -> YRotate<T> {
        YRotate::new(angle, self)
    }
}
