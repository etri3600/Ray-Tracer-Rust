use point::Point;
use vector::Vector3;

use image::{DynamicImage, Rgba, GenericImage, Pixel};
use ray::Ray;

pub trait Intersectable{
    fn intersect(&self, ray: &Ray) -> bool;
}

const GAMMA: f32 = 2.2;

fn gamma_encode(linear: f32) -> f32 {
    linear.powf(1.0 / GAMMA)
}

fn gamma_decode(encoded: f32) -> f32 {
    encoded.powf(GAMMA)
}

pub struct Color {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
}

impl Color{
    pub fn to_rgba(&self) -> Rgba<u8> {
        Rgba::from_channels((gamma_encode(self.red) * 255.0) as u8,
                            (gamma_encode(self.green) * 255.0) as u8,
                            (gamma_encode(self.blue) * 255.0) as u8,
                            255)
    }

    pub fn from_rgba(rgba: Rgba<u8>) -> Color {
        Color {
            red: gamma_decode((rgba.data[0] as f32) / 255.0),
            green: gamma_decode((rgba.data[1] as f32) / 255.0),
            blue: gamma_decode((rgba.data[2] as f32) / 255.0),
        }
    }
}

pub struct Sphere {
    pub center: Point,
    pub radius: f64,
    pub color: Color,
}

impl Intersectable for Sphere{
    fn intersect(&self, ray: &Ray) -> bool {
        let l: Vector3 = self.center - ray.origin;
        let adj = l.dot(&ray.direction);
        let d = l.dot(&l) - (adj * adj);
        d < (self.radius * self.radius)
    }
}

pub struct Scene {
    pub width: u32,
    pub height: u32,
    pub fov: f64,
    pub sphere: Sphere,
}

pub fn render(scene: &Scene) -> DynamicImage{
    let mut image = DynamicImage::new_rgb8(scene.width, scene.height);
    let black = Rgba{ data: [0,0,0,0] };
    for x in 0..scene.width{
        for y in 0..scene.height{
            let ray = Ray::create_prime(x, y, scene);
            if scene.sphere.intersect(&ray){
                image.put_pixel(x, y, scene.sphere.color.to_rgba());
            }
            else{
                image.put_pixel(x, y, black);
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
            },
        }
    };

    let img = render(&scene);
    assert_eq!(scene.width, img.width());
    assert_eq!(scene.height, img.height());
}
