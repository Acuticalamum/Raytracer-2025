use crate::rtweekend::random_double;
use crate::{
    aabb::AABB,
    hittable::{HitRecord, Hittable},
    hittable_list::HittableList,
    interval::Interval,
    material::Material,
    ray::Ray,
    rtweekend,
    vec3::{Point3, Vec3},
};
use std::sync::Arc;

pub struct Triangle {
    q: Point3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    mat: Arc<dyn Material>,
    bbox: AABB,
    normal: Vec3,
    D: f64,
    area: f64,
}

impl Triangle {
    pub fn new_with_points(q: Point3, a: Point3, b: Point3, mat: Arc<dyn Material>) -> Self {
        Triangle::new_with_vector(q, a - q, b - q, mat)
    }

    pub fn new_with_vector(q: Point3, u: Vec3, v: Vec3, mat: Arc<dyn Material>) -> Self {
        let n = Vec3::cross(u, v);
        let normal = Vec3::unit_vector(n);
        let D = Vec3::dot(q, normal);
        let w = n / Vec3::dot(n, n);
        let area = n.length() / 2.0;
        let mut quad = Self {
            q,
            u,
            v,
            w,
            mat,
            bbox: AABB::empty(),
            normal,
            D,
            area,
        };
        quad.set_bounding_box();
        quad
    }

    fn set_bounding_box(&mut self) {
        let diagonal1 = AABB::from_points(self.q, self.q + self.u + self.v);
        let diagonal2 = AABB::from_points(self.q + self.u, self.q + self.v);
        self.bbox = AABB::from_boxes(diagonal1, diagonal2);
        self.bbox.pad_to_minimums();
    }

    fn is_interior(a: f64, b: f64, rec: &mut HitRecord) -> bool {
        let unit_interval = Interval::new(0.0, 1.0);
        if !unit_interval.contains(a)
            || !unit_interval.contains(b)
            || !unit_interval.contains(a + b)
        {
            return false;
        }
        rec.u = a;
        rec.v = b;
        true
    }
}

impl Hittable for Triangle {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let denom = Vec3::dot(r.direction(), self.normal);

        if denom.abs() < 1e-6 {
            return false;
        }

        let t = (self.D - Vec3::dot(self.normal, r.origin())) / denom;

        if !ray_t.contains(t) {
            return false;
        }

        let intersection = r.at(t);

        let planar_hitpt_vector = intersection - self.q;
        let alpha = Vec3::dot(self.w, Vec3::cross(planar_hitpt_vector, self.v));
        let beta = Vec3::dot(self.w, Vec3::cross(self.u, planar_hitpt_vector));

        if !Triangle::is_interior(alpha, beta, rec) {
            return false;
        }

        rec.t = t;
        rec.p = intersection;
        rec.mat = Some(Arc::clone(&self.mat));
        rec.set_face_normal(r, self.normal);

        true
    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }

    fn pdf_value(&self, origin: Point3, direction: Vec3) -> f64 {
        let ray = Ray::new_with_time(origin, direction, 0.0);
        let mut rec = HitRecord::default();

        if !self.hit(&ray, Interval::new(0.001, f64::INFINITY), &mut rec) {
            return 0.0;
        }

        let distance_squared = rec.t * rec.t * direction.length_squared();
        let cosine = (Vec3::dot(direction, rec.normal) / direction.length()).abs();

        distance_squared / (cosine * self.area)
    }

    fn random(&self, origin: Point3) -> Vec3 {
        let alpha = rtweekend::random_double_range(0.0, 1.0);
        let beta = rtweekend::random_double_range(0.0, 1.0 - alpha);
        let random_point = self.q + self.u * alpha + self.v * beta;
        random_point - origin
    }
}
