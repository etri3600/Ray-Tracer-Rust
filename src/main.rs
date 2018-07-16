#[macro_use]
extern crate serde_derive;
extern crate image;
extern crate serde;

mod scene;
mod vector;
mod point;
mod ray;

use scene::*;
use point::Point;
use image::DynamicImage;

fn main() {
    let scene = Scene {
        width: 800,
        height: 600,
        fov: 90.0,
        sphere: Sphere{
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
            },
        }
    };
    let image = render(&scene);
    let r = image.save(std::path::Path::new("D:\\test.bmp"));
    if r.is_err(){
        println!("last OS error: {:?}", std::io::Error::last_os_error());
    }
}
