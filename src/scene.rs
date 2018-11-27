use shape::Shape;
use math::vector::Vector3;
use point::Point;
use light::Light;
use color::Color;

use std::vec::Vec;

use image::{DynamicImage, Rgba, GenericImage};
use ray::Ray;

pub struct Scene {
    pub width: u32,
    pub height: u32,
    pub fov: f64,
    pub light: Light,
    pub shapes: Vec<&'static dyn Shape>,
}

pub fn render(scene: &Scene) -> DynamicImage{
    let mut image = DynamicImage::new_rgb8(scene.width, scene.height);
    for x in 0..scene.width{
        for y in 0..scene.height{
            let ray = Ray::create_primary_ray(x, y, scene);
            let color = trace(scene, ray, 0);
            image.put_pixel(x, y, color.to_rgba());
        }
    }
    image
}

pub fn trace(scene: &Scene, ray: Ray, order: u8) -> Color {
    let color: Color;

    let mut hit_normal = Vector3::zero();
    let mut hit_point = Point::zero();

    let mut min_distance = std::f64::INFINITY;
    let mut closest_shape: Option<&dyn Shape> = None;

    // find intersect
    for shape in scene.shapes.iter() {
        if shape.intersect(&ray, &mut hit_normal, &mut hit_point) {
            let distance = hit_point.size_squared();
            if distance < min_distance {
                min_distance = distance;
                closest_shape = Some(*shape);
            }
        }
    }

    if let Some(shape) = closest_shape {
        let direction = (scene.light.location - shape.location()).normalize();
        let sray = Ray { origin: shape.location() + direction * 0.1 , direction };
        let mut shit_normal = Vector3::zero();
        let mut shit_point = Point::zero();
        if shape.intersect(&sray, &mut shit_normal, &mut shit_point) {
            // shadow
            color = shape.color();
        }
        else {
            // light
            color = shape.color() * scene.light.color;
        }

        // may reflection or refration calculation
    }
    else {
        color = Color { r:0.0, g:0.0, b:0.0, a:1.0 };
    }

    color
}