use image::{Rgba, Pixel};
use serde_derive::Deserialize;
use crate::math::*;
use std::ops::{Mul, Add};

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
            r: gamma_decode(f32::from(rgba[0]) / 255.0),
            g: gamma_decode(f32::from(rgba[1]) / 255.0),
            b: gamma_decode(f32::from(rgba[2]) / 255.0),
            a: gamma_decode(f32::from(rgba[3]) / 255.0)
        }
    }
}

impl Add for Color {
    type Output = Color;

    fn add(self, rhs: Color) -> Color {
        let mut color = Color { r: self.r + rhs.r, g: self.g + rhs.g, b: self.b + rhs.b, a: self.a + rhs.a };
        color.r = clamp(color.r, 0.0, 1.0);
        color.g = clamp(color.g, 0.0, 1.0);
        color.b = clamp(color.b, 0.0, 1.0);
        color.a = clamp(color.a, 0.0, 1.0);
        color
    }
}

impl Mul for Color {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Color { r: self.r * rhs.r, g: self.g * rhs.g, b: self.b * rhs.b, a: self.a * rhs.a }
    }
}

impl Mul<f32> for Color {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        Color { r: self.r * rhs, g: self.g * rhs, b: self.b * rhs, a: self.a * rhs }
    }
}

impl Mul<Color> for f32 {
    type Output = Color;

    fn mul(self, rhs: Color) -> Color {
        Color { r: self * rhs.r, g: self * rhs.g, b: self * rhs.b, a: self * rhs.a }
    }
}