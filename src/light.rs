use math::vector::Vector3;
use point::Point;
use color::Color;

pub enum LightType {
    Point,
    Directional,
}

pub struct Light {
    pub location: Point,
    pub direction: Vector3,
    pub light_type: LightType,
    pub color: Color,
}