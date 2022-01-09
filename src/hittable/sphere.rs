use super::{HitRecord, Hittable};
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
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc: Vec3 = ray.origin - self.origin;

        let a = ray.direction.dot(ray.direction);
        let half_b = ray.direction.dot(oc);
        let c = oc.dot(oc) - (self.radius * self.radius);
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            // No intersection.
            return None;
        }

        // Find the nearest root such that tmin < t < tmax.
        let mut root = -half_b - discriminant.sqrt() / a;
        if root < t_min || t_max < root {
            root = (-half_b - discriminant.sqrt()) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }
        let p = ray.at(root);

        Some(HitRecord {
            p,
            t: root,
            normal: p - self.origin,
        })
    }
}
