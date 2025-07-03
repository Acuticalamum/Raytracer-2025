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

    pub fn noise(&self, p: &Point3) -> f64 {
        let i = ((4.0 * p.x()).floor() as isize & 255) as usize;
        let j = ((4.0 * p.y()).floor() as isize & 255) as usize;
        let k = ((4.0 * p.z()).floor() as isize & 255) as usize;

        let idx = self.perm_x[i] ^ self.perm_y[j] ^ self.perm_z[k];
        self.rand_float[idx]
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
}
