pub use super::vec3::Vec3 as Point;

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
