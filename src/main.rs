#[macro_use]
extern crate serde_derive;
extern crate image;
extern crate serde;

mod scene;
mod vector;
mod point;
mod ray;
mod shape;
mod quaternion;

use scene::*;
use shape::*;
use point::Point;

use std::vec::Vec;

fn main() {
    let mut shapes = Vec::new();

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
