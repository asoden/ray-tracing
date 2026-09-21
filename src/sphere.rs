use crate::{
    aabb::{Aabb, surrounding_box},
    hittable::{HitRecord, Hittable},
    material::Material,
    vec3::{Point, Vec3},
};
fn get_sphere_uv(p: &Vec3) -> (f64, f64) {
    let phi = p.z.atan2(p.x);
    let theta = p.y.asin();
    let u = 1.0 - (phi + std::f64::consts::PI) / (2.0 * std::f64::consts::PI);
    let v = (theta + std::f64::consts::FRAC_PI_2) / std::f64::consts::PI;
    (u, v)
}

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub material: Material,
}

impl Sphere {
    pub fn new(center: Point, radius: f64, m: Material) -> Self {
        Self {
            center,
            radius: f64::max(0.0, radius),
            material: m,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &crate::ray::Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord<'_>> {
        let oc = self.center - ray.orig;
        let a = ray.direction.length_squared();
        let h = ray.direction.dot(&oc);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (h - sqrtd) / a;
        if root <= ray_tmin || ray_tmax <= root {
            root = (h + sqrtd) / a;
            if root <= ray_tmin || ray_tmax <= root {
                return None;
            }
        }
        let (u, v) = get_sphere_uv(&Default::default());

        let mut record = HitRecord {
            t: root,
            u,
            v,
            p: ray.at(root),
            material: &self.material,
            normal: Default::default(),
            front_face: Default::default(),
        };
        let outward_normal = (record.p - self.center) / self.radius;
        let (u, v) = get_sphere_uv(&outward_normal);
        record.set_face_normal(ray, &outward_normal);
        record.u = u;
        record.v = v;

        Some(record)
    }

    fn bounding_box(&self, _t0: f64, _t1: f64) -> Aabb {
        Aabb {
            min: self.center - Vec3::from(self.radius),
            max: self.center + Vec3::from(self.radius),
        }
    }
}

pub struct MovingSphere {
    pub center0: Vec3,
    pub center1: Vec3,
    pub time0: f64,
    pub time1: f64,
    pub radius: f64,
    pub material: Material,
}

impl MovingSphere {
    pub fn new(
        center0: Vec3,
        center1: Vec3,
        time0: f64,
        time1: f64,
        radius: f64,
        m: Material,
    ) -> Self {
        Self {
            center0,
            center1,
            time0,
            time1,
            radius: f64::max(0.0, radius),
            material: m,
        }
    }

    pub fn center(&self, time: f64) -> Vec3 {
        self.center0
            + ((time - self.time0) / (self.time1 - self.time0)) * (self.center1 - self.center0)
    }
}

impl Hittable for MovingSphere {
    fn hit(&self, ray: &crate::ray::Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord<'_>> {
        let current_center = self.center(ray.time);
        let oc = current_center - ray.orig;
        let a = ray.direction.length_squared();
        let h = ray.direction.dot(&oc);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (h - sqrtd) / a;
        if root <= ray_tmin || ray_tmax <= root {
            root = (h + sqrtd) / a;
            if root <= ray_tmin || ray_tmax <= root {
                return None;
            }
        }

        let (u, v) = get_sphere_uv(&Default::default());

        let mut record = HitRecord {
            t: root,
            u,
            v,
            p: ray.at(root),
            material: &self.material,
            normal: Default::default(),
            front_face: Default::default(),
        };
        let outward_normal = (record.p - current_center) / self.radius;
        let (u, v) = get_sphere_uv(&outward_normal);
        record.set_face_normal(ray, &outward_normal);
        record.u = u;
        record.v = v;

        Some(record)
    }

    fn bounding_box(&self, t0: f64, t1: f64) -> Aabb {
        let radius = Vec3::new(self.radius, self.radius, self.radius);
        let min0 = self.center(t0) - radius;
        let max0 = self.center(t0) + radius;
        let min1 = self.center(t1) - radius;
        let max1 = self.center(t1) + radius;
        let aabb0 = Aabb {
            min: min0,
            max: max0,
        };
        let aabb1 = Aabb {
            min: min1,
            max: max1,
        };
        surrounding_box(&aabb0, &aabb1)
    }
}
