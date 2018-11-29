use shape::Shape;
use math::vector::Vector3;
use light::*;
use color::Color;
use math::clamp;

use std::vec::Vec;

use image::{DynamicImage, Rgba, GenericImage};
use ray::Ray;

pub struct Scene {
    pub width: u32,
    pub height: u32,
    pub fov: f64,
    pub lights: Vec<Light>,
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
    const BLACK: Color = Color { r:0.0, g:0.0, b:0.0, a:1.0 };
    let mut color: Color = BLACK;

    if order > 1 {
        return BLACK;
    }

    let mut hit_normal = Vector3::zero();
    let mut hit_point = Vector3::zero();

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
        for light in scene.lights.iter() {
            let mut light_direction: Vector3;        
            
            if light.light_type == LightType::Directional {
                light_direction = -light.direction.normalize();
            } else {
                light_direction = (light.location - hit_point).normalize();
            }
            
            let sray = Ray { origin: hit_point + light_direction * 0.1 , direction: light_direction };
            let mut shit_normal = Vector3::zero();
            let mut shit_point = Vector3::zero();

            if shape.intersect(&sray, &mut shit_normal, &mut shit_point) == false {
                let lm = light_direction;
                let n_dot_l = lm.normalize().dot(&hit_normal);
                if n_dot_l > 0.0 {
                    color = color + (shape.color() * n_dot_l as f32) * light.diffuse_color;
                    let view = -hit_point.normalize();
                    let reflected = reflection(lm, hit_normal);
                    let dot = view.dot(&reflected);
                    if dot > 0.0 {
                        //color = color + dot as f32 * light.specular_color;
                    }
                }
            }

            let light_tuple = light_calculation(ray.direction, hit_normal, 1.0, shape.refractive_index());
            // reflection
            color = color + light_tuple.0 * trace(scene, Ray { origin: hit_point, direction: light_tuple.1 }, order + 1);
            // refraction
            if light_tuple.0 < 1.0 {
                color = color + (1.0 - light_tuple.0) * trace(scene, Ray { origin: hit_point, direction: light_tuple.2 }, order + 1);
            }
        }
    }
    else {
        color = Color { r:0.2, g:0.2, b:0.2, a:1.0 };
    }

    color
}

fn refraction(incident: Vector3, normal: Vector3, n1: f32, n2: f32, refracted_ray: &mut Vector3) -> bool {
    let n = (n1 / n2) as f64;
    let cos_i = -incident.dot(&normal);
    let sin_t2 = n * n * (1.0 - cos_i * cos_i);
    if sin_t2 > 1.0 {
        // total infernal reflection
        return false;
    }
    let cos_t = (1.0 - sin_t2).sqrt();

    *refracted_ray = n * incident + (n * cos_i - cos_t) * normal;
    true
}

fn reflection(incident: Vector3, normal: Vector3) -> Vector3
{
    let cos_i = -incident.dot(&normal);
    incident + 2.0 * cos_i * normal
}

fn reflectance(incident: Vector3, normal: Vector3, n1: f32, n2: f32, refracted_ray: &mut Vector3) -> f32 {
    let n = (n1 / n2) as f64;
    let cos_i = -incident.dot(&normal);
    let sin_t2 = n * n * (1.0 - cos_i * cos_i);
    if sin_t2 > 1.0 {
        // total infernal reflection
        return 1.0;
    }
    let cos_t = (1.0 - sin_t2).sqrt();

    let ortho = (n1 as f64 * cos_i - n2 as f64 * cos_t) / (n1 as f64 * cos_i + n2 as f64 * cos_t);
    let parallel = (n2 as f64 * cos_i - n1 as f64 * cos_t) / (n2 as f64 * cos_i + n1 as f64 * cos_t);
    ((ortho * ortho + parallel * parallel) / 2.0) as f32
}

fn light_calculation(incident: Vector3, normal: Vector3, n1: f32, n2: f32) -> (f32, Vector3, Vector3) {
    let mut reflectance = 0.5;
    let mut refraction_ray = Vector3::zero();
    
    let n = (n1 / n2) as f64;
    let cos_i = -incident.dot(&normal);
    let sin_t2 = n * n * (1.0 - cos_i * cos_i);
    if sin_t2 > 1.0 {
        // total infernal reflection
        reflectance = 1.0;
    }
    else {
        let cos_t = (1.0 - sin_t2).sqrt();

        let ortho = (n1 as f64 * cos_i - n2 as f64 * cos_t) / (n1 as f64 * cos_i + n2 as f64 * cos_t);
        let parallel = (n2 as f64 * cos_i - n1 as f64 * cos_t) / (n2 as f64 * cos_i + n1 as f64 * cos_t);

        reflectance = ((ortho * ortho + parallel * parallel) / 2.0) as f32;
        reflectance = 0.5;
        refraction_ray = n * incident + (n * cos_i - cos_t) * normal;
    }
    
    let reflection_ray = incident + 2.0 * cos_i * normal;

    (reflectance, reflection_ray, refraction_ray)
}