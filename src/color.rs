use image::{Rgba, Pixel};

use std::ops::Mul;

const GAMMA: f32 = 2.2;

fn gamma_encode(linear: f32) -> f32 {
    linear.powf(1.0 / GAMMA)
}

fn gamma_decode(encoded: f32) -> f32 {
    encoded.powf(GAMMA)
}

#[derive(Copy, Clone, Debug, Deserialize)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32
}

impl Color{
    pub fn to_rgba(&self) -> Rgba<u8> {
        Rgba::from_channels((gamma_encode(self.r) * 255.0) as u8,
                            (gamma_encode(self.g) * 255.0) as u8,
                            (gamma_encode(self.b) * 255.0) as u8,
                            (gamma_encode(self.a) * 255.0) as u8)
    }

    pub fn from_rgba(rgba: Rgba<u8>) -> Color {
        Color {
            r: gamma_decode((rgba.data[0] as f32) / 255.0),
            g: gamma_decode((rgba.data[1] as f32) / 255.0),
            b: gamma_decode((rgba.data[2] as f32) / 255.0),
            a: gamma_decode((rgba.data[3] as f32) / 255.0)
        }
    }
}

impl Mul for Color {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Color { r: self.r * rhs.r, g: self.g * rhs.g, b: self.b * rhs.b, a: self.a * rhs.a }
    }
}