// ray.rs

use crate::vec3::{Point3, Vec3};
use std::f64;

#[derive(Debug)]
pub struct Ray {
    orig: Point3,
    dir: Vec3,
    tm: f64,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Ray {
            orig: origin,
            dir: direction,
            tm: 0.0,
        }
    }
    pub fn new_with_time(origin: Point3, direction: Vec3, time: f64) -> Self {
        let mut r = Ray::new(origin, direction);
        r.tm = time;
        r
    }
    pub fn origin(&self) -> Point3 {
        self.orig
    }
    pub fn direction(&self) -> Vec3 {
        self.dir
    }
    pub fn time(&self) -> f64 {
        self.tm
    }
    pub fn at(&self, t: f64) -> Point3 {
        self.orig + self.dir * t
    }
}
