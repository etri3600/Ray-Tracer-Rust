use math::vector::Vector3;

#[derive(Copy, Clone, Debug, Deserialize)]
#[repr(C)]
pub struct Quat {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32
}

impl Quat {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Quat {
        Quat { x, y, z, w }
    }

    pub fn identity() -> Quat {
        Quat { x: 1.0, y: 0.0 , z: 0.0 , w: 0.0 }
    }

    pub fn normalize(&mut self) -> &mut Self {
        let size_squared = self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w;
        let size = size_squared.sqrt();

        self.x /= size;
        self.y /= size;
        self.z /= size;
        self.w /= size;

        self
    }

    pub fn conjugate(&mut self) -> &mut Self {
        self.x = -self.x;
        self.y = -self.y;
        self.z = -self.z;
        self
    }

    #[allow(clippy::let_and_return)]
    pub fn rotate(&self, vec: Vector3) -> Vector3 {
        let q = Vector3 { x: self.x, y: self.y, z: self.z };
        let t = 2.0 * q.cross(&vec);
        let r = vec + (self.w * t) + q.cross(&t);
        r
    }
}