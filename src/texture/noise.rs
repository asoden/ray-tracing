use crate::{
    texture::perlin::Perlin,
    vec3::{Color, Point},
};

#[derive(Clone, Debug)]
pub enum NoiseType {
    Perlin,
    Turbulence,
    Marble,
}

#[derive(Clone, Debug)]
pub struct Noise {
    pub noise_gen: Perlin,
    pub scale: f64,
    pub noise_type: NoiseType,
}

impl Noise {
    pub fn value(&self, point: &Point) -> Color {
        let value = match self.noise_type {
            NoiseType::Perlin => 0.5 * (1.0 + self.noise_gen.noise(&(*point * self.scale))),
            NoiseType::Turbulence => self.noise_gen.turbulence(&(*point * self.scale), 7),
            NoiseType::Marble => {
                0.5 * (1.0
                    + (self.scale * point.z + 10.0 * self.noise_gen.turbulence(point, 7)).sin())
            }
        };

        Color::from(value)
    }
}
