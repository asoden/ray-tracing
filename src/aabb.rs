use std::ops::Range;

use crate::{Vec3, ray::Ray};

#[derive(Clone, Copy, Debug, Default)]
pub struct Aabb {
    pub min: Vec3,
    pub max: Vec3,
}

pub fn surrounding_box(box0: &Aabb, box1: &Aabb) -> Aabb {
    let min = Vec3::new(
        f64::min(box0.min.x, box1.min.x),
        f64::min(box0.min.y, box1.min.y),
        f64::min(box0.min.z, box1.min.z),
    );
    let max = Vec3::new(
        f64::max(box0.max.x, box1.max.x),
        f64::max(box0.max.y, box1.max.y),
        f64::max(box0.max.z, box1.max.z),
    );
    Aabb { min, max }
}

impl Aabb {
    pub fn hit(&self, ray: &Ray, interval: Range<f64>) -> bool {
        let inverse_direction = ray.direction.map(|x| 1. / x);
        let t0 = (self.min - ray.orig) * inverse_direction;
        let t1 = (self.max - ray.orig) * inverse_direction;
        let (t0, t1) = (
            inverse_direction.zip_with3(t0, t1, |i, a, b| if i < 0. { b } else { a }),
            inverse_direction.zip_with3(t0, t1, |i, a, b| if i < 0. { a } else { b }),
        );
        let start = interval.start.max(t0.reduce(f64::max));
        let end = interval.end.min(t1.reduce(f64::min));
        end > start
    }
}
