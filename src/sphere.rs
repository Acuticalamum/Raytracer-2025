use crate::aabb::AABB;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::material::Material;
use crate::onb::ONB;
use crate::ray::Ray;
use crate::rtweekend::INFINITY;
use crate::vec3::{Point3, Vec3};
use std::f64::consts::PI;
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
    pub fn get_sphere_uv(p: &Point3, u: &mut f64, v: &mut f64) {
        let theta = (-p.y).acos();
        let phi = (-p.z).atan2(p.x) + std::f64::consts::PI;
        *u = phi / (2.0 * std::f64::consts::PI);
        *v = theta / std::f64::consts::PI;
    }
}

impl Sphere {
    fn random_to_sphere(radius: f64, distance_squared: f64) -> Vec3 {
        let r1 = rand::random::<f64>();
        let r2 = rand::random::<f64>();
        let z = 1.0 + r2 * (f64::sqrt(1.0 - radius * radius / distance_squared) - 1.0);

        let phi = 2.0 * PI * r1;
        let x = f64::cos(phi) * f64::sqrt(1.0 - z * z);
        let y = f64::sin(phi) * f64::sqrt(1.0 - z * z);
        Vec3::new(x, y, z)
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
        Self::get_sphere_uv(&outward_normal, &mut rec.u, &mut rec.v);
        rec.mat = self.mat.clone();

        true
    }
    fn bounding_box(&self) -> AABB {
        self.bbox
    }

    fn pdf_value(&self, origin: Point3, direction: Vec3) -> f64 {
        let mut rec = HitRecord::default();
        if !self.hit(
            &Ray::new(origin, direction),
            Interval::new(0.001, INFINITY),
            &mut rec,
        ) {
            return 0.0;
        }

        let distance_squared = (self.center.at(0.0) - origin).length_squared();
        let cos_theta_max = f64::sqrt(1.0 - self.radius * self.radius / distance_squared);
        let solid_angle = 2.0 * PI * (1.0 - cos_theta_max);
        1.0 / solid_angle
    }

    fn random(&self, origin: Point3) -> Vec3 {
        let direction = self.center.at(0.0) - origin;
        let distance_squared = direction.length_squared();
        let uvw = ONB::new(direction);
        uvw.transform(Sphere::random_to_sphere(self.radius, distance_squared))
    }
}
