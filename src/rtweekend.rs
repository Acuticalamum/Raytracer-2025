use rand::Rng;
use std::f64::consts::PI;
use std::sync::Arc;

pub const INFINITY: f64 = f64::INFINITY;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

pub fn random_double() -> f64 {
    let mut rng = rand::rng();
    rng.random_range(0.0..1.0) // [0.0, 1.0)
}

pub fn random_double_range(min: f64, max: f64) -> f64 {
    let mut rng = rand::rng();
    rng.random_range(min..max) // [min, max)
}

pub fn random_int(min: i32, max: i32) -> i32 {
    random_double_range(min as f64, (max + 1) as f64) as i32
}
