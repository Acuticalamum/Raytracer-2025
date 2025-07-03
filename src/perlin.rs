use rand::prelude::*;
use rand::rngs::ThreadRng;

use crate::rtweekend;
use crate::vec3::Point3;
use array_init::array_init;

const POINT_COUNT: usize = 256;

pub struct Perlin {
    rand_float: [f64; POINT_COUNT],
    perm_x: [usize; POINT_COUNT],
    perm_y: [usize; POINT_COUNT],
    perm_z: [usize; POINT_COUNT],
}

impl Perlin {
    pub fn new() -> Self {
        let mut rng = rand::rng();

        let mut rand_float = [0.0; POINT_COUNT];
        for i in 0..POINT_COUNT {
            rand_float[i] = rtweekend::random_double();
        }

        let perm_x = Self::generate_perm(&mut rng);
        let perm_y = Self::generate_perm(&mut rng);
        let perm_z = Self::generate_perm(&mut rng);

        Self {
            rand_float,
            perm_x,
            perm_y,
            perm_z,
        }
    }

    pub fn noise(&self, p: Point3) -> f64 {
        let u = p.x() - p.x().floor();
        let v = p.y() - p.y().floor();
        let w = p.z() - p.z().floor();

        let u = u * u * (3.0 - 2.0 * u);
        let v = v * v * (3.0 - 2.0 * v);
        let w = w * w * (3.0 - 2.0 * w);

        let i = (p.x().floor() + 10000.0) as usize;
        let j = (p.y().floor() + 10000.0) as usize;
        let k = (p.z().floor() + 10000.0) as usize;

        let mut c = [[[0.0; 2]; 2]; 2];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    let idx = self.perm_x[(i + di) & 255]
                        ^ self.perm_y[(j + dj) & 255]
                        ^ self.perm_z[(k + dk) & 255];
                    c[di][dj][dk] = self.rand_float[idx];
                }
            }
        }

        Perlin::trilinear_interp(&c, u, v, w)
    }

    fn generate_perm(rng: &mut ThreadRng) -> [usize; POINT_COUNT] {
        let mut p: [usize; POINT_COUNT] = array_init(|i| i);
        Self::permute(&mut p, rng);
        p
    }

    fn permute(p: &mut [usize; POINT_COUNT], rng: &mut ThreadRng) {
        for i in (1..POINT_COUNT).rev() {
            let target = rng.random_range(0..=i);
            p.swap(i, target);
        }
    }

    fn trilinear_interp(c: &[[[f64; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let mut accum = 0.0;
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let weight = (i as f64 * u + (1.0 - i as f64) * (1.0 - u))
                        * (j as f64 * v + (1.0 - j as f64) * (1.0 - v))
                        * (k as f64 * w + (1.0 - k as f64) * (1.0 - w));
                    accum += weight * c[i][j][k];
                }
            }
        }
        accum
    }
}
