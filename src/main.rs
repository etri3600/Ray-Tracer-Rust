#[macro_use]
extern crate serde_derive;
extern crate image;
extern crate serde;

mod scene;
mod math;
mod point;
mod ray;
mod shape;
mod light;
mod color;

use scene::*;
use shape::*;
use point::Point;
use color::Color;
use light::*;
use math::vector::Vector3;

use std::vec::Vec;

fn main() {
    let mut shapes = Vec::<&Shape>::new();
    shapes.push(&Sphere{
            center: Point{
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
    });

    let scene = Scene {
        width: 800,
        height: 600,
        fov: 90.0,
        light: Light {
            location: Point::zero(),
            direction: Vector3{ x: -1.0, y: -1.0, z: 0.0 },
            light_type: LightType::Directional,
            color: Color { r: 0.4, g: 0.4, b: 0.1, a: 0.0 }
        },
        shapes
    };
    let image = render(&scene);
    let r = image.save(std::path::Path::new(r#"./Render.png"#));
    if r.is_err(){
        println!("last OS error: {:?}", std::io::Error::last_os_error());
    }
}
