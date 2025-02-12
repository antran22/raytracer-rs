use core::f64;
use std::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Range, Sub, SubAssign,
};

use crate::utils::{rand_double, rand_range};

#[derive(Copy, Clone, Default, Debug)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub const fn zero() -> Vec3 {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub const fn new(a: f64, b: f64, c: f64) -> Vec3 {
        Self { x: a, y: b, z: c }
    }

    pub const fn all(a: f64) -> Vec3 {
        Self { x: a, y: a, z: a }
    }

    pub fn rand() -> Self {
        Vec3 {
            x: rand_double(),
            y: rand_double(),
            z: rand_double(),
        }
    }

    pub fn rand_range(r: Range<f64>) -> Self {
        Vec3 {
            x: rand_range(r.clone()),
            y: rand_range(r.clone()),
            z: rand_range(r.clone()),
        }
    }

    pub fn rand_unit() -> Self {
        loop {
            let v = Vec3::rand_range(-1.0..1.0);
            let lensq = v.length_squared();
            if 1e-160 < lensq && lensq < 1.0 {
                return v / lensq.sqrt();
            }
        }
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn dot(&self, b: &Vec3) -> f64 {
        self.x * b.x + self.y * b.y + self.z * b.z
    }

    pub fn cross(&self, b: &Vec3) -> Vec3 {
        Vec3::new(
            self.y * b.z - self.z * b.y,
            self.z * b.x - self.x * b.z,
            self.x * b.y - self.y * b.x,
        )
    }

    pub fn is_near_zero(&self) -> bool {
        const EPS: f64 = 1e-8;
        self.x.abs() < EPS && self.y.abs() < EPS && self.y.abs() < EPS
    }

    pub fn to_unit(&self) -> Vec3 {
        *self / self.length()
    }

    pub fn reflect(&self, normal: &Vec3) -> Vec3 {
        *self - (2.0 * self.dot(normal) * normal)
    }

    pub fn refract(&self, normal: &Vec3, refract_ratio: f64) -> Vec3 {
        let cos_theta = f64::min((-self).dot(normal), 1.0);
        let r_out_perpendicular = refract_ratio * (*self + cos_theta * normal);
        let r_out_parallel = -(1.0 - r_out_perpendicular.length_squared()).abs().sqrt() * normal;
        r_out_parallel + r_out_perpendicular
    }

    pub const ZERO: Vec3 = Vec3::all(0.0);
    pub const INFINITY: Vec3 = Vec3::all(f64::INFINITY);
    pub const NEG_INFINITY: Vec3 = Vec3::all(f64::NEG_INFINITY);
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Neg for &Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Vec3) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Vec3) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, vec: Vec3) -> Vec3 {
        Vec3::new(self.x * vec.x, self.y * vec.y, self.z * vec.z)
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, scalar: f64) -> Vec3 {
        Vec3::new(self.x * scalar, self.y * scalar, self.z * scalar)
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, vec: Vec3) -> Vec3 {
        vec * self
    }
}

impl Mul<&Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, vec: &Vec3) -> Vec3 {
        *vec * self
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, scalar: f64) {
        self.x *= scalar;
        self.y *= scalar;
        self.z *= scalar;
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, scalar: f64) -> Vec3 {
        self * (1.0 / scalar)
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, scalar: f64) {
        *self *= 1.0 / scalar
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Index out of bounds for Vec3"),
        }
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Index out of bounds for Vec3"),
        }
    }
}
