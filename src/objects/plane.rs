use crate::math::vector3d::Vector3D;
use crate::core::ray::Ray;
use super::object::{Object, Intersection};

pub struct Plane {
    pub point: Vector3D,
    pub normal: Vector3D,
}

impl Plane {
    pub fn new(point: Vector3D, normal: Vector3D) -> Self {
        Self { point, normal: normal.normalize() }
    }
}

impl Object for Plane {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let denom = ray.direction.dot(&self.normal);
        if denom.abs() > 1e-6 {
            let v = self.point - ray.origin;
            let t = v.dot(&self.normal) / denom;
            if t >= 0.0 {
                let point = ray.point_at(t);
                Some(Intersection { t, point, normal: self.normal })
            } else {
                None
            }
        } else {
            None
        }
    }

    fn normal(&self, _point: &Vector3D) -> Vector3D {
        self.normal
    }
}