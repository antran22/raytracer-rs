use std::io::Write;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Range, Sub, SubAssign};

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

    pub const fn val(a: f64, b: f64, c: f64) -> Vec3 {
        Self { x: a, y: b, z: c }
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
            x: rand_range_double(r.clone()),
            y: rand_range_double(r.clone()),
            z: rand_range_double(r.clone()),
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
        Vec3::val(
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
        return r_out_parallel + r_out_perpendicular;
    }
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
        Vec3::val(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3::val(self.x - other.x, self.y - other.y, self.z - other.z)
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
        Vec3::val(self.x * vec.x, self.y * vec.y, self.z * vec.z)
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, scalar: f64) -> Vec3 {
        Vec3::val(self.x * scalar, self.y * scalar, self.z * scalar)
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

pub use Vec3 as Point;
impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}
impl Point {
    pub fn dist(&self, b: Point) -> f64 {
        return (b - *self).length();
    }
}

pub use Vec3 as Color;

use crate::interval::Interval;
use crate::utils::{linear_to_gamma, rand_double, rand_range_double};
const COLOR_INTENSITY: Interval = Interval::new(0.0, 0.9999);
impl Color {
    pub fn print_color(&self, stream: &mut dyn Write) -> Result<(), std::io::Error> {
        let r = linear_to_gamma(self.x);
        let g = linear_to_gamma(self.y);
        let b = linear_to_gamma(self.z);

        let rbyte = (COLOR_INTENSITY.clamp(r) * 256.0) as u8;
        let gbyte = (COLOR_INTENSITY.clamp(g) * 256.0) as u8;
        let bbyte = (COLOR_INTENSITY.clamp(b) * 256.0) as u8;

        writeln!(stream, "{} {} {}", rbyte, gbyte, bbyte)
    }

    pub const BLACK: Self = Self::zero();
    pub const WHITE: Self = Self::val(1.0, 1.0, 1.0);
    pub const RED: Self = Self::val(1.0, 0.0, 0.0);
}
