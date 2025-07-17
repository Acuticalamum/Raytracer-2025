use crate::vec3;
use vec3::Vec3;

#[allow(clippy::upper_case_acronyms)]
#[derive(Clone, Copy, Debug)]
pub struct ONB {
    axis: [Vec3; 3],
}

impl ONB {
    pub fn new(n: Vec3) -> Self {
        let w = Vec3::unit_vector(n);
        let a = if w.x.abs() > 0.9 {
            Vec3::new(0.0, 1.0, 0.0)
        } else {
            Vec3::new(1.0, 0.0, 0.0)
        };
        let v = Vec3::unit_vector(Vec3::cross(w, a));
        let u = Vec3::cross(w, v);

        Self { axis: [u, v, w] }
    }

    pub fn u(&self) -> Vec3 {
        self.axis[0]
    }

    pub fn v(&self) -> Vec3 {
        self.axis[1]
    }

    pub fn w(&self) -> Vec3 {
        self.axis[2]
    }

    pub fn transform(&self, local: Vec3) -> Vec3 {
        self.u() * local.x + self.v() * local.y + self.w() * local.z
    }
}
