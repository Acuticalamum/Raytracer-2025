use crate::vec3::{Point3, Vec3};
use array_init::array_init;
use rand::prelude::*;
use rand::rngs::ThreadRng;

const POINT_COUNT: usize = 256;

pub struct Perlin {
    perm_x: [usize; POINT_COUNT],
    perm_y: [usize; POINT_COUNT],
    perm_z: [usize; POINT_COUNT],
    rand_vec: [Vec3; POINT_COUNT],
}

impl Perlin {
    pub fn _new() -> Self {
        let mut rng = rand::rng();

        let mut rand_vec = [Vec3::zero(); POINT_COUNT];
        for val in rand_vec.iter_mut().take(POINT_COUNT) {
            *val = Vec3::unit_vector(Vec3::random_range(-1.0, 1.0));
        }

        let perm_x = Self::_generate_perm(&mut rng);
        let perm_y = Self::_generate_perm(&mut rng);
        let perm_z = Self::_generate_perm(&mut rng);

        Self {
            rand_vec,
            perm_x,
            perm_y,
            perm_z,
        }
    }

    pub fn noise(&self, p: Point3) -> f64 {
        let u = p.x() - p.x().floor();
        let v = p.y() - p.y().floor();
        let w = p.z() - p.z().floor();

        let i = (p.x().floor() + 10000.0) as usize;
        let j = (p.y().floor() + 10000.0) as usize;
        let k = (p.z().floor() + 10000.0) as usize;

        let mut c = [[[Vec3::zero(); 2]; 2]; 2];

        for (di, plane) in c.iter_mut().enumerate() {
            for (dj, row) in plane.iter_mut().enumerate() {
                for (dk, val) in row.iter_mut().enumerate() {
                    let idx = self.perm_x[(i + di) & 255]
                        ^ self.perm_y[(j + dj) & 255]
                        ^ self.perm_z[(k + dk) & 255];
                    *val = self.rand_vec[idx];
                }
            }
        }

        Perlin::trilinear_interp(&c, u, v, w)
    }

    pub fn turb(&self, mut p: Point3, depth: usize) -> f64 {
        let mut accum = 0.0;
        let mut weight = 1.0;

        for _ in 0..depth {
            accum += weight * self.noise(p);
            weight *= 0.5;
            p *= 2.0;
        }

        accum.abs()
    }

    fn _generate_perm(rng: &mut ThreadRng) -> [usize; POINT_COUNT] {
        let mut p: [usize; POINT_COUNT] = array_init(|i| i);
        Self::_permute(&mut p, rng);
        p
    }

    fn _permute(p: &mut [usize; POINT_COUNT], rng: &mut ThreadRng) {
        for i in (1..POINT_COUNT).rev() {
            let target = rng.random_range(0..=i);
            p.swap(i, target);
        }
    }

    fn trilinear_interp(c: &[[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);

        let mut accum = 0.0;
        for (i, plane) in c.iter().enumerate() {
            for (j, row) in plane.iter().enumerate() {
                for (k, &val) in row.iter().enumerate() {
                    let weight_v = Vec3::new(u - i as f64, v - j as f64, w - k as f64);
                    let weight = (i as f64 * uu + (1.0 - i as f64) * (1.0 - uu))
                        * (j as f64 * vv + (1.0 - j as f64) * (1.0 - vv))
                        * (k as f64 * ww + (1.0 - k as f64) * (1.0 - ww));
                    accum += weight * Vec3::dot(weight_v, val);
                }
            }
        }
        accum
    }
}
