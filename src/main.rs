use std::io::stdout;

use crate::bvh::Bvh;
use crate::common::{random_double, random_double_range};
use crate::cube::Cube;
use crate::hittable::Hittable;
use crate::hittable_list::HittableList;
use crate::material::Material;
use crate::quad::Quad;
use crate::rotate::rotate_y;
use crate::sphere::MovingSphere;
use crate::texture::Texture;
use crate::texture::noise::{Noise, NoiseType};
use crate::texture::perlin::Perlin;
use crate::translate::Translate;
use crate::vec3::Vec3;
use crate::{
    camera::Camera,
    sphere::Sphere,
    vec3::{Color, Point},
};

mod aabb;
mod bvh;
mod camera;
mod common;
mod cube;
mod hittable;
mod hittable_list;
mod material;
mod quad;
mod ray;
mod rotate;
mod sphere;
mod texture;
mod translate;
mod vec3;

#[allow(dead_code)]
fn random_balls() -> Box<dyn Hittable> {
    // World
    let mut world: Vec<Box<dyn Hittable>> = Vec::new();
    let material_ground = Material::Lambertian {
        albedo: Texture::color(Color::new(0.5, 0.5, 0.5)),
    };
    world.push(Box::new(Sphere::new(
        Point::new(0.0, -1000.0, 0.0),
        1000.0,
        material_ground,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_double();
            let center = Point::new(
                a as f64 + 0.9 * random_double(),
                0.2,
                b as f64 + 0.9 * random_double(),
            );

            if (center - Point::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::random() * Color::random();
                    let material = Material::Lambertian {
                        albedo: Texture::color(albedo),
                    };
                    world.push(Box::new(Sphere::new(center, 0.2, material)));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random();
                    let fuzz = random_double_range(0.0, 0.5);
                    let material = Material::Metal { albedo, fuzz };
                    world.push(Box::new(Sphere::new(center, 0.2, material)));
                } else {
                    // glass
                    let material = Material::Dielectric {
                        refraction_index: 1.5,
                    };
                    world.push(Box::new(Sphere::new(center, 0.2, material)));
                }
            }
        }
    }

    let material1 = Material::Dielectric {
        refraction_index: 1.5,
    };
    world.push(Box::new(Sphere::new(
        Point::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = Material::Lambertian {
        albedo: Texture::color(Color::new(0.4, 0.2, 0.1)),
    };
    world.push(Box::new(Sphere::new(
        Point::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Material::Metal {
        albedo: Color::new(0.7, 0.6, 0.5),
        fuzz: 0.0,
    };
    world.push(Box::new(Sphere::new(
        Point::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    Box::new(Bvh::new(world, 0.0, 1.0))
}

#[allow(dead_code)]
fn random_bouncy_balls() -> Box<dyn Hittable> {
    // World
    let mut world: Vec<Box<dyn Hittable>> = Vec::new();
    let material_ground = Material::Lambertian {
        albedo: Texture::checkered(
            Texture::color(Color::new(0.2, 0.3, 0.1)),
            Texture::color(Color::new(0.9, 0.9, 0.9)),
        ),
    };
    world.push(Box::new(Sphere::new(
        Point::new(0.0, -1000.0, 0.0),
        1000.0,
        material_ground,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_double();
            let center = Point::new(
                a as f64 + 0.9 * random_double(),
                0.2,
                b as f64 + 0.9 * random_double(),
            );

            if (center - Point::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::random() * Color::random();
                    let material = Material::Lambertian {
                        albedo: Texture::color(albedo),
                    };
                    world.push(Box::new(MovingSphere::new(
                        center,
                        center + Vec3::new(0., random_double_range(0., 0.5), 0.),
                        0.,
                        1.,
                        0.2,
                        material,
                    )));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random();
                    let fuzz = random_double_range(0.0, 0.5);
                    let material = Material::Metal { albedo, fuzz };
                    world.push(Box::new(Sphere::new(center, 0.2, material)));
                } else {
                    // glass
                    let material = Material::Dielectric {
                        refraction_index: 1.5,
                    };
                    world.push(Box::new(Sphere::new(center, 0.2, material)));
                }
            }
        }
    }

    let material1 = Material::Dielectric {
        refraction_index: 1.5,
    };
    world.push(Box::new(Sphere::new(
        Point::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = Material::Lambertian {
        albedo: Texture::color(Color::new(0.4, 0.2, 0.1)),
    };
    world.push(Box::new(Sphere::new(
        Point::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Material::Metal {
        albedo: Color::new(0.7, 0.6, 0.5),
        fuzz: 0.0,
    };
    world.push(Box::new(Sphere::new(
        Point::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    Box::new(Bvh::new(world, 0., 1.))
}

#[allow(dead_code)]
fn checkered_spheres() -> Box<dyn Hittable> {
    let mut world = HittableList::default();
    let checkered = Material::Lambertian {
        albedo: Texture::checkered(
            Texture::color(Color::new(0.2, 0.3, 0.1)),
            Texture::color(Color::new(0.9, 0.9, 0.9)),
        ),
    };

    world.add(Box::new(Sphere::new(
        Vec3::new(0.0, -10.0, 0.0),
        10.0,
        checkered.clone(),
    )));
    world.add(Box::new(Sphere::new(
        Vec3::new(0.0, 10.0, 0.0),
        10.0,
        checkered,
    )));

    Box::new(world)
}

#[allow(dead_code)]
fn image_sphere() -> Box<dyn Hittable> {
    let mut world = HittableList::default();
    let image = Material::Lambertian {
        albedo: Texture::image("assets/matt.png"),
    };

    world.add(Box::new(Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, image)));

    Box::new(world)
}

#[allow(dead_code)]
fn perlin_shperes(noise_type: NoiseType) -> Box<dyn Hittable> {
    let mut world = HittableList::default();
    let material = Material::Lambertian {
        albedo: Texture::Noise(Noise {
            noise_gen: Perlin::new(),
            scale: 4.0,
            noise_type,
        }),
    };

    world.add(Box::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        material.clone(),
    )));
    world.add(Box::new(Sphere::new(
        Vec3::new(0.0, 2.0, 0.0),
        2.0,
        material,
    )));

    Box::new(world)
}

#[allow(dead_code)]
fn quads() -> Box<dyn Hittable> {
    let mut world = HittableList::default();

    let left = Material::Metal {
        albedo: Color::new(1.0, 0.2, 0.2),
        fuzz: 0.0,
    };
    let back = Material::Lambertian {
        albedo: Texture::image("assets/jeren.png"),
    };
    let right = Material::Metal {
        albedo: Color::new(0.2, 0.2, 1.0),
        fuzz: 0.0,
    };
    let upper = Material::Metal {
        albedo: Color::new(1.0, 0.5, 0.0),
        fuzz: 0.0,
    };
    let lower = Material::Metal {
        albedo: Color::new(0.2, 0.8, 0.8),
        fuzz: 0.0,
    };

    world.add(Box::new(Quad::new(
        Point::new(-3.0, -2.0, 5.0),
        Vec3::new(0.0, 0.0, -4.0),
        Vec3::new(0.0, 4.0, 0.0),
        left,
    )));
    world.add(Box::new(Quad::new(
        Point::new(-2.0, -2.0, 0.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 4.0, 0.0),
        back,
    )));
    world.add(Box::new(Quad::new(
        Point::new(3.0, -2.0, 1.0),
        Vec3::new(0.0, 0.0, 4.0),
        Vec3::new(0.0, 4.0, 0.0),
        right,
    )));
    world.add(Box::new(Quad::new(
        Point::new(-2.0, 3.0, 1.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 4.0),
        upper,
    )));
    world.add(Box::new(Quad::new(
        Point::new(-2.0, -3.0, 5.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -4.0),
        lower,
    )));

    Box::new(world)
}

#[allow(dead_code)]
fn simple_light() -> Box<dyn Hittable> {
    let mut world = HittableList::default();

    let perlin_texture = Material::Lambertian {
        albedo: Texture::Noise(Noise {
            noise_gen: Perlin::new(),
            scale: 4.0,
            noise_type: NoiseType::Turbulence,
        }),
    };

    world.add(Box::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        perlin_texture.clone(),
    )));
    world.add(Box::new(Sphere::new(
        Vec3::new(0.0, 2.0, 0.0),
        2.0,
        perlin_texture,
    )));

    let diff_light = Material::DiffuseLight {
        emitter: Texture::Constant {
            color: Color::new(1.0, 1.0, 1.0),
        },
        intensity: 4.0,
    };

    world.add(Box::new(Sphere::new(
        Vec3::new(0.0, 7.0, 0.0),
        2.0,
        diff_light.clone(),
    )));

    world.add(Box::new(Quad::new(
        Point::new(3.0, 1.0, -2.0),
        Vec3::new(2.0, 0.0, 0.0),
        Vec3::new(0.0, 2.0, 0.0),
        diff_light,
    )));

    Box::new(world)
}

#[allow(dead_code)]
fn cornell_box() -> Box<dyn Hittable> {
    let mut world = HittableList::default();

    let red = Material::Lambertian {
        albedo: Texture::Constant {
            color: Color::new(0.65, 0.05, 0.05),
        },
    };
    let white = Material::Lambertian {
        albedo: Texture::Constant {
            color: Color::new(0.73, 0.73, 0.73),
        },
    };
    let green = Material::Lambertian {
        albedo: Texture::Constant {
            color: Color::new(0.12, 0.45, 0.15),
        },
    };
    let light = Material::DiffuseLight {
        emitter: Texture::Constant {
            color: Color::new(1.0, 1.0, 1.0),
        },
        intensity: 15.0,
    };

    world.add(Box::new(Quad::new(
        Point::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        green,
    )));

    world.add(Box::new(Quad::new(
        Point::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        red,
    )));

    world.add(Box::new(Quad::new(
        Point::new(343.0, 554.0, 332.0),
        Vec3::new(-130.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -105.0),
        light,
    )));

    world.add(Box::new(Quad::new(
        Point::new(0.0, 0.0, 0.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        white.clone(),
    )));

    world.add(Box::new(Quad::new(
        Point::new(555.0, 555.0, 555.0),
        Vec3::new(-555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -555.0),
        white.clone(),
    )));

    world.add(Box::new(Quad::new(
        Point::new(0.0, 0.0, 555.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        white.clone(),
    )));

    let cube = Cube::new(
        Point::new(0.0, 0.0, 0.0),
        Point::new(165.0, 330.0, 165.0),
        white.clone(),
    );
    let cube = Translate {
        offset: Vec3::new(265.0, 0.0, 295.0),
        shape: rotate_y(15.0, cube),
    };
    world.add(Box::new(cube));

    let cube = Cube::new(
        Point::new(0.0, 0.0, 0.0),
        Point::new(165.0, 165.0, 165.0),
        white.clone(),
    );
    let cube = Translate {
        offset: Vec3::new(130.0, 0.0, 65.0),
        shape: rotate_y(-18.0, cube),
    };
    world.add(Box::new(cube));

    Box::new(world)
}

fn main() {
    let mut out = stdout().lock();

    // let R = f64::cos(PI / 4.0);
    // let aspect_ratio = 16.0 / 9.0;
    let aspect_ratio = 1.0;
    let image_width = 600;
    let samples = 10000;
    let max_depth = 50;
    let vfov = 40.0;
    let look_from = Point::new(278.0, 278.0, -800.0);
    let look_at = Point::new(278.0, 278.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let defocus_angle = 0.;
    let focus_dist = 10.0;
    let background = Color::new(0.0, 0.0, 0.0);

    let world = cornell_box();

    let mut cam = Camera::new(
        aspect_ratio,
        image_width,
        samples,
        max_depth,
        vfov,
        look_from,
        look_at,
        vup,
        defocus_angle,
        focus_dist,
        background,
    );

    cam.render(&mut out, world);
}
