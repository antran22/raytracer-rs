use std::f64;

#[derive(Clone, Copy, Debug)]
pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    pub const fn new(min: f64, max: f64) -> Self {
        Self { min, max }
    }

    pub fn new_checked(min: f64, max: f64) -> Self {
        if min > max {
            return Self { min: max, max: min };
        }
        Self { min, max }
    }

    pub fn join(a: &Self, b: &Self) -> Self {
        Self {
            min: f64::min(a.min, b.min),
            max: f64::max(a.max, b.max),
        }
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

    pub fn expand(&self, delta: f64) -> Interval {
        let delta = delta / 2.0;
        Interval {
            min: self.min - delta,
            max: self.max + delta,
        }
    }

    pub fn clamp(&self, x: f64) -> f64 {
        if x < self.min {
            return self.min;
        }
        if x > self.max {
            return self.max;
        }
        return x;
    }

    pub const ZERO: Interval = Interval::new(0.0, 0.0);
    pub const ONE: Interval = Interval::new(0.0, 1.0);
    pub const EMPTY: Interval = Interval::new(f64::INFINITY, f64::NEG_INFINITY);
    pub const UNIVERSE: Interval = Interval::new(f64::NEG_INFINITY, f64::INFINITY);
    pub const POSITIVE: Interval = Interval::new(0.0, f64::INFINITY);
}
