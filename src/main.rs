mod scene;
mod math;
mod ray;
mod shape;
mod light;
mod color;

use crate::scene::*;
use crate::shape::*;
use crate::color::Color;
use crate::light::*;
use crate::math::vector::Vector3;

use std::vec::Vec;

fn main() {
    let mut lights = Vec::<Light>::new();
    lights.push(Light {
            location: Vector3::zero(),
            direction: Vector3{ x: -1.0, y: -1.0, z: 0.0 },
            light_type: LightType::Directional,
            diffuse_color: Color { r: 0.4, g: 0.4, b: 0.1, a: 1.0 },
            specular_color: Color { r: 0.8, g: 0.8, b: 0.0, a: 1.0 },
    });

    lights.push(Light {
            location: Vector3::zero(),
            direction: Vector3{ x: 1.0, y: -2.0, z: 0.0 },
            light_type: LightType::Directional,
            diffuse_color: Color { r: 0.4, g: 0.3, b: 0.7, a: 1.0 },
            specular_color: Color { r: 0.8, g: 0.1, b: 1.0, a: 1.0 },
    });

    let mut shapes = Vec::<&Shape>::new();
    shapes.push(&Sphere{
            center: Vector3{
                x: 0.0,
                y: 0.0,
                z: -5.0,
            },
            radius: 1.0,
            color: Color{
                r: 0.4,
                g: 1.0,
                b: 0.4,
                a: 1.0
            },
            refractive_index: 1.0
    });

    shapes.push(&Sphere{
            center: Vector3{
                x: -4.0,
                y: 2.0,
                z: -8.0,
            },
            radius: 2.0,
            color: Color{
                r: 0.0,
                g: 0.1,
                b: 8.0,
                a: 1.0
            },
            refractive_index: 1.2
    });

    shapes.push(&Sphere{
            center: Vector3{
                x: -1.0,
                y: -1.0,
                z: -2.0,
            },
            radius: 0.3,
            color: Color{
                r: 1.0,
                g: 1.0,
                b: 0.0,
                a: 1.0
            },
            refractive_index: 1.4
    });

    shapes.push(&Sphere{
            center: Vector3{
                x: 2.0,
                y: 1.0,
                z: -5.0,
            },
            radius: 0.5,
            color: Color{
                r: 0.0,
                g: 1.0,
                b: 1.0,
                a: 1.0
            },
            refractive_index: 1.5
    });

    shapes.push(&Sphere{
            center: Vector3{
                x: -8.5,
                y: 5.0,
                z: -5.0,
            },
            radius: 0.5,
            color: Color{
                r: 0.3,
                g: 0.3,
                b: 0.3,
                a: 1.0
            },
            refractive_index: 1.0
    });

    let scene = Scene {
        width: 1280,
        height: 720,
        fov: 90.0,
        lights,
        shapes,
    };
    let image = render(&scene);
    let r = image.save(std::path::Path::new(r#"./Render.png"#));
    if r.is_err(){
        println!("last OS error: {:?}", std::io::Error::last_os_error());
    }
}
