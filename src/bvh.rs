use crate::aabb::AABB;
use crate::hittable::{HitRecord, Hittable};
use crate::hittable_list::HittableList;
use crate::interval::Interval;
use crate::ray::Ray;
use std::cmp::Ordering;
use std::sync::Arc;

#[derive(Clone)]

pub struct BVHNode {
    left: Option<Arc<dyn Hittable>>,
    right: Option<Arc<dyn Hittable>>,
    bbox: AABB,
}

impl BVHNode {
    fn box_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>, axis_index: usize) -> Ordering {
        let a_box = a.as_ref().bounding_box();
        let a_axis_interval = a_box.axis_interval(axis_index);
        let b_box = b.as_ref().bounding_box();
        let b_axis_interval = b_box.axis_interval(axis_index);
        a_axis_interval
            .min
            .partial_cmp(&b_axis_interval.min)
            .unwrap_or(Ordering::Equal)
    }
    fn box_x_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
        Self::box_compare(a, b, 0)
    }
    fn box_y_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
        Self::box_compare(a, b, 1)
    }
    fn box_z_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
        Self::box_compare(a, b, 2)
    }
}

impl BVHNode {
    pub fn new_from_list(list: &mut HittableList) -> Self {
        let len = list.objects.len();
        BVHNode::new(&mut list.objects, 0, len)
    }
    pub fn new(objects: &mut Vec<Arc<dyn Hittable>>, start: usize, end: usize) -> Self {
        let mut bbox = AABB::empty();
        for object_index in start..end {
            bbox = AABB::from_boxes(bbox, objects[object_index].bounding_box());
        }
        let axis = bbox.longest_axis();
        let comparator: Box<dyn Fn(&Arc<dyn Hittable>, &Arc<dyn Hittable>) -> Ordering> = match axis
        {
            0 => Box::new(BVHNode::box_x_compare),
            1 => Box::new(BVHNode::box_y_compare),
            _ => Box::new(BVHNode::box_z_compare),
        };
        let object_span = end - start;
        let (left, right) = if object_span == 1 {
            (Some(objects[start].clone()), Some(objects[start].clone()))
        } else if object_span == 2 {
            (
                Some(objects[start].clone()),
                Some(objects[start + 1].clone()),
            )
        } else {
            objects[start..end].sort_by(comparator);
            let mid = start + object_span / 2;
            (
                Some(Arc::new(BVHNode::new(objects, start, mid)) as Arc<dyn Hittable>),
                Some(Arc::new(BVHNode::new(objects, mid, end)) as Arc<dyn Hittable>),
            )
        };
        //let bbox = AABB::from_boxes(left.as_ref().unwrap().bounding_box(), right.as_ref().unwrap().bounding_box());
        BVHNode { left, right, bbox }
    }
}

impl Hittable for BVHNode {
    fn hit(&self, r: &Ray, t: Interval, rec: &mut HitRecord) -> bool {
        if !self.bbox.hit(r, t) {
            return false;
        }
        let mut hit_left = false;
        let mut hit_right = false;
        if self.left.is_some() {
            hit_left = self.left.as_ref().unwrap().hit(r, t, rec);
        }
        if self.right.is_some() {
            let new_max = if hit_left { rec.t } else { t.max };
            let new_t = Interval::new(t.min, new_max);
            hit_right = self.right.as_ref().unwrap().hit(r, new_t, rec);
        }
        hit_left || hit_right
    }
    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}
