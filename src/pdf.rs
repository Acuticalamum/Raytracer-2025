use crate::onb::ONB;
use crate::vec3::Vec3;
use std::f64::consts::PI;

pub trait Pdf {
    fn value(&self, direction: Vec3) -> f64;
    fn generate(&self) -> Vec3;
}

pub struct SpherePdf;

impl Pdf for SpherePdf {
    fn value(&self, _direction: Vec3) -> f64 {
        1.0 / (4.0 * PI)
    }

    fn generate(&self) -> Vec3 {
        Vec3::random_unit_vector()
    }
}

pub struct CosinePdf {
    uvw: ONB,
}

impl CosinePdf {
    pub fn new(w: Vec3) -> Self {
        Self {
            uvw: ONB::new(Vec3::unit_vector(w)),
        }
    }
}

impl Pdf for CosinePdf {
    fn value(&self, direction: Vec3) -> f64 {
        let cosine = Vec3::dot(Vec3::unit_vector(direction), self.uvw.w());
        if cosine <= 0.0 { 0.0 } else { cosine / PI }
    }

    fn generate(&self) -> Vec3 {
        self.uvw.transform(Vec3::random_cosine_direction())
    }
}
