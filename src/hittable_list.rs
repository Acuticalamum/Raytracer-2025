use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::ray::Ray;
use std::sync::Arc;
use crate::aabb::AABB;
use crate::vec3::Vec3;

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

    pub fn from(object: Arc<dyn Hittable>) -> Self {
        let mut list = Self::new();
        list.add(object);
        list
    }

    pub fn clear(&mut self) {
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
}
