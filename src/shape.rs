use point::Point;
use math::vector::Vector3;
use math::quaternion::Quat;
use ray::Ray;
use color::Color;

pub trait Intersectable{
    fn intersect(&self, ray: &Ray, normal: &mut Vector3, point: &mut Point) -> bool;
}

pub trait Shape : Intersectable {
    fn location(&self) -> Point;
    fn color(&self) -> Color;
}

#[derive(Deserialize)]
pub struct Sphere {
    pub center: Point,
    pub radius: f64,
    pub color: Color,
}

impl Shape for Sphere{
    fn location(&self) -> Point {
        self.center
    }
    fn color(&self) -> Color {
        self.color
    }
}

impl Intersectable for Sphere{
    fn intersect(&self, ray: &Ray, normal: &mut Vector3, point: &mut Point) -> bool {
        let l: Vector3 = self.center - ray.origin;
        let adj = ray.direction.dot(&l);
        let d = l.dot(&l) - (adj * adj);

        if d > 0.0 && d < (self.radius * self.radius) {
            let t1 = adj + self.radius;
            let t2 = adj - self.radius;
            
            if t2 > 0.0 { *point = ray.origin + t2 * ray.direction; }
            else { *point = ray.origin + t1 * ray.direction; }
            *normal = *point - self.center;
            normal.normalize();

            true
        }
        else {
            false
        }
    }
}

#[derive(Deserialize)]
pub struct Cube {
    pub location: Point,
    pub color: Color,
    pub extent: Vector3,
    pub rotation: Quat,
}

impl Shape for Cube{
    fn location(&self) -> Point {
        self.location
    }
    fn color(&self) -> Color {
        self.color
    }
}

impl Intersectable for Cube{
    fn intersect(&self, ray: &Ray, normal: &mut Vector3, point: &mut Point) -> bool {
        let l: Vector3 = self.location - ray.origin;
        
        false
    }
}