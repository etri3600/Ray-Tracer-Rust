use point::Point;
use math::vector::Vector3;
use math::quaternion::Quat;
use ray::Ray;

use image::{Rgba, Pixel};

const GAMMA: f32 = 2.2;

fn gamma_encode(linear: f32) -> f32 {
    linear.powf(1.0 / GAMMA)
}

fn gamma_decode(encoded: f32) -> f32 {
    encoded.powf(GAMMA)
}

#[derive(Copy, Clone, Debug, Deserialize)]
pub struct Color {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
    pub alpha: f32
}

impl Color{
    pub fn to_rgba(&self) -> Rgba<u8> {
        Rgba::from_channels((gamma_encode(self.red) * 255.0) as u8,
                            (gamma_encode(self.green) * 255.0) as u8,
                            (gamma_encode(self.blue) * 255.0) as u8,
                            (gamma_encode(self.alpha) * 255.0) as u8)
    }

    pub fn from_rgba(rgba: Rgba<u8>) -> Color {
        Color {
            red: gamma_decode((rgba.data[0] as f32) / 255.0),
            green: gamma_decode((rgba.data[1] as f32) / 255.0),
            blue: gamma_decode((rgba.data[2] as f32) / 255.0),
            alpha: gamma_decode((rgba.data[3] as f32) / 255.0)
        }
    }
}

pub trait Intersectable{
    fn intersect(&self, ray: &Ray) -> bool;
}

pub trait Shape : Intersectable {
    fn location(&self) -> Point;
    fn color(&self) -> Color;
}

#[derive(Deserialize)]
pub struct Sphere {
    pub center: Point,
    pub radius: f64,
    pub color: Color,
}

impl Shape for Sphere{
    fn location(&self) -> Point {
        self.center
    }
    fn color(&self) -> Color {
        self.color
    }
}

impl Intersectable for Sphere{
    fn intersect(&self, ray: &Ray) -> bool {
        let l: Vector3 = self.center - ray.origin;
        let adj = l.dot(&ray.direction);
        let d = l.dot(&l) - (adj * adj);
        d < (self.radius * self.radius)
    }
}

#[derive(Deserialize)]
pub struct Cube {
    pub location: Point,
    pub color: Color,
    pub extent: Vector3,
    pub rotation: Quat,
}

impl Shape for Cube{
    fn location(&self) -> Point {
        self.location
    }
    fn color(&self) -> Color {
        self.color
    }
}

impl Intersectable for Cube{
    fn intersect(&self, ray: &Ray) -> bool {
        let l: Vector3 = self.location - ray.origin;
        
        false
    }
}