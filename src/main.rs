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
use crate::volume::Volume;
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
mod volume;

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

#[allow(dead_code)]
fn cornell_smoke() -> Box<dyn Hittable> {
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
        intensity: 7.0,
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
        Point::new(113.0, 554.0, 127.0),
        Vec3::new(330.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 305.0),
        light,
    )));

    world.add(Box::new(Quad::new(
        Point::new(0.0, 0.0, 0.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        white.clone(),
    )));

    world.add(Box::new(Quad::new(
        Point::new(0.0, 555.0, 0.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
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
    let cube = Volume {
        boundary: cube,
        material: Material::Isotropic {
            tex: Texture::Constant {
                color: Color::new(0.0, 0.0, 0.0),
            },
        },
        density: 0.01,
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
    let cube = Volume {
        boundary: cube,
        material: Material::Isotropic {
            tex: Texture::Constant {
                color: Color::new(1.0, 1.0, 1.0),
            },
        },
        density: 0.01,
    };
    world.add(Box::new(cube));

    Box::new(world)
}

#[allow(dead_code)]
fn final_scene() -> Box<dyn Hittable> {
    let mut world: Vec<Box<dyn Hittable>> = Vec::new();

    // make ground
    let mut boxes: Vec<Box<dyn Hittable>> = Vec::new();
    let ground = Material::Lambertian {
        albedo: Texture::Constant {
            color: Color::new(0.48, 0.83, 0.53),
        },
    };
    let boxes_per_side = 20;
    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let w = 100.0;
            let x0 = -1000.0 + i as f64 * w;
            let z0 = -1000.0 + j as f64 * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = random_double_range(1.0, 101.0);
            let z1 = z0 + w;

            boxes.push(Box::new(Cube::new(
                Point::new(x0, y0, z0),
                Point::new(x1, y1, z1),
                ground.clone(),
            )));
        }
    }
    let boxes = Box::new(Bvh::new(boxes, 0.0, 1.0));
    world.push(boxes);

    // make light
    let light = Material::DiffuseLight {
        emitter: Texture::Constant {
            color: Color::new(1.0, 1.0, 1.0),
        },
        intensity: 7.0,
    };
    world.push(Box::new(Quad::new(
        Point::new(123.0, 554.0, 147.0),
        Vec3::new(300.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 265.0),
        light,
    )));

    // moving sphere
    let moving_sphere = Material::Lambertian {
        albedo: Texture::Constant {
            color: Color::new(0.7, 0.3, 0.1),
        },
    };
    let center1 = Point::new(400.0, 400.0, 200.0);
    let center2 = center1 + Point::new(30.0, 0.0, 0.0);
    world.push(Box::new(MovingSphere::new(
        center1,
        center2,
        0.0,
        1.0,
        50.0,
        moving_sphere,
    )));

    // glass sphere
    world.push(Box::new(Sphere::new(
        Point::new(260.0, 150.0, 145.0),
        50.0,
        Material::Dielectric {
            refraction_index: 1.5,
        },
    )));

    // metal sphere
    world.push(Box::new(Sphere::new(
        Point::new(0.0, 150.0, 145.0),
        50.0,
        Material::Metal {
            albedo: Color::new(0.8, 0.8, 0.9),
            fuzz: 1.0,
        },
    )));

    // blue glass sphere
    world.push(Box::new(Sphere::new(
        Point::new(360.0, 150.0, 145.0),
        70.0,
        Material::Dielectric {
            refraction_index: 1.5,
        },
    )));
    world.push(Box::new(Volume {
        density: 0.2,
        material: Material::Isotropic {
            tex: Texture::Constant {
                color: Color::new(0.2, 0.4, 0.9),
            },
        },
        boundary: Sphere::new(
            Point::new(360.0, 150.0, 145.0),
            70.0,
            Material::Dielectric {
                refraction_index: 1.5,
            },
        ),
    }));

    // world fog
    world.push(Box::new(Volume {
        density: 0.0001,
        material: Material::Lambertian {
            albedo: Texture::Constant {
                color: Color::new(1.0, 1.0, 1.0),
            },
        },
        boundary: Sphere::new(
            Point::new(0.0, 0.0, 0.0),
            5000.0,
            Material::Dielectric {
                // does not matter
                refraction_index: 1.5,
            },
        ),
    }));

    // image sphere
    world.push(Box::new(Translate {
        offset: Point::new(400.0, 200.0, 400.0),
        shape: rotate_y(
            90.0,
            Sphere::new(
                Point::new(0.0, 0.0, 0.0),
                100.0,
                Material::Lambertian {
                    albedo: Texture::image("assets/matt.png"),
                },
            ),
        ),
    }));

    // perlin sphere
    world.push(Box::new(Sphere::new(
        Point::new(220.0, 280.0, 300.0),
        80.0,
        Material::Lambertian {
            albedo: Texture::Noise(Noise {
                noise_gen: Perlin::new(),
                scale: 0.2,
                noise_type: NoiseType::Perlin,
            }),
        },
    )));

    const SPHERES: usize = 1000;
    let white = Material::Lambertian {
        albedo: Texture::Constant {
            color: Color::from(0.73),
        },
    };
    let spheres = (0..SPHERES)
        .map(|_| {
            Box::new(Translate {
                offset: 165.0 * Vec3::random(),
                shape: Sphere::new(Point::new(0.0, 0.0, 0.0), 10.0, white.clone()),
            }) as Box<dyn Hittable>
        })
        .collect();
    let bvh = Bvh::new(spheres, 0.0, 1.0);

    let translate = Translate {
        offset: Vec3::new(-100.0, 270.0, 395.0),
        shape: rotate_y(15.0, bvh),
    };
    world.push(Box::new(translate));

    Box::new(Bvh::new(world, 0.0, 1.0))
}

fn main() {
    let mut out = stdout().lock();

    // let R = f64::cos(PI / 4.0);
    // let aspect_ratio = 16.0 / 9.0;
    let aspect_ratio = 1.0;
    let image_width = 800;
    let samples = 10000;
    let max_depth = 40;
    let vfov = 40.0;
    let look_from = Point::new(478.0, 278.0, -600.0);
    let look_at = Point::new(278.0, 278.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let defocus_angle = 0.;
    let focus_dist = 10.0;
    let background = Color::new(0.0, 0.0, 0.0);

    let world = final_scene();

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
