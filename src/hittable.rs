use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};
use std::sync::Arc;
use crate::aabb::AABB;

#[derive(Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub mat: Option<Arc<dyn Material>>,
    pub t: f64,
    pub front_face: bool,
}
impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        self.front_face = Vec3::dot(r.direction(), outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}

impl Default for HitRecord {
    fn default() -> HitRecord {
        HitRecord {
            p: Point3::zero(),
            normal: Vec3::zero(),
            mat: None,
            t: 0.0,
            front_face: true,
        }
    }
}
pub trait Hittable {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool;
    fn bounding_box(&self) -> AABB;
}
