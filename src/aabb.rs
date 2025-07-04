use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};
use std::cmp::{max, min};
use std::ops::Add;

#[derive(Clone, Copy)]
pub struct AABB {
    pub x: Interval,
    pub y: Interval,
    pub z: Interval,
}

impl AABB {
    pub fn empty() -> Self {
        AABB {
            x: Interval::empty(),
            y: Interval::empty(),
            z: Interval::empty(),
        }
    }

    pub fn new(x: Interval, y: Interval, z: Interval) -> Self {
        let mut ret = AABB { x, y, z };
        ret.pad_to_minimums();
        ret
    }

    pub fn from_points(a: Point3, b: Point3) -> Self {
        let x = if a.x <= b.x {
            Interval::new(a.x, b.x)
        } else {
            Interval::new(b.x, a.x)
        };

        let y = if a.y <= b.y {
            Interval::new(a.y, b.y)
        } else {
            Interval::new(b.y, a.y)
        };

        let z = if a.z <= b.z {
            Interval::new(a.z, b.z)
        } else {
            Interval::new(b.z, a.z)
        };

        AABB { x, y, z }
    }

    pub fn from_boxes(box0: AABB, box1: AABB) -> Self {
        Self {
            x: box0.x.union(&box1.x),
            y: box0.y.union(&box1.y),
            z: box0.z.union(&box1.z),
        }
    }

    pub fn axis_interval(&self, n: usize) -> &Interval {
        match n {
            0 => &self.x,
            1 => &self.y,
            _ => &self.z,
        }
    }

    pub fn longest_axis(&self) -> usize {
        if self.x.size() > self.y.size() {
            if (self.x.size() > self.z.size()) {
                0
            } else {
                2
            }
        } else {
            if (self.y.size() > self.z.size()) {
                1
            } else {
                2
            }
        }
    }

    pub fn pad_to_minimums(&mut self) {
        let delta = 0.0001;

        if self.x.size() < delta {
            self.x = self.x.expand(delta);
        }
        if self.y.size() < delta {
            self.y = self.y.expand(delta);
        }
        if self.z.size() < delta {
            self.z = self.z.expand(delta);
        }
    }

    pub fn hit(&self, r: &Ray, t: Interval) -> bool {
        let ray_orig = r.origin();
        let ray_dir = r.direction();

        for axis in 0..3 {
            let ax = self.axis_interval(axis);
            let x_y_z_dir = match axis {
                0 => ray_dir.x(),
                1 => ray_dir.y(),
                _ => ray_dir.z(),
            };
            let x_y_z_ori = match axis {
                0 => ray_orig.x(),
                1 => ray_orig.y(),
                _ => ray_orig.z(),
            };
            let adinv = 1.0 / x_y_z_dir;

            let t0 = (ax.min - x_y_z_ori) * adinv;
            let t1 = (ax.max - x_y_z_ori) * adinv;

            let mut ray_t = Interval::new(t.min, t.max);

            if t0 < t1 {
                ray_t.min = ray_t.min.max(t0);
                ray_t.max = ray_t.max.min(t1);
            } else {
                ray_t.min = ray_t.min.max(t1);
                ray_t.max = ray_t.max.min(t0);
            }

            if ray_t.max <= ray_t.min {
                return false;
            }
        }
        true
    }
}

impl AABB {
    pub fn offset(self, offset: Vec3) -> AABB {
        AABB {
            x: Interval {
                min: self.x.min + offset.x(),
                max: self.x.max + offset.x(),
            },
            y: Interval {
                min: self.y.min + offset.y(),
                max: self.y.max + offset.y(),
            },
            z: Interval {
                min: self.z.min + offset.z(),
                max: self.z.max + offset.z(),
            },
        }
    }
}

impl Add<Vec3> for AABB {
    type Output = AABB;
    fn add(self, offset: Vec3) -> AABB {
        self.offset(offset)
    }
}
