use crate::aabb::AABB;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};
use std::sync::Arc;

pub struct Sphere {
    center: Ray,
    radius: f64,
    mat: Option<Arc<dyn Material>>,
    bbox: AABB,
}
impl Sphere {
    pub fn static_new(static_center: Point3, radius: f64, mat: Option<Arc<dyn Material>>) -> Self {
        let rvec = Vec3::new(radius, radius, radius);
        Self {
            center: Ray::new(static_center, Vec3::new(0.0, 0.0, 0.0)),
            radius: f64::max(0.0, radius),
            mat,
            bbox: AABB::from_points(static_center - rvec, static_center + rvec),
        }
    }
    pub fn new(
        center1: Point3,
        center2: Point3,
        radius: f64,
        mat: Option<Arc<dyn Material>>,
    ) -> Self {
        let rvec = Vec3::new(radius, radius, radius);
        let bbox1 = AABB::from_points(center1 - rvec, center1 + rvec);
        let bbox2 = AABB::from_points(center2 - rvec, center2 + rvec);
        Self {
            center: Ray::new(center1, center2 - center1),
            radius: f64::max(0.0, radius),
            mat,
            bbox: AABB::from_boxes(bbox1, bbox2),
        }
    }
}
impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let current_center = self.center.at(r.time());
        let oc = current_center - r.origin();
        let a = r.direction().length_squared();
        let h = Vec3::dot(oc, r.direction());
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return false;
        }

        let sqrtd = discriminant.sqrt();

        let mut root = (h - sqrtd) / a;
        if root <= ray_t.min || root >= ray_t.max {
            root = (h + sqrtd) / a;
            if root <= ray_t.min || root >= ray_t.max {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = (rec.p - current_center) / self.radius;
        rec.set_face_normal(r, outward_normal);
        rec.mat = self.mat.clone();

        true
    }
    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}
