use shape::{Shape, Intersectable};

use std::vec::Vec;

use image::{DynamicImage, Rgba, GenericImage};
use ray::Ray;

pub struct Scene {
    pub width: u32,
    pub height: u32,
    pub fov: f64,
    pub shapes: Vec<&'static Shape>,
}

pub fn render(scene: &Scene) -> DynamicImage{
    let mut image = DynamicImage::new_rgb8(scene.width, scene.height);
    let black = Rgba{ data: [0,0,0,0] };
    for x in 0..scene.width{
        for y in 0..scene.height{
            let ray = Ray::create_normal_ray(x, y, scene);
            for shape in scene.shapes.iter() {
                if shape.intersect(&ray) {
                    image.put_pixel(x, y, shape.color().to_rgba());
                }
                else {
                    image.put_pixel(x, y, black);
                }
            }
        }
    }
    image
}

#[test]
fn test_can_render_scene(){
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
                alpha: 1.0
            },
        }
    };

    let img = render(&scene);
    assert_eq!(scene.width, img.width());
    assert_eq!(scene.height, img.height());
}
