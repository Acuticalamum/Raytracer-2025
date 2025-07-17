use crate::aabb::AABB;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::ray::Ray;
use crate::rtweekend;
use crate::vec3::{Point3, Vec3};
use std::sync::Arc;

#[derive(Clone)]
pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable>>,
    bbox: AABB,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            bbox: AABB::new(Interval::empty(), Interval::empty(), Interval::empty()),
        }
    }

    pub fn _from(object: Arc<dyn Hittable>) -> Self {
        let mut list = Self::new();
        list.add(object);
        list
    }

    pub fn _clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Arc<dyn Hittable>) {
        self.bbox = AABB::from_boxes(self.bbox, object.bounding_box());
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::default();
        let mut hit_anything = false;
        let mut closest_so_far = ray_t.max;

        for object in &self.objects {
            if object.hit(r, Interval::new(ray_t.min, closest_so_far), &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec.clone();
            }
        }

        hit_anything
    }
    fn bounding_box(&self) -> AABB {
        self.bbox
    }
    fn pdf_value(&self, origin: Point3, direction: Vec3) -> f64 {
        if self.objects.is_empty() {
            return 0.0;
        }

        let weight = 1.0 / self.objects.len() as f64;
        let mut sum = 0.0;

        for object in &self.objects {
            sum += weight * object.pdf_value(origin, direction);
        }

        sum
    }

    fn random(&self, origin: Point3) -> Vec3 {
        let len = self.objects.len() as i32;
        if len == 0 {
            return Vec3::new(1.0, 0.0, 0.0);
        }
        let index = rtweekend::random_int(0, len - 1);
        self.objects[index as usize].random(origin)
    }
}
