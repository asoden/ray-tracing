use crate::{
    aabb::Aabb,
    common::random_double,
    hittable::{HitRecord, Hittable},
    material::Material,
    ray::Ray,
    vec3::Vec3,
};

pub struct Volume<S: Hittable> {
    pub boundary: S,
    pub material: Material,
    pub density: f64,
}

impl<S: Hittable> Hittable for Volume<S> {
    fn hit(&self, ray: &Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord<'_>> {
        if let Some(mut hit1) = self.boundary.hit(ray, f64::MIN, f64::MAX) {
            if let Some(mut hit2) = self.boundary.hit(ray, hit1.t + 0.0001, f64::MAX) {
                hit1.t = hit1.t.max(ray_tmin);
                hit2.t = hit2.t.min(ray_tmax);
                if hit1.t >= hit2.t {
                    return None;
                }

                let distance = (hit2.t - hit1.t) * ray.direction.length();
                let hit_distance = -(1.0 / self.density) * random_double().ln();
                if hit_distance < distance {
                    let t = hit1.t + hit_distance / ray.direction.length();
                    return Some(HitRecord {
                        t,
                        p: ray.at(t),
                        material: &self.material,
                        normal: Vec3::new(1.0, 0.0, 0.0), // Arbitrary
                        front_face: true,                 // Arbitrary,
                        u: hit1.u,
                        v: hit1.v,
                    });
                }
            }
        }
        None
    }

    fn bounding_box(&self, t0: f64, t1: f64) -> Aabb {
        self.boundary.bounding_box(t0, t1)
    }
}
