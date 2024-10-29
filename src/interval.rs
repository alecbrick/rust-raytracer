use once_cell::sync::Lazy;

pub struct Interval {
    pub min: f32,
    pub max: f32,
}

impl Interval {
    pub fn new(min: f32, max: f32) -> Interval {
        Interval {
            min: min,
            max: max,
        }
    }

    pub fn size(&self) -> f32 {
        self.max - self.min
    }

    pub fn contains(&self, x: f32) -> bool {
        (self.min <= x) && (x <= self.max)
    }

    pub fn surrounds(&self, x: f32) -> bool {
        (self.min < x) && (x < self.max)
    }

    pub fn clamp(&self, x: f32) -> f32 {
        if x < self.min {
            self.min
        } else if x > self.max {
            self.max
        } else {
            x
        }
    }
}

impl Default for Interval {
    fn default() -> Interval {
        Interval {
            min: f32::MAX,
            max: f32::MIN,
        }
    }
}

pub const EMPTY: Lazy<Interval> = Lazy::new(|| {
    Interval::new(f32::MAX, f32::MIN)
});
pub const UNIVERSE: Lazy<Interval> = Lazy::new(|| {
    Interval::new(f32::MIN, f32::MAX)
});