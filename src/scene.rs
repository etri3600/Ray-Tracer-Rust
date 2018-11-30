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

    if order > 2 {
        return BLACK;
    }

    let mut hit_normal = Vector3::zero();
    let mut hit_point = Vector3::zero();

    let mut min_distance = std::f64::INFINITY;
    let mut closest_shape: Option<&dyn Shape> = None;

    // find intersect
    for shape in scene.shapes.iter() {
        let mut normal = Vector3::zero();
        let mut point = Vector3::zero();
        if shape.intersect(&ray, &mut normal, &mut point) {
            let distance = point.size_squared();
            if distance < min_distance {
                min_distance = distance;
                closest_shape = Some(*shape);
                hit_normal = normal;
                hit_point = point;
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
                let (diffuse, specular) = blinn_phong(shape, light, hit_point, Vector3::zero() - hit_point, hit_normal); 
                color = color + diffuse + specular;
            }
        }

        let (reflectance, reflection_ray, refraction_ray) = light_calculation(ray.direction, hit_normal, 1.0, shape.refractive_index());
        // reflection
        color = color + reflectance * trace(scene, Ray { origin: hit_point, direction: reflection_ray }, order + 1);
        // refraction
        if reflectance < 1.0 {
            color = color + (1.0 - reflectance) * trace(scene, Ray { origin: hit_point, direction: refraction_ray }, order + 1);
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

fn fresnel(incident: Vector3, normal: Vector3, n1: f32, n2: f32) -> f32 {
    let n = (n1 / n2) as f64;
    let mut cos_i = -incident.dot(&normal);

    let sin_t2 = n * n * (1.0 - cos_i * cos_i).max(0.0);
    if sin_t2 > 1.0 {
        return 1.0;
    }

    let cos_t = (1.0 - sin_t2).sqrt();
    cos_i = cos_i.abs();
    
    let rs = ((n2 as f64 * cos_i) - (n1 as f64 * cos_t)) / ((n2 as f64 * cos_i) + (n1 as f64 * cos_t));
    let rp = ((n1 as f64 * cos_i) - (n2 as f64 * cos_t)) / ((n1 as f64 * cos_i) + (n2 as f64 * cos_t));

    ((rs * rs + rp * rp) / 2.0) as f32
}

fn light_calculation(incident: Vector3, normal: Vector3, n1: f32, n2: f32) -> (f32, Vector3, Vector3) {
    let reflectance;
    let mut refraction_ray = Vector3::zero();
    
    let n = (n1 / n2) as f64;
    let cos_i = -incident.dot(&normal);
    let sin_t2 = n * n * (1.0 - cos_i * cos_i).max(0.0);
    if sin_t2 > 1.0 {
        // total infernal reflection
        reflectance = 1.0;
    }
    else {
        let cos_t = (1.0 - sin_t2).sqrt();
        let cos_i_abs = cos_i.abs();
        
        let ortho = (n1 as f64 * cos_i_abs - n2 as f64 * cos_t) / (n1 as f64 * cos_i_abs + n2 as f64 * cos_t);
        let parallel = (n2 as f64 * cos_i_abs - n1 as f64 * cos_t) / (n2 as f64 * cos_i_abs + n1 as f64 * cos_t);

        reflectance = clamp(((ortho * ortho + parallel * parallel) / 2.0) as f32, 0.0, 1.0);
        refraction_ray = n * incident + (n * cos_i - cos_t) * normal;
    }
    
    let reflection_ray = incident + 2.0 * cos_i * normal;

    (reflectance, reflection_ray.normalize(), refraction_ray.normalize())
}