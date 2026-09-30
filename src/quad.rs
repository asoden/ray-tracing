use crate::{
    aabb::{self, Aabb},
    hittable::{HitRecord, Hittable},
    material::Material,
    ray::Ray,
    vec3::{Point, Vec3},
};

pub struct Quad {
    q: Point,
    u: Vec3,
    v: Vec3,
    material: Material,
    w: Vec3,
    normal: Vec3,
    plane_d: f64,
}

impl Quad {
    pub fn new(q: Point, u: Vec3, v: Vec3, material: Material) -> Self {
        let n = u.cross(&v);
        let normal = n.to_unit();
        let plane_d = normal.dot(&q);
        let w = n / n.dot(&n);
        Self {
            q,
            u,
            v,
            material,
            w,
            normal,
            plane_d,
        }
    }
}

impl Hittable for Quad {
    fn hit(&self, ray: &Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord<'_>> {
        let denom = self.normal.dot(&ray.direction);

        // no hit if the ray is parallel to the plane
        if denom.abs() < 1e-8 {
            return None;
        }

        let t = (self.plane_d - self.normal.dot(&ray.orig)) / denom;
        if t < ray_tmin || t > ray_tmax {
            return None;
        }

        let intersection = ray.at(t);
        let planar_hit_point = intersection - self.q;

        let alpha = self.w.dot(&planar_hit_point.cross(&self.v));
        let beta = self.w.dot(&self.u.cross(&planar_hit_point));

        if alpha < 0.0 || alpha > 1.0 || beta < 0.0 || beta > 1.0 {
            return None;
        }

        let mut rec = HitRecord {
            t,
            p: intersection,
            material: &self.material,
            u: alpha,
            v: beta,
            normal: Default::default(),
            front_face: Default::default(),
        };

        rec.set_face_normal(ray, &self.normal);

        Some(rec)
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
