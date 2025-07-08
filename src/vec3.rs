use crate::rtweekend;
use crate::rtweekend::random_double;
use rand::Rng;
use std::f64;
use std::fmt;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
    pub(crate) x: f64,
    pub(crate) y: f64,
    pub(crate) z: f64,
}

pub type Point3 = Vec3;

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
    pub fn zero() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
    pub fn x(&self) -> f64 {
        self.x
    }
    pub fn y(&self) -> f64 {
        self.y
    }
    pub fn z(&self) -> f64 {
        self.z
    }
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }
    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    pub fn near_zero(&self) -> bool {
        const EPS: f64 = 1e-8;
        self.x < EPS && self.y < EPS && self.z < EPS
    }
    pub fn random() -> Self {
        Vec3::new(
            rtweekend::random_double(),
            rtweekend::random_double(),
            rtweekend::random_double(),
        )
    }
    pub fn random_range(min: f64, max: f64) -> Self {
        Vec3::new(
            rtweekend::random_double_range(min, max),
            rtweekend::random_double_range(min, max),
            rtweekend::random_double_range(min, max),
        )
    }
    pub fn unit_vector(v: Vec3) -> Self {
        v / v.length()
    }
    pub fn random_unit_vector() -> Self {
        loop {
            let p = Vec3::random_range(-1.0, 1.0);
            let lensq = p.length_squared();
            if 1e-160 <= lensq && lensq <= 1.0 {
                return p / lensq.sqrt();
            }
        }
    }
    pub fn random_on_hemisphere(normal: Vec3) -> Self {
        let on_unit_sphere = Vec3::random_unit_vector();
        if Vec3::dot(on_unit_sphere, normal) > 0.0 {
            on_unit_sphere
        } else {
            -on_unit_sphere
        }
    }
    pub fn random_in_unit_disk() -> Self {
        loop {
            let p = Vec3::new(
                rtweekend::random_double_range(-1.0, 1.0),
                rtweekend::random_double_range(-1.0, 1.0),
                0.0,
            );
            if (p.length_squared() < 1.0) {
                return p;
            }
        }
    }
    pub fn random_cosine_direction() -> Vec3 {
        let r1: f64 = random_double();
        let r2: f64 = random_double();

        let phi = 2.0 * std::f64::consts::PI * r1;
        let x = phi.cos() * r2.sqrt();
        let y = phi.sin() * r2.sqrt();
        let z = (1.0 - r2).sqrt();
        
        Vec3::new(x, y, z)
    }
}

impl Add for Vec3 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;
    fn mul(self, t: f64) -> Self {
        Self {
            x: self.x * t,
            y: self.y * t,
            z: self.z * t,
        }
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, t: f64) {
        self.x *= t;
        self.y *= t;
        self.z *= t;
    }
}

impl Mul for Vec3 {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, t: f64) -> Self {
        self * (1.0 / t)
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, t: f64) {
        *self *= 1.0 / t;
    }
}

impl Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Vec3 {
    pub fn dot(u: Vec3, v: Vec3) -> f64 {
        u.x * v.x + u.y * v.y + u.z * v.z
    }
    pub fn cross(u: Vec3, v: Vec3) -> Vec3 {
        Vec3 {
            x: u.y * v.z - u.z * v.y,
            y: u.z * v.x - u.x * v.z,
            z: u.x * v.y - u.y * v.x,
        }
    }
    pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
        v - n * (Vec3::dot(v, n) * 2.0)
    }
    pub fn refract(uv: Vec3, n: Vec3, etai_over_etat: f64) -> Vec3 {
        let cos_theta = f64::min(Vec3::dot(-uv, n), 1.0);
        let r_out_perp = (uv + n * cos_theta) * etai_over_etat;
        let r_out_parallel = -n * (1.0 - r_out_perp.length_squared()).abs().sqrt();
        r_out_perp + r_out_parallel
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}
