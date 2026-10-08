use crate::{
    aabb::Aabb,
    hittable::{HitRecord, Hittable},
    ray::Ray,
    vec3::Vec3,
};

pub struct Translate<S> {
    pub offset: Vec3,
    pub shape: S,
}

impl<S: Hittable> Hittable for Translate<S> {
    fn hit(&self, ray: &Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord<'_>> {
        let t_ray = Ray::new(ray.orig - self.offset, ray.direction, ray.time);
        self.shape
            .hit(&t_ray, ray_tmin, ray_tmax)
            .map(|hit| HitRecord {
                p: hit.p + self.offset,
                ..hit
            })
    }

    fn bounding_box(&self, t0: f64, t1: f64) -> Aabb {
        let b = self.shape.bounding_box(t0, t1);
        Aabb {
            min: b.min + self.offset,
            max: b.max + self.offset,
        }
    }
}
