pub mod sphere;
use crate::ray::Ray;

pub trait Hittable {
    fn hit(&self, ray: &Ray) -> Option<f32>;
}
