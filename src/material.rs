use crate::{
    common::random_double,
    hittable::HitRecord,
    ray::Ray,
    texture::Texture,
    vec3::{Color, Vec3},
};

pub struct ScatterRecord {
    pub attenuation: Color,
    pub scattered: Ray,
}

#[derive(Clone)]
pub enum Material {
    Lambertian { albedo: Texture },
    Metal { albedo: Color, fuzz: f64 },
    Dielectric { refraction_index: f64 },
}

impl Material {
    pub fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        match self {
            Material::Lambertian { albedo } => {
                let mut scatter_direction = rec.normal + Vec3::random_unit_vector();

                if scatter_direction.near_zero() {
                    scatter_direction = rec.normal;
                }

                Some(ScatterRecord {
                    attenuation: albedo.value(rec.u, rec.v, &rec.p),
                    scattered: Ray::new(rec.p, scatter_direction, r_in.time),
                })
            }
            Material::Metal { albedo, fuzz } => {
                let reflected = Vec3::reflect(&r_in.direction, &rec.normal);
                let scattered = Ray::new(
                    rec.p,
                    reflected + fuzz * Vec3::random_unit_vector(),
                    r_in.time,
                );

                if scattered.direction.dot(&rec.normal) > 0. {
                    Some(ScatterRecord {
                        attenuation: *albedo,
                        scattered: Ray::new(rec.p, reflected, r_in.time),
                    })
                } else {
                    None
                }
            }
            Material::Dielectric { refraction_index } => {
                let ri = if rec.front_face {
                    1.0 / refraction_index
                } else {
                    *refraction_index
                };

                let unit_direction = r_in.direction.to_unit();
                let cos_theta = f64::min((-unit_direction).dot(&rec.normal), 1.0);
                let sin_theta = f64::sqrt(1.0 - cos_theta * cos_theta);

                let cannot_refract = ri * sin_theta > 1.0;

                let direction: Vec3 =
                    if cannot_refract || reflectance(cos_theta, ri) > random_double() {
                        Vec3::reflect(&unit_direction, &rec.normal)
                    } else {
                        Vec3::refract(&unit_direction, &rec.normal, ri)
                    };

                Some(ScatterRecord {
                    attenuation: Color::new(1.0, 1.0, 1.0),
                    scattered: Ray::new(rec.p, direction, r_in.time),
                })
            }
        }
    }
}

fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
    // Use Schlick's apporximation for reflectance.
    let mut r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
    r0 = r0 * r0;

    r0 + (1.0 - r0) * f64::powi(1.0 - cosine, 5)
}
