use crate::texture::NormalTexture;
use crate::{
    aabb::AABB,
    hittable::{HitRecord, Hittable},
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
    d: f64,
    area: f64,
    pub uv0: Point3,
    pub uv1: Point3,
    pub uv2: Point3,
    normal_map: Option<Arc<NormalTexture>>,
}

impl Triangle {
    pub fn _new_with_points(q: Point3, a: Point3, b: Point3, mat: Arc<dyn Material>) -> Self {
        Triangle::_new_with_vector(q, a - q, b - q, mat)
    }

    pub fn _new_with_vector(q: Point3, u: Vec3, v: Vec3, mat: Arc<dyn Material>) -> Self {
        let n = Vec3::cross(u, v);
        let normal = Vec3::unit_vector(n);
        let d = Vec3::dot(q, normal);
        let w = n / Vec3::dot(n, n);
        let area = n.length() / 2.0;
        let mut triangle = Self {
            q,
            u,
            v,
            w,
            mat,
            bbox: AABB::empty(),
            normal,
            d,
            area,
            uv0: Point3::new(0.0, 0.0, 0.0),
            uv1: Point3::new(1.0, 0.0, 0.0),
            uv2: Point3::new(0.0, 1.0, 0.0),
            normal_map: None,
        };
        triangle._set_bounding_box();
        triangle
    }

    fn _set_bounding_box(&mut self) {
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

        let t = (self.d - Vec3::dot(self.normal, r.origin())) / denom;

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

        let p0 = self.q;
        let p1 = self.q + self.u;
        let p2 = self.q + self.v;

        let uv0 = self.uv0;
        let uv1 = self.uv1;
        let uv2 = self.uv2;

        let edge1 = p1 - p0;
        let edge2 = p2 - p0;
        let delta_uv1 = uv1 - uv0;
        let delta_uv2 = uv2 - uv0;

        let f = 1.0 / (delta_uv1.x * delta_uv2.y - delta_uv2.x * delta_uv1.y);
        let tangent = (edge1 * delta_uv2.y - edge2 * delta_uv1.y) * f;
        let bitangent = (edge2 * delta_uv1.x - edge1 * delta_uv2.x) * f;

        rec.tangent = Vec3::unit_vector(tangent);
        rec.bitangent = Vec3::unit_vector(bitangent);

        if let Some(normal_map) = &self.normal_map {
            let normal = normal_map.normal(rec.u, rec.v, &rec.p);

            let t = rec.tangent;
            let b = rec.bitangent;
            let n = rec.normal;

            let world_normal = Vec3::unit_vector(t * normal.x() + b * normal.y() + n * normal.z());
            rec.set_face_normal(r, world_normal);
        }

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
