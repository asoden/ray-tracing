use crate::{
    aabb::Aabb,
    hittable::{HitRecord, Hittable},
    material::Material,
    quad::Quad,
    ray::Ray,
    vec3::Point,
};

pub struct Cube {
    min: Point,
    max: Point,
    sides: [Quad; 6],
}

impl Cube {
    pub fn new(min: Point, max: Point, material: Material) -> Self {
        let dx = Point::new(max.x - min.x, 0.0, 0.0);
        let dy = Point::new(0.0, max.y - min.y, 0.0);
        let dz = Point::new(0.0, 0.0, max.z - min.z);

        Self {
            min,
            max,
            sides: [
                // +X face
                Quad::new(Point::new(max.x, min.y, min.z), dy, dz, material.clone()),
                // -X face
                Quad::new(min, dy, dz, material.clone()),
                // +Y face
                Quad::new(Point::new(min.x, max.y, min.z), dx, dz, material.clone()),
                // -Y face
                Quad::new(min, dx, dz, material.clone()),
                // +Z face
                Quad::new(Point::new(min.x, min.y, max.z), dx, dy, material.clone()),
                // -Z face
                Quad::new(min, dx, dy, material),
            ],
        }
    }
}

impl Hittable for Cube {
    fn hit(&self, ray: &Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord<'_>> {
        self.sides
            .iter()
            .fold(None as Option<HitRecord>, |best, child| {
                let t_best = best.as_ref().map_or(ray_tmax, |h| h.t);
                child.hit(ray, ray_tmin, t_best).or(best)
            })
    }

    fn bounding_box(&self, _: f64, _: f64) -> Aabb {
        Aabb {
            min: self.min,
            max: self.max,
        }
    }
}
