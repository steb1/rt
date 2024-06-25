use crate::math::vector3d::Vector3D;
use crate::core::ray::Ray;
use super::object::{Object, Intersection};

pub struct Sphere {
    pub center: Vector3D,
    pub radius: f32,
}

impl Sphere {
    pub fn new(center: Vector3D, radius: f32) -> Self {
        Self { center, radius }
    }
}

impl Object for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let oc = ray.origin - self.center;
        let a = ray.direction.dot(&ray.direction);
        let b = 2.0 * oc.dot(&ray.direction);
        let c = oc.dot(&oc) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            None
        } else {
            let t = (-b - discriminant.sqrt()) / (2.0 * a);
            if t > 0.0 {
                let point = ray.point_at(t);
                let normal = (point - self.center).normalize();
                Some(Intersection { t, point, normal })
            } else {
                None
            }
        }
    }

    fn normal(&self, point: &Vector3D) -> Vector3D {
        (*point - self.center).normalize()
    }
}