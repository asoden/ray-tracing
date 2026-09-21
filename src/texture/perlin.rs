use crate::{
    common::{random_double_range, random_usize_range},
    vec3::{Point, Vec3},
};

const POINT_COUNT: usize = 256;

fn perlin_generate() -> [usize; POINT_COUNT] {
    let mut points = [0; POINT_COUNT];

    for i in 0..POINT_COUNT {
        points[i] = i;
    }

    permute(&mut points, POINT_COUNT);

    points
}

fn permute(p: &mut [usize], n: usize) {
    for i in (1..n).rev() {
        p.swap(i, random_usize_range(0, i));
    }
}

fn trilinear_interop(c: &mut [[[Vec3; 2]; 2]; 2], uvw: (f64, f64, f64)) -> f64 {
    let (u, v, w) = (
        (uvw.0 * uvw.0) * (3.0 - 2.0 * uvw.0),
        (uvw.1 * uvw.1) * (3.0 - 2.0 * uvw.1),
        (uvw.2 * uvw.2) * (3.0 - 2.0 * uvw.2),
    );

    let mut acc = 0.0;

    for (i, ci) in c.iter().enumerate() {
        for (j, cj) in ci.iter().enumerate() {
            for (k, ck) in cj.iter().enumerate() {
                let fi = i as f64;
                let fj = j as f64;
                let fk = k as f64;

                let weight_vec = Vec3::new(u - fi, v - fj, w - fk);

                acc += (fi * u + (1.0 - fi) * (1.0 - u))
                    * (fj * v + (1.0 - fj) * (1.0 - v))
                    * (fk * w + (1.0 - fk) * (1.0 - w))
                    * weight_vec.dot(ck);
            }
        }
    }

    acc
}

#[derive(Debug, Clone)]
pub struct Perlin {
    random_vecs: [Vec3; POINT_COUNT],
    perm_x: [usize; POINT_COUNT],
    perm_y: [usize; POINT_COUNT],
    perm_z: [usize; POINT_COUNT],
}

impl Perlin {
    pub fn new() -> Self {
        let mut random_vecs = [Vec3::new(0.0, 0.0, 0.0); POINT_COUNT];

        for elt in random_vecs.iter_mut() {
            *elt = Vec3::new(
                random_double_range(-1.0, 1.0),
                random_double_range(-1.0, 1.0),
                random_double_range(-1.0, 1.0),
            );
        }

        Self {
            random_vecs,
            perm_x: perlin_generate(),
            perm_y: perlin_generate(),
            perm_z: perlin_generate(),
        }
    }

    pub fn noise(&self, point: &Point) -> f64 {
        let uvw = (
            point.x - point.x.floor(),
            point.y - point.y.floor(),
            point.z - point.z.floor(),
        );

        let ijk = (
            point.x.floor() as i32,
            point.y.floor() as i32,
            point.z.floor() as i32,
        );

        let c = &mut [[[Vec3::new(0.0, 0., 0.); 2]; 2]; 2];

        for (di, ci) in c.iter_mut().enumerate() {
            for (dj, cj) in ci.iter_mut().enumerate() {
                for (dk, ck) in cj.iter_mut().enumerate() {
                    *ck = self.random_vecs[self.perm_x[((ijk.0 + di as i32) & 255) as usize]
                        ^ self.perm_y[((ijk.1 + dj as i32) & 255) as usize]
                        ^ self.perm_z[((ijk.2 + dk as i32) & 255) as usize]];
                }
            }
        }

        trilinear_interop(c, uvw)
    }

    pub fn turbulence(&self, point: &Point, depth: i32) -> f64 {
        let mut acc = 0.0;
        let mut temp_p = *point;
        let mut weight = 1.0;

        for _ in 0..depth {
            acc += weight * self.noise(&temp_p);
            weight *= 0.5;
            temp_p *= 2.0;
        }

        acc.abs()
    }
}
