use core::f64;

use crate::{
    aabb::Aabb,
    hittable::{HitRecord, Hittable},
    ray::Ray,
    vec3::Vec3,
};

pub struct RotateY<S> {
    pub shape: S,
    sin_theta: f64,
    cos_theta: f64,
}

impl<S: Hittable> Hittable for RotateY<S> {
    fn hit(&self, ray: &Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord<'_>> {
        fn rot(p: Vec3, sin_theta: f64, cos_theta: f64) -> Vec3 {
            Vec3::new(
                p.dot(&Vec3::new(cos_theta, 0.0, sin_theta)),
                p.dot(&Vec3 {
                    x: 0.0,
                    y: 1.0,
                    z: 0.0,
                }),
                p.dot(&Vec3 {
                    x: -sin_theta,
                    y: 0.0,
                    z: cos_theta,
                }),
            )
        }
        let rot_ray = Ray {
            orig: rot(ray.orig, -self.sin_theta, self.cos_theta),
            direction: rot(ray.direction, -self.sin_theta, self.cos_theta),
            ..*ray
        };

        self.shape
            .hit(&rot_ray, ray_tmin, ray_tmax)
            .map(|hit| HitRecord {
                p: rot(hit.p, self.sin_theta, self.cos_theta),
                normal: rot(hit.normal, self.sin_theta, self.cos_theta),
                ..hit
            })
    }

    fn bounding_box(&self, t0: f64, t1: f64) -> Aabb {
        fn rot(p: Vec3, sin_theta: f64, cos_theta: f64) -> Vec3 {
            Vec3::new(
                p.dot(&Vec3::new(cos_theta, 0.0, sin_theta)),
                p.dot(&Vec3::new(0.0, 1.0, 0.0)),
                p.dot(&Vec3::new(-sin_theta, 0.0, cos_theta)),
            )
        }

        let (min, max) = self.shape.bounding_box(t0, t1).corners().fold(
            (Vec3::from(f64::MAX), Vec3::from(f64::MIN)),
            |(min, max), c| {
                let rot_c = rot(c, self.sin_theta, self.cos_theta);
                (min.zip_with(rot_c, f64::min), max.zip_with(rot_c, f64::max))
            },
        );
        Aabb { min, max }
    }
}

pub fn rotate_y<S: Hittable>(degrees: f64, shape: S) -> RotateY<S> {
    let radians = degrees * f64::consts::PI / 180.0;
    RotateY {
        shape,
        sin_theta: radians.sin(),
        cos_theta: radians.cos(),
    }
}
