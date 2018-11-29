use math::vector::Vector3;
use color::Color;

#[derive(PartialEq)]
pub enum LightType {
    Point,
    Directional,
}

pub struct Light {
    pub location: Vector3,
    pub direction: Vector3,
    pub light_type: LightType,
    pub color: Color,
}