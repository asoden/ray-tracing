use std::path::Path;

use image::ImageReader;
use image::RgbaImage;

use crate::texture::noise::Noise;
use crate::vec3::{Color, Point};

pub mod noise;
pub mod perlin;

#[derive(Debug, Clone)]
pub enum Texture {
    Constant {
        color: Color,
    },
    Checkered {
        odd: Box<Texture>,
        even: Box<Texture>,
    },
    Image {
        data: Option<RgbaImage>,
    },
    Noise(Noise),
}

impl Texture {
    pub fn color(color: Color) -> Self {
        Self::Constant { color }
    }

    pub fn checkered(odd: Texture, even: Texture) -> Self {
        Self::Checkered {
            odd: odd.into(),
            even: even.into(),
        }
    }

    pub fn image<T: AsRef<Path>>(file: T) -> Self {
        let data = ImageReader::open(file)
            .ok()
            .and_then(|raw| raw.decode().map(|x| x.to_rgba8()).ok());
        Self::Image { data }
    }

    pub fn value(&self, u: f64, v: f64, p: &Point) -> Color {
        match self {
            Texture::Constant { color } => *color,
            Texture::Checkered { odd, even } => {
                let sines = f64::sin(10.0 * p.x) * f64::sin(10.0 * p.y) * f64::sin(10.0 * p.z);
                if sines < 0.0 {
                    odd.value(u, v, p)
                } else {
                    even.value(u, v, p)
                }
            }
            Texture::Image { data } => {
                if let Some(data) = data {
                    let u = u.clamp(0.0, 1.0);
                    let v = 1.0 - v.clamp(0.0, 1.0);

                    let (i, j) = {
                        let mut i = (u * data.width() as f64) as u32;
                        let mut j = (v * data.height() as f64) as u32;

                        if i >= data.width() {
                            i = data.width() - 1;
                        }
                        if j >= data.height() {
                            j = data.height() - 1;
                        }

                        (i, j)
                    };

                    let pixel = data.get_pixel(i, j).0;

                    let r = pixel[0] as f64 / 255.0;
                    let g = pixel[1] as f64 / 255.0;
                    let b = pixel[2] as f64 / 255.0;

                    Color::new(r, g, b)
                } else {
                    // no image found use Magenta for broken image
                    Color::new(1.0, 0.0, 1.0)
                }
            }
            Self::Noise(noise) => noise.value(p),
        }
    }
}

// #[derive(Clone)]
// pub struct ConstantTexture {
//     color: Color,
// }

// impl ConstantTexture {
//     pub fn new(color: Color) -> Self {
//         Self { color }
//     }
// }

// impl Texture for ConstantTexture {
//     fn value(&self, _u: f64, _v: f64, _p: &Vec3) -> Color {
//         self.color
//     }
// }

// pub struct CheckerTexture<T: Texture, U: Texture> {
//     odd: T,
//     even: U,
// }

// impl<T: Texture, U: Texture> CheckerTexture<T, U> {
//     pub fn new(odd: T, even: U) -> Self {
//         Self { odd, even }
//     }
// }

// impl<T: Texture, U: Texture> Texture for CheckerTexture<T, U> {
//     fn value(&self, u: f64, v: f64, p: &Vec3) -> Color {
//         let sines = f64::sin(10.0 * p.x) * f64::sin(10.0 * p.y) * f64::sin(10.0 * p.z);
//         if sines < 0.0 {
//             self.odd.value(u, v, p)
//         } else {
//             self.even.value(u, v, p)
//         }
//     }
// }
