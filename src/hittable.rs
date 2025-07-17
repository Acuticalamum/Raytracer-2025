use crate::aabb::AABB;
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};
use std::sync::Arc;

#[derive(Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub mat: Option<Arc<dyn Material>>,
    pub t: f64,
    pub front_face: bool,
    pub u: f64,
    pub v: f64,
    pub tangent: Vec3,
    pub bitangent: Vec3,
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
            u: 0.0,
            v: 0.0,
            tangent: Vec3::zero(),
            bitangent: Vec3::zero(),
        }
    }
}
pub trait Hittable: Send + Sync {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool;
    fn bounding_box(&self) -> AABB;
    fn pdf_value(&self, _origin: Point3, _direction: Vec3) -> f64 {
        0.0
    }
    fn random(&self, _origin: Point3) -> Vec3 {
        Vec3::new(1.0, 0.0, 0.0)
    }
}

pub struct Translate {
    object: Arc<dyn Hittable>,
    offset: Vec3,
    bbox: AABB,
}

impl Translate {
    pub fn new(object: Arc<dyn Hittable>, offset: Vec3) -> Self {
        let bbox = object.bounding_box() + offset;
        Self {
            object,
            offset,
            bbox,
        }
    }
}

impl Hittable for Translate {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let moved_r = Ray::new_with_time(r.origin() - self.offset, r.direction(), r.time());
        if !self.object.hit(&moved_r, ray_t, rec) {
            return false;
        }
        rec.p += self.offset;
        true
    }
    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}

pub struct RotateY {
    object: Arc<dyn Hittable>,
    sin_theta: f64,
    cos_theta: f64,
    bbox: AABB,
}

impl RotateY {
    pub fn new(object: Arc<dyn Hittable>, angle_degrees: f64) -> Self {
        let radians = angle_degrees.to_radians();
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        let mut min = Point3::new(f64::INFINITY, f64::INFINITY, f64::INFINITY);
        let mut max = Point3::new(f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY);

        let mut bbox = object.bounding_box();

        for i in 0..=1 {
            let x_val = if i == 1 { bbox.x.max } else { bbox.x.min };
            for j in 0..=1 {
                let y_val = if j == 1 { bbox.y.max } else { bbox.y.min };
                for k in 0..=1 {
                    let z_val = if k == 1 { bbox.z.max } else { bbox.z.min };
                    let new_x = cos_theta * x_val + sin_theta * z_val;
                    let new_z = -sin_theta * x_val + cos_theta * z_val;

                    let _tester = Vec3::new(new_x, y_val, new_z);

                    min.x = min.x.min(new_x);
                    max.x = max.x.max(new_x);
                    min.y = min.y.min(y_val);
                    max.y = max.y.max(y_val);
                    min.z = min.z.min(new_z);
                    max.z = max.z.max(new_z);
                }
            }
        }

        bbox = AABB::from_points(min, max);

        Self {
            object,
            sin_theta,
            cos_theta,
            bbox,
        }
    }
}

impl Hittable for RotateY {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let origin = Point3::new(
            self.cos_theta * r.origin().x - self.sin_theta * r.origin().z,
            r.origin().y,
            self.sin_theta * r.origin().x + self.cos_theta * r.origin().z,
        );

        let direction = Vec3::new(
            self.cos_theta * r.direction().x - self.sin_theta * r.direction().z,
            r.direction().y,
            self.sin_theta * r.direction().x + self.cos_theta * r.direction().z,
        );

        let rotated_r = Ray::new_with_time(origin, direction, r.time());

        if !self.object.hit(&rotated_r, ray_t, rec) {
            return false;
        }

        let p = Point3::new(
            self.cos_theta * rec.p.x + self.sin_theta * rec.p.z,
            rec.p.y,
            -self.sin_theta * rec.p.x + self.cos_theta * rec.p.z,
        );

        let normal = Vec3::new(
            self.cos_theta * rec.normal.x + self.sin_theta * rec.normal.z,
            rec.normal.y,
            -self.sin_theta * rec.normal.x + self.cos_theta * rec.normal.z,
        );

        rec.p = p;
        rec.normal = normal;

        true
    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}
