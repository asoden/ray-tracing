use std::range::Range;

use crate::{
    aabb::{Aabb, surrounding_box},
    hittable::{HitRecord, Hittable},
    vec3::Axis::{self, X, Y, Z},
};

pub struct Bvh {
    bounding_box: Aabb,
    contents: BvhContents,
}

enum BvhContents {
    Node { left: Box<Bvh>, right: Box<Bvh> },
    Leaf(Box<dyn Hittable>),
}

impl Bvh {
    pub fn new(mut objects: Vec<Box<dyn Hittable>>, time0: f64, time1: f64) -> Self {
        fn axis_range(objects: &[Box<dyn Hittable>], interval: Range<f64>, axis: Axis) -> f64 {
            let range = objects.iter().fold(f64::MAX..f64::MIN, |range, object| {
                let bb = object.bounding_box(interval.start, interval.end);
                let min = bb.min[axis].min(bb.max[axis]);
                let max = bb.min[axis].max(bb.max[axis]);
                range.start.min(min)..range.end.max(max)
            });
            range.end - range.start
        }

        // Find the axis that has the greatest range for this collection of objects
        let axis = {
            let mut ranges = [
                (X, axis_range(&objects, Range::from(time0..time1), Axis::X)),
                (Y, axis_range(&objects, Range::from(time0..time1), Axis::Y)),
                (Z, axis_range(&objects, Range::from(time0..time1), Axis::Z)),
            ];
            // reverse comparison to sort descending
            ranges.sort_unstable_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
            ranges[0].0
        };

        objects.sort_unstable_by(|a, b| {
            let abb = a.bounding_box(time0, time1);
            let bbb = b.bounding_box(time0, time1);
            let av = abb.min[axis] + abb.max[axis];
            let bv = bbb.min[axis] + bbb.max[axis];
            av.partial_cmp(&bv).unwrap()
        });

        match objects.len() {
            0 => panic!("No objects in scene"),
            1 => Bvh {
                bounding_box: objects[0].bounding_box(time0, time1),
                contents: BvhContents::Leaf(objects.pop().unwrap()),
            },
            _ => {
                let right = Box::new(Bvh::new(
                    objects.drain(objects.len() / 2..).collect(),
                    time0,
                    time1,
                ));
                let left = Box::new(Bvh::new(objects, time0, time1));
                Bvh {
                    bounding_box: surrounding_box(&left.bounding_box, &right.bounding_box),
                    contents: BvhContents::Node { left, right },
                }
            }
        }
    }
}

impl Hittable for Bvh {
    fn hit(
        &self,
        ray: &crate::ray::Ray,
        ray_tmin: f64,
        mut ray_tmax: f64,
    ) -> Option<HitRecord<'_>> {
        if self.bounding_box.hit(ray, ray_tmin..ray_tmax) {
            match &self.contents {
                BvhContents::Leaf(hittable) => hittable.hit(ray, ray_tmin, ray_tmax),
                BvhContents::Node { left, right } => {
                    let left = left.hit(ray, ray_tmin, ray_tmax);
                    if let Some(left) = &left {
                        ray_tmax = left.t;
                    }
                    let right = right.hit(ray, ray_tmin, ray_tmax);
                    if right.is_some() { right } else { left }
                }
            }
        } else {
            None
        }
    }

    fn bounding_box(&self, _t0: f64, _t1: f64) -> Aabb {
        self.bounding_box
    }
}
