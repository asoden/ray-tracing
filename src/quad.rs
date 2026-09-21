use crate::{
    aabb::{self, Aabb},
    hittable::Hittable,
    material::Material,
    vec3::{Point, Vec3},
};

pub struct Quad {
    q: Point,
    u: Vec3,
    v: Vec3,
    material: Material,
}

impl Quad {
    pub fn new(q: Point, u: Vec3, v: Vec3, material: Material) -> Self {
        Self { q, u, v, material }
    }
}

impl Hittable for Quad {
    fn hit(
        &self,
        ray: &crate::ray::Ray,
        ray_tmin: f64,
        ray_tmax: f64,
    ) -> Option<crate::hittable::HitRecord<'_>> {
        todo!()
    }

    fn bounding_box(&self, _: f64, _: f64) -> Aabb {
        let box_diag1 = Aabb {
            min: self.q,
            max: self.q + self.u + self.v,
        };
        let box_diag2 = Aabb {
            min: self.q + self.u,
            max: self.q + self.v,
        };
        let mut bbox = aabb::surrounding_box(&box_diag1, &box_diag2);

        // adjust the bounding box so that no side is narrower than some delta, padding if necessary
        const DELTA: f64 = 0.0001;

        if bbox.max.x - bbox.min.x < DELTA {
            bbox.min.x -= DELTA;
            bbox.max.x += DELTA;
        }
        if bbox.max.y - bbox.min.y < DELTA {
            bbox.min.y -= DELTA;
            bbox.max.y += DELTA;
        }
        if bbox.max.z - bbox.min.z < DELTA {
            bbox.min.z -= DELTA;
            bbox.max.z += DELTA;
        }
        bbox
    }
}
