use std::f64::{INFINITY, NEG_INFINITY};

pub struct Interval {
    pub min: f64,
    pub max: f64,
}

// static constant intervals & constructors
impl Interval {
    const EMPTY: Interval = Interval {
        min: INFINITY,
        max: NEG_INFINITY,
    };

    const UNIVERSE: Interval = Interval {
        min: NEG_INFINITY,
        max: INFINITY,
    };

    pub const fn empty() -> Self {
        Self::EMPTY
    }

    pub fn new(min: f64, max: f64) -> Self {
        Interval { min, max }
    }
}

// methods
impl Interval {
    pub fn size(&self) -> f64 {
        self.max - self.min
    }

    pub fn contains(&self, point: f64) -> bool {
        self.min <= point && point <= self.max
    }

    pub fn surrounds(&self, point: f64) -> bool {
        self.min < point && point < self.max
    }
}
