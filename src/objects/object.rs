use crate::math::vector3d::Vector3D;
use crate::core::ray::Ray;

pub struct Intersection {
    pub t: f32,
    pub point: Vector3D,
    pub normal: Vector3D,
}

pub trait Object: Send + Sync {
    fn intersect(&self, ray: &Ray) -> Option<Intersection>;
    fn normal(&self, point: &Vector3D) -> Vector3D;
}