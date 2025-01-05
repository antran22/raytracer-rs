use std::io::Write;
use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Sub, SubAssign};

#[derive(Copy, Clone)]
pub struct Vec3 {
    e: [f64; 3],
}

impl Vec3 {
    pub fn zero() -> Vec3 {
        Self { e: [0.0, 0.0, 0.0] }
    }

    pub fn val(a: f64, b: f64, c: f64) -> Vec3 {
        Self { e: [a, b, c] }
    }

    pub fn x(&self) -> f64 {
        self.e[0]
    }
    pub fn y(&self) -> f64 {
        self.e[1]
    }
    pub fn z(&self) -> f64 {
        self.e[2]
    }

    pub fn length_squared(&self) -> f64 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn dot(&self, b: Vec3) -> f64 {
        self.e[0] * b.e[0] + self.e[1] + b.e[1] + self.e[2] * b.e[2]
    }

    pub fn cross(&self, b: Vec3) -> Vec3 {
        Vec3::val(
            self.e[1] * b.e[2] - self.e[2] * b.e[1],
            self.e[2] * b.e[0] - self.e[0] * b.e[2],
            self.e[0] * b.e[1] - self.e[1] * b.e[0],
        )
    }

    pub fn unit_vec(&self) -> Vec3 {
        *self / self.length()
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3::val(
            self.x() + other.x(),
            self.y() + other.y(),
            self.z() + other.z(),
        )
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3::val(
            self.x() - other.x(),
            self.y() - other.y(),
            self.z() - other.z(),
        )
    }
}

impl Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3::val(
            self.x() * other.x(),
            self.y() * other.y(),
            self.z() * other.z(),
        )
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Vec3) {
        self.e[0] += other.x();
        self.e[1] += other.y();
        self.e[2] += other.z();
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Vec3) {
        self.e[0] -= other.x();
        self.e[1] -= other.y();
        self.e[2] -= other.z();
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, scalar: f64) -> Vec3 {
        Vec3::val(self.x() * scalar, self.y() * scalar, self.z() * scalar)
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, scalar: f64) {
        self.e[0] *= scalar;
        self.e[1] *= scalar;
        self.e[2] *= scalar;
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
        &self.e[index]
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.e[index]
    }
}

pub use Vec3 as Point;
impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x(), self.y(), self.z())
    }
}

pub use Vec3 as Color;

impl Color {
    pub fn print_color(&self, stream: &mut dyn Write) -> Result<(), std::io::Error> {
        let r = (self.x() * 255.999) as u8;
        let g = (self.y() * 255.999) as u8;
        let b = (self.z() * 255.999) as u8;
        
        writeln!(stream, "{} {} {}", r, g, b)
    }
    
}
