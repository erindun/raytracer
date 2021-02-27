use crate::{color::Color, ray::Ray, scene_object::SceneObject};
use glam::Vec3;

pub struct Plane {
    pub origin: Vec3,
    pub normal: Vec3,
    pub color: Color,
}

impl SceneObject for Plane {
    fn color(&self) -> Color {
        self.color
    }

    fn intersect(&self, ray: &Ray) -> Option<f32> {
        let denominator = self.normal.dot(ray.direction);
        if denominator.abs() > 1e-6 {
            let difference = self.origin - ray.origin;
            let t = difference.dot(self.normal) / denominator;
            if t > 1e-6 {
                // An intersection is made!
                return Some(t);
            }
        }
        // No intersection.
        None
    }
}
