extern crate image;

use image::DynamicImage;

pub struct Color {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
}

pub struct Sphere {
    pub center: Point,
    pub radius: f64,
    pub color: Color,
}

pub struct Scene {
    pub width: u32,
    pub height: u32,
    pub fov: f64,
    pub sphere: Sphee,
}

pub fn render(scene: &Scene) -> DynamicImage{
    DynamicImage::new_rgb8(scene.width, scene.height)
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
            },
        }
    };

    let img = render(&scene);
    assert_eq!(scene.width, img.width());
    assert_eq!(scene.height, img.height());
}