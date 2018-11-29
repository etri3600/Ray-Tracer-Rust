#[macro_use]
extern crate serde_derive;
extern crate image;
extern crate serde;

mod scene;
mod math;
mod ray;
mod shape;
mod light;
mod color;

use scene::*;
use shape::*;
use color::Color;
use light::*;
use math::vector::Vector3;

use std::vec::Vec;

fn main() {
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

    let scene = Scene {
        width: 800,
        height: 600,
        fov: 90.0,
        light: Light {
            location: Vector3::zero(),
            direction: Vector3{ x: -1.0, y: -1.0, z: 0.0 },
            light_type: LightType::Directional,
            color: Color { r: 0.4, g: 0.4, b: 0.1, a: 1.0 }
        },
        shapes
    };
    let image = render(&scene);
    let r = image.save(std::path::Path::new(r#"./Render.png"#));
    if r.is_err(){
        println!("last OS error: {:?}", std::io::Error::last_os_error());
    }
}
