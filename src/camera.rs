use std::io::Write;

use indicatif::ProgressIterator;
use rayon::prelude::*;

use crate::common::{degrees_to_radians, random_double};
use crate::hittable::Hittable;
use crate::ray::Ray;
use crate::vec3::{Color, Point, Vec3};

#[derive(Default)]
pub struct Camera {
    /// Ration of image width over height
    pub aspect_ratio: f64,
    /// Rendered image width in pixel count
    pub image_width: i32,
    /// Count of random samples for each pixel
    pub samples_per_pixel: i32,
    /// Maximum number of ray bounces into scene
    pub max_depth: i32,
    /// Vertical view angle (field of view)
    pub vfov: f64,
    /// Point camera is looking from
    pub look_from: Point,
    /// Point camera is looking at
    pub look_at: Point,
    /// Camera relative "up" direction
    pub vup: Vec3,
    /// Variation of angle of rays through each pixel
    pub defocus_angle: f64,
    /// Distance from camera `look_from` point to plane of perfect focus
    pub focus_dist: f64,
    /// Scene background color
    pub background: Color,
    /// Rendered image height
    image_height: i32,
    /// Color scale factor for a sum of pixel samples
    pixel_samples_scale: f64,
    /// Camera center
    center: Point,
    /// Location of pixel 0, 0
    pixel00_loc: Point,
    /// Offset to pixel to the right
    pixel_delta_u: Vec3,
    /// Offset to pixel below
    pixel_delta_v: Vec3,
    /// Defocus disk horizontal radius
    defocus_disk_u: Vec3,
    /// Defocus disk vertical radius
    defocus_disk_v: Vec3,
}

impl Camera {
    #[allow(dead_code)]
    pub fn default() -> Self {
        Self::new(
            1.0,
            100,
            10,
            10,
            90.,
            Point::new(0., 0., 0.),
            Point::new(0., 0., -1.),
            Vec3::new(0., 1., 0.),
            0.0,
            10.0,
            Color::new(0.0, 0.0, 0.0),
        )
    }

    pub fn new(
        aspect_ratio: f64,
        image_width: i32,
        samples_per_pixel: i32,
        max_depth: i32,
        vfov: f64,
        look_from: Point,
        look_at: Point,
        vup: Vec3,
        defocus_angle: f64,
        focus_dist: f64,
        background: Color,
    ) -> Self {
        Self {
            aspect_ratio,
            image_width,
            samples_per_pixel,
            max_depth,
            vfov,
            look_from,
            look_at,
            vup,
            defocus_angle,
            focus_dist,
            background,
            ..Default::default()
        }
    }

    pub fn render(&mut self, out: &mut impl Write, world: Box<dyn Hittable>) {
        self.initialize();

        println!("P3\n{} {}\n255", self.image_width, self.image_height);

        for j in (0..self.image_height).progress() {
            let pixel_colors = (0..self.image_width)
                .into_par_iter()
                .map(|i| {
                    let mut pixel_color = Color::default();

                    for _ in 0..self.samples_per_pixel {
                        let r = self.get_ray(i, j);
                        pixel_color += self.ray_color(&r, self.max_depth, &*world);
                    }
                    pixel_color
                })
                .collect::<Vec<Color>>();
            for pixel_color in pixel_colors {
                write_color(out, self.pixel_samples_scale * pixel_color);
            }
        }
    }

    fn initialize(&mut self) {
        // Calculate the image height, and ensure that it's at least 1.
        let image_height = ((self.image_width as f64) / self.aspect_ratio) as i32;
        self.image_height = if image_height < 1 { 1 } else { image_height };

        self.pixel_samples_scale = 1.0 / self.samples_per_pixel as f64;

        self.center = self.look_from;

        // let focal_length: f64 = (self.look_from - self.look_at).length();
        let theta = degrees_to_radians(self.vfov);
        let h = f64::tan(theta / 2.0);
        let viewport_height = 2.0 * h * self.focus_dist;
        let viewport_width = viewport_height * (self.image_width as f64 / image_height as f64);

        // Calculate the u, v, w unit basis vectors fo rthe camera coordinate frame.
        let w = (self.look_from - self.look_at).to_unit();
        let u = Vec3::cross(self.vup, &w).to_unit();
        let v = Vec3::cross(w, &u);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = viewport_width * u;
        let viewport_v = viewport_height * -v;

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;

        // Calculate the location of the upper left pixel.
        let viewport_upper_left =
            self.center - (self.focus_dist * w) - viewport_u / 2.0 - viewport_v / 2.0;
        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);

        // Calculate the camera defocus disk basis vectors.
        let defocus_radius =
            self.focus_dist * f64::tan(degrees_to_radians(self.defocus_angle / 2.0));
        self.defocus_disk_u = u * defocus_radius;
        self.defocus_disk_v = v * defocus_radius;
    }

    fn ray_color(&self, ray: &Ray, depth: i32, world: &dyn Hittable) -> Color {
        // if we've exceeded the ray bounce limit, no more light is gathered.
        if depth <= 0 {
            return Color::new(0., 0., 0.);
        }

        match world.hit(ray, 0.001, f64::INFINITY) {
            Some(hit_record) => {
                let emitted_color =
                    hit_record
                        .material
                        .emitted(hit_record.u, hit_record.v, &hit_record.p);
                match hit_record.material.scatter(ray, &hit_record) {
                    Some(scatter_rec) => {
                        let scatter_color = scatter_rec.attenuation
                            * self.ray_color(&scatter_rec.scattered, depth - 1, world);
                        emitted_color + scatter_color
                    }
                    None => emitted_color,
                }
            }
            None => self.background,
        }
    }

    fn get_ray(&self, i: i32, j: i32) -> Ray {
        let offset = Self::sample_square();
        let pixel_sample = self.pixel00_loc
            + ((i as f64 + offset.x) * self.pixel_delta_u)
            + ((j as f64 + offset.y) * self.pixel_delta_v);

        let ray_origin = if self.defocus_angle <= 0.0 {
            self.center
        } else {
            self.defocus_disk_sample()
        };
        let ray_direction = pixel_sample - ray_origin;
        let ray_time = random_double();

        Ray::new(ray_origin, ray_direction, ray_time)
    }

    fn sample_square() -> Vec3 {
        Vec3::new(random_double() - 0.5, random_double() - 0.5, 0.0)
    }

    fn defocus_disk_sample(&self) -> Point {
        // Returns a random point in the camera defocus disk.
        let p = Vec3::random_in_unit_disk();
        self.center + (p.x * self.defocus_disk_u) + (p.y * self.defocus_disk_v)
    }
}

#[inline]
fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0. {
        return linear_component.sqrt();
    }

    0.
}

fn write_color(out: &mut impl Write, pixel_color: Color) {
    let r = linear_to_gamma(pixel_color.x);
    let g = linear_to_gamma(pixel_color.y);
    let b = linear_to_gamma(pixel_color.z);

    let ir = (256.0 * r.clamp(0.0, 0.999)) as i32;
    let ig = (256.0 * g.clamp(0.0, 0.999)) as i32;
    let ib = (256.0 * b.clamp(0.0, 0.999)) as i32;

    writeln!(out, "{ir} {ig} {ib}").unwrap();
}
