use math::vector::*;
use color::Color;
use math::*;
use shape::Shape;

#[derive(PartialEq)]
pub enum LightType {
    Point,
    Directional,
}

pub struct Light {
    pub location: Vector3,
    pub direction: Vector3,
    pub light_type: LightType,
    pub diffuse_color: Color,
    pub specular_color: Color,
}

pub fn blinn_phong(shape: &dyn Shape, light: &Light, pos: Vector3, view: Vector3, normal: Vector3) -> (Color, Color) {    
    let mut light_dir;
    let mut distance;
    if light.light_type == LightType::Directional {
        light_dir = -light.direction;
        distance = 1.0;
    } else {
        light_dir = light.location - pos;
        distance = light_dir.length();
        light_dir = light_dir / distance;
        distance = distance * distance;
    }

    let n_dot_l = normal.dot(&light_dir);
    let diffuse_intensity = clamp(n_dot_l, 0.0, 1.0);

    let diffuse = shape.color() * diffuse_intensity as f32 * light.diffuse_color * (1.0/* diffuse power */ / distance) as f32;

    let half = (light_dir + view.normalize()).normalize();
    let n_dot_h = half.dot(&normal);
    let specular_intensity = clamp(n_dot_h, 0.0, 1.0).powf(4.0/* specular hardness */);

    let specular = shape.color() * specular_intensity as f32 * light.specular_color * (1.0/* specular power */ / distance) as f32;

    (diffuse, specular)
}