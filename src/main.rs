#[macro_use]
extern crate serde_derive;
extern crate image;
extern crate serde;

mod scene;
mod math;
mod point;
mod ray;
mod shape;

use scene::*;
use shape::*;
use point::Point;

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
                red: 0.4,
                green: 1.0,
                blue: 0.4,
                alpha: 1.0
            },
    });

    let scene = Scene {
        width: 800,
        height: 600,
        fov: 90.0,
        shapes
    };
    let image = render(&scene);
    let r = image.save(std::path::Path::new(r#"./Render.png"#));
    if r.is_err(){
        println!("last OS error: {:?}", std::io::Error::last_os_error());
    }
}
