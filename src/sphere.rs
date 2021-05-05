use crate::{color::Color, ray::Ray, vec3::Vec3};

pub struct Sphere {
    pub origin: Vec3,
    pub radius: f32,
    pub color: Color,
}

impl Sphere {
    pub fn new(origin: Vec3, radius: f32, color: Color) -> Sphere {
        Sphere {
            origin,
            radius,
            color,
        }
    }

    pub fn hit(&self, ray: &Ray) -> Option<f32> {
        let oc: Vec3 = ray.origin - self.origin;

        let a = ray.direction.dot(ray.direction);
        let half_b = ray.direction.dot(oc);
        let c = oc.dot(oc) - (self.radius * self.radius);
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            // No intersection.
            None
        } else {
            Some((-half_b - discriminant.sqrt()) / a)
        }
    }
}
