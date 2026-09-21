use crate::vec3::{Point, Vec3};

#[derive(Clone, Copy, Debug, Default)]
pub struct Ray {
    pub orig: Point,
    pub direction: Vec3,
    pub time: f64,
}

impl Ray {
    pub fn new(origin: Point, direction: Vec3, time: f64) -> Self {
        Self {
            orig: origin,
            direction,
            time,
        }
    }

    pub fn at(self, t: f64) -> Point {
        self.orig + t * self.direction
    }
}
