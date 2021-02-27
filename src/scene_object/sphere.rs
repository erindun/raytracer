use crate::{color::Color, ray::Ray, scene_object::SceneObject};
use glam::Vec3;

pub struct Sphere {
    pub origin: Vec3,
    pub radius: f32,
    pub color: Color,
}

impl SceneObject for Sphere {
    fn color(&self) -> Color {
        self.color
    }

    fn intersect(&self, ray: &Ray) -> Option<f32> {
        let oc: Vec3 = ray.origin - self.origin;

        let a = ray.direction.dot(ray.direction);
        let b = 2.0 * (ray.direction.dot(oc));
        let c = oc.dot(oc) - (self.radius * self.radius);
        let discriminant = (b * b) - (4.0 * a * c);

        if discriminant < 0.0 {
            // No intersection.
            None
        } else {
            // An intersection is made! Return the
            // smallest t value where t > 0.
            let t1 = (-b + discriminant.sqrt()) / 2.0 * a;
            let t2 = (-b - discriminant.sqrt()) / 2.0 * a;

            if t1 > 0.0 && t2 > 0.0 {
                Some(t1.min(t2))
            } else if t1 > 0.0 {
                Some(t1)
            } else {
                Some(t2)
            }
        }
    }
}
