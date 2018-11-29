use math::vector::Vector3;
use math::quaternion::Quat;
use ray::Ray;
use color::Color;

pub trait Intersectable{
    fn intersect(&self, ray: &Ray, normal: &mut Vector3, point: &mut Vector3) -> bool;
}

pub trait Shape : Intersectable {
    fn location(&self) -> Vector3;
    fn color(&self) -> Color;
}

#[derive(Deserialize)]
pub struct Sphere {
    pub center: Vector3,
    pub radius: f64,
    pub color: Color,
}

impl Shape for Sphere{
    fn location(&self) -> Vector3 {
        self.center
    }
    fn color(&self) -> Color {
        self.color
    }
}

impl Intersectable for Sphere{
    fn intersect(&self, ray: &Ray, normal: &mut Vector3, point: &mut Vector3) -> bool {
        let l = self.center - ray.origin;
        let tc = l.dot(&ray.direction);
        if tc < 0.0 {
            return false;
        }
        let sd = l.dot(&l) - tc * tc;


        if sd >= 0.0 {
            let td = (self.radius * self.radius - sd).sqrt();
            let t0 = tc - td;
            let t1 = tc + td;
            if t0 > 0.0 || t1 > 0.0 {
                if t0 > 0.0 { *point = ray.origin + t1 * ray.direction; }
                else { *point = ray.origin + t1 * ray.direction; }

                *normal = *point - self.center;
                normal.normalize();

                true
            }
            else {
                false
            }
        }
        else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use ::shape::*;
    use ::point::Point;
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
        };
        
        let mut hit_normal = Vector3::zero();
        let mut hit_point = Vector3::zero();

        let mut ray = Ray { origin: Vector3::zero(), direction: Vector3 { x:0.0, y:1.0, z:0.0 }};
        assert_eq!(sphere.intersect(&ray, &mut hit_normal, &mut hit_point), false);

        ray = Ray { origin: Vector3::zero(), direction: Vector3 { x:0.0, y:0.0, z:-1.0 }};
        assert_eq!(sphere.intersect(&ray, &mut hit_normal, &mut hit_point), true);

        ray = Ray { origin: Vector3 { x:0.0, y:0.0, z:-5.0 }, direction: Vector3 { x:0.0, y:1.0, z:0.0 }};
        assert_eq!(sphere.intersect(&ray, &mut hit_normal, &mut hit_point), true);

        ray = Ray { origin: Vector3 { x:0.0, y:-1.0, z:-5.0 }, direction: Vector3 { x:0.0, y:1.0, z:0.0 }};
        assert_eq!(sphere.intersect(&ray, &mut hit_normal, &mut hit_point), true);

        ray = Ray { origin: Vector3 { x:0.0, y:1.0, z:-5.0 }, direction: Vector3 { x:0.0, y:1.0, z:0.0 }};
        assert_eq!(sphere.intersect(&ray, &mut hit_normal, &mut hit_point), false);
    }
}

#[derive(Deserialize)]
pub struct Cube {
    pub location: Vector3,
    pub color: Color,
    pub extent: Vector3,
    pub rotation: Quat,
}

impl Shape for Cube{
    fn location(&self) -> Vector3 {
        self.location
    }
    fn color(&self) -> Color {
        self.color
    }
}

impl Intersectable for Cube{
    fn intersect(&self, ray: &Ray, normal: &mut Vector3, point: &mut Vector3) -> bool {
        let l: Vector3 = self.location - ray.origin;
        
        false
    }
}