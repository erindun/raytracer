use glam::Vec3;

pub struct Camera {
    origin: Vec3,
    forward: Vec3,
    up: Vec3,
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            origin: Vec3::new(0.0, 0.0, 0.0),
            forward: Vec3::new(0.0, 0.0, 1.0),
            up: Vec3::new(0.0, 1.0, 0.0),
        }
    }

    pub fn eye(&self) -> Vec3 {
        self.origin
    }

    pub fn calculate_ray_dir(&self, x: f32, y: f32) -> Vec3 {
        let right = self.up.cross(self.forward);
        let fov = 90.0 * std::f32::consts::PI / 180.0;
        let aspect: f32 = 1.3333;

        let u = ((2.0 * x) - 1.0) * (fov / 2.0).tan();
        let v = ((2.0 * y) - 1.0) * (fov / 2.0).tan() / aspect;
        let vx: Vec3 = (u * right) + (v * self.up);
        let np: Vec3 = vx + self.forward;
        np.normalize()
    }
}
