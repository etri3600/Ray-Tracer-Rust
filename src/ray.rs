use scene::Scene;
use math::vector::Vector3;

#[derive(Copy, Clone, Debug, Deserialize)]
#[repr(C)]
pub struct Ray{
    pub origin: Vector3,
    pub direction: Vector3,
}

impl Ray{
    pub fn create_primary_ray(x: u32, y: u32, scene: &Scene) -> Ray{
        assert!(scene.width > scene.height);
        let fov_adjustment = (scene.fov.to_radians() / 2.0).tan();
        let aspect_ratio = f64::from(scene.width) / f64::from(scene.height);
        let sensor_x = ((f64::from(x) + 0.5) / f64::from(scene.width) * 2.0 - 1.0) * aspect_ratio * fov_adjustment;
        let sensor_y = (1.0 - (f64::from(y) + 0.5) / f64::from(scene.height) * 2.0) * fov_adjustment;

        Ray{
            origin: Vector3::zero(),
            direction: Vector3{
                x: sensor_x,
                y: sensor_y,
                z: -1.0,
            }.normalize()
        }
    }
}