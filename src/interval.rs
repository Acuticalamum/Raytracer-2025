use ::std::ops::Add;
#[derive(Debug, Clone, Copy)]
pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    pub fn empty() -> Self {
        Self {
            min: f64::INFINITY,
            max: f64::NEG_INFINITY,
        }
    }
    pub fn new(min: f64, max: f64) -> Self {
        Self { min, max }
    }
    pub fn new_ab(a: f64, b: f64) -> Self {
        let min = if a < b { a } else { b };
        let max = if a > b { a } else { b };
        Self::new(min, max)
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
    pub fn universe() -> Self {
        Self {
            min: f64::NEG_INFINITY,
            max: f64::INFINITY,
        }
    }
    pub fn intersection(&self, other: &Self) -> Self {
        let min = self.min.max(other.min);
        let max = self.max.min(other.max);
        if min <= max {
            Self { min, max }
        } else {
            Self::empty()
        }
    }
    pub fn union(&self, other: &Self) -> Self {
        let min = self.min.min(other.min);
        let max = self.max.max(other.max);
        Self { min, max }
    }
    pub fn clamp(&self, x: f64) -> f64 {
        if x < self.min {
            return self.min;
        }
        if x > self.max {
            return self.max;
        }
        x
    }
    pub fn expand(&self, delta: f64) -> Self {
        let padding = delta / 2.0;
        Interval::new(self.min - padding, self.max + padding)
    }
}

impl Interval {
    pub fn offset(self, displacement: f64) -> Interval {
        Interval {
            min: self.min + displacement,
            max: self.max + displacement,
        }
    }
}

impl Add<f64> for Interval {
    type Output = Interval;
    fn add(self, rhs: f64) -> Interval {
        self.offset(rhs)
    }
}
