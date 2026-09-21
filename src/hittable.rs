use crate::{
    aabb::Aabb,
    material::Material,
    ray::Ray,
    vec3::{Point, Vec3},
};

pub trait Hittable: Send + Sync {
    fn hit(&self, ray: &Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord<'_>>;
    fn bounding_box(&self, t0: f64, t1: f64) -> Aabb;
}

#[derive(Clone)]
pub struct HitRecord<'m> {
    pub p: Point,
    pub normal: Vec3,
    pub material: &'m Material,
    pub t: f64,
    pub u: f64,
    pub v: f64,
    pub front_face: bool,
}

impl HitRecord<'_> {
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vec3) {
        // Sets the hit record normal vector.
        // NOTE: the parameter `outward_normal` is assumed to have unit length.

        self.front_face = ray.direction.dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -*outward_normal
        };
    }
}
