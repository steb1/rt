use crate::math::vector3d::Vector3D;
use crate::core::ray::Ray;
use super::object::{Object, Intersection};

pub struct Cylinder {
    pub center: Vector3D,
    pub axis: Vector3D,
    pub radius: f32,
    pub height: f32,
}

impl Cylinder {
    pub fn new(center: Vector3D, axis: Vector3D, radius: f32, height: f32) -> Self {
        Self {
            center,
            axis: axis.normalize(),
            radius,
            height,
        }
    }
}

impl Object for Cylinder {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let oc = ray.origin - self.center;
        let a = ray.direction.dot(&ray.direction) - ray.direction.dot(&self.axis).powi(2);
        let b = 2.0 * (ray.direction.dot(&oc) - ray.direction.dot(&self.axis) * oc.dot(&self.axis));
        let c = oc.dot(&oc) - oc.dot(&self.axis).powi(2) - self.radius.powi(2);

        let discriminant = b * b - 4.0 * a * c;
        if discriminant < 0.0 {
            return None;
        }

        let t = (-b - discriminant.sqrt()) / (2.0 * a);
        if t <= 0.0 {
            return None;
        }

        let point = ray.point_at(t);
        let height_check = (point - self.center).dot(&self.axis);
        if height_check < 0.0 || height_check > self.height {
            return None;
        }

        let normal = (point - (self.center + self.axis * height_check)).normalize();
        Some(Intersection { t, point, normal })
    }

    fn normal(&self, point: &Vector3D) -> Vector3D {
        let height_check = (*point - self.center).dot(&self.axis);
        (*point - (self.center + self.axis * height_check)).normalize()
    }
}