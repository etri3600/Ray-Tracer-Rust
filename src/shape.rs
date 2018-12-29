use math::*;
use math::vector::Vector3;
use math::quaternion::Quat;
use ray::Ray;
use color::Color;

pub trait Intersectable{
    fn intersect(&self, ray: &Ray, normal: &mut Vector3, point: &mut Vector3) -> u8;
}

pub trait Shape : Intersectable {
    fn location(&self) -> Vector3;
    fn color(&self) -> Color;
    fn refractive_index(&self) -> f32;
}

#[derive(Deserialize)]
pub struct Sphere {
    pub center: Vector3,
    pub radius: f64,
    pub color: Color,
    pub refractive_index: f32
}

impl Shape for Sphere{
    fn location(&self) -> Vector3 {
        self.center
    }
    fn color(&self) -> Color {
        self.color
    }
    fn refractive_index(&self) -> f32 {
        self.refractive_index
    }
}

impl Intersectable for Sphere{
    fn intersect(&self, ray: &Ray, normal: &mut Vector3, point: &mut Vector3) -> u8 {
        let l = self.center - ray.origin;
        let tc = l.dot(&ray.direction);
        if tc < 0.0 { return 0; }

        let sd = l.dot(&l) - tc * tc;
        if sd >= 0.0 {
            let td = (self.radius * self.radius - sd).sqrt();
            let t0 = tc - td;
            let t1 = tc + td;
            if t0 > 0.0 || t1 > 0.0 {
                if t0.is_nearly_zero() == false { 
                    *point = ray.origin + t0 * ray.direction; 
                }
                else { 
                    *point = ray.origin + t1 * ray.direction; 
                }

                *normal = (*point - self.center).normalize();

                if t0 > 0.0 && t1 > 0.0 { 2 } else { 1 }
            }
            else {
                0
            }
        }
        else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use ::shape::*;
    use ::color::Color;
    use ::math::vector::Vector3;
    use ::ray::Ray;
    #[test]
    fn goo() { 
        let sphere = Sphere{
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
        };
        
        let mut hit_normal = Vector3::zero();
        let mut hit_point = Vector3::zero();

        let mut ray = Ray { origin: Vector3::zero(), direction: Vector3 { x:0.0, y:1.0, z:0.0 }};
        assert_eq!(sphere.intersect(&ray, &mut hit_normal, &mut hit_point), 0);

        ray = Ray { origin: Vector3::zero(), direction: Vector3 { x:0.0, y:0.0, z:-1.0 }};
        assert_eq!(sphere.intersect(&ray, &mut hit_normal, &mut hit_point) > 0, true);

        ray = Ray { origin: Vector3 { x:0.0, y:0.0, z:-5.0 }, direction: Vector3 { x:0.0, y:1.0, z:0.0 }};
        assert_eq!(sphere.intersect(&ray, &mut hit_normal, &mut hit_point) > 0, true);

        ray = Ray { origin: Vector3 { x:0.0, y:-1.0, z:-5.0 }, direction: Vector3 { x:0.0, y:1.0, z:0.0 }};
        assert_eq!(sphere.intersect(&ray, &mut hit_normal, &mut hit_point) > 0, true);

        ray = Ray { origin: Vector3 { x:0.0, y:1.0, z:-5.0 }, direction: Vector3 { x:0.0, y:1.0, z:0.0 }};
        assert_eq!(sphere.intersect(&ray, &mut hit_normal, &mut hit_point), 0);

        ray = Ray { origin: Vector3 { x:0.0, y:0.0, z:0.0 }, direction: Vector3 { x:0.0, y:0.0, z:-1.0 }};
        assert_eq!(sphere.intersect(&ray, &mut hit_normal, &mut hit_point) > 0, true);

        println!("{:?}, {:?}", hit_normal, hit_point);
    }
}

#[derive(Deserialize)]
pub struct Cube {
    pub location: Vector3,
    pub color: Color,
    pub extent: Vector3,
    pub rotation: Quat,
    pub refractive_index: f32,
}

impl Shape for Cube{
    fn location(&self) -> Vector3 {
        self.location
    }
    fn color(&self) -> Color {
        self.color
    }
    fn refractive_index(&self) -> f32 {
        self.refractive_index
    }
}

impl Intersectable for Cube{
    fn intersect(&self, ray: &Ray, normal: &mut Vector3, point: &mut Vector3) -> u8 {
        let l: Vector3 = self.location - ray.origin;
        
        0
    }
}