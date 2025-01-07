use std::f64;

pub struct Interval {
    pub min: f64,
    pub max: f64,
}

pub const EMPTY: Interval = Interval::new(f64::INFINITY, f64::NEG_INFINITY);
pub const UNIVERSE: Interval = Interval::new(f64::NEG_INFINITY, f64::INFINITY);
pub const POSITIVE: Interval = Interval::new(0.0, f64::INFINITY);

impl Interval {
    pub const fn universe() -> Self {
        UNIVERSE
    }

    pub const fn new(min: f64, max: f64) -> Self {
        Self { min: min, max: max }
    }

    pub fn size(&self) -> f64 {
        self.max - self.min
    }

    pub fn contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }
}
