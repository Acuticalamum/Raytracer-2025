use crate::aabb::AABB;
use crate::color::Color;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::material::{Isotropic, Material};
use crate::ray::Ray;
use crate::rtweekend;
use crate::texture::Texture;
use crate::vec3::{Point3, Vec3};
use std::sync::Arc;

pub struct ConstantMedium {
    boundary: Arc<dyn Hittable>,
    neg_inv_density: f64,
    phase_function: Arc<dyn Material>,
}

impl ConstantMedium {
    pub fn new_with_texture(
        boundary: Arc<dyn Hittable>,
        density: f64,
        texture: Arc<dyn Texture>,
    ) -> Self {
        Self {
            boundary,
            neg_inv_density: -1.0 / density,
            phase_function: Arc::new(Isotropic::new_with_texture(texture)),
        }
    }

    pub fn new_with_color(boundary: Arc<dyn Hittable>, density: f64, albedo: Color) -> Self {
        Self {
            boundary,
            neg_inv_density: -1.0 / density,
            phase_function: Arc::new(Isotropic::new_with_color(albedo)),
        }
    }
}

impl Hittable for ConstantMedium {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let mut rec1 = HitRecord::default();
        let mut rec2 = HitRecord::default();

        if !self.boundary.hit(r, Interval::universe(), &mut rec1) {
            return false;
        }

        if !self
            .boundary
            .hit(r, Interval::new(rec1.t + 0.0001, f64::INFINITY), &mut rec2)
        {
            return false;
        }

        let mut t1 = rec1.t.max(ray_t.min);
        let mut t2 = rec2.t.min(ray_t.max);

        if t1 >= t2 {
            return false;
        }

        if t1 < 0.0 {
            t1 = 0.0;
        }

        let ray_length = r.direction().length();
        let distance_inside_boundary = (t2 - t1) * ray_length;
        let hit_distance = self.neg_inv_density * rtweekend::random_double().ln();

        if hit_distance > distance_inside_boundary {
            return false;
        }

        rec.t = t1 + hit_distance / ray_length;
        rec.p = r.at(rec.t);

        rec.normal = Vec3::new(1.0, 0.0, 0.0);
        rec.front_face = true;
        rec.mat = Some(self.phase_function.clone());

        true
    }

    fn bounding_box(&self) -> AABB {
        self.boundary.bounding_box()
    }
}
