use crate::vec3::Vec3;

struct Ray {
    origin: Vec3,
    direction: Vec3,
}

impl Ray {
    fn new(origin: Vec3, direction: Vec3) -> Ray {
        Ray { origin, direction }
    }

    fn at(&self, t: f32) -> Vec3 {
        self.origin + t * self.direction
    }
}
