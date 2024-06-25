use crate::math::vector3d::Vector3D;
use crate::core::ray::Ray;
use super::object::{Object, Intersection};

pub struct Cube {
    pub min: Vector3D,
    pub max: Vector3D,
}

impl Cube {
    pub fn new(min: Vector3D, max: Vector3D) -> Self {
        Self { min, max }
    }
}

impl Object for Cube {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let mut tmin = (self.min.x - ray.origin.x) / ray.direction.x;
        let mut tmax = (self.max.x - ray.origin.x) / ray.direction.x;

        if tmin > tmax {
            std::mem::swap(&mut tmin, &mut tmax);
        }

        let mut tymin = (self.min.y - ray.origin.y) / ray.direction.y;
        let mut tymax = (self.max.y - ray.origin.y) / ray.direction.y;

        if tymin > tymax {
            std::mem::swap(&mut tymin, &mut tymax);
        }

        if tmin > tymax || tymin > tmax {
            return None;
        }

        if tymin > tmin {
            tmin = tymin;
        }

        if tymax < tmax {
            tmax = tymax;
        }

        let mut tzmin = (self.min.z - ray.origin.z) / ray.direction.z;
        let mut tzmax = (self.max.z - ray.origin.z) / ray.direction.z;

        if tzmin > tzmax {
            std::mem::swap(&mut tzmin, &mut tzmax);
        }

        if tmin > tzmax || tzmin > tmax {
            return None;
        }

        if tzmin > tmin {
            tmin = tzmin;
        }

        if tzmax < tmax {
            tmax = tzmax;
        }

        if tmin < 0.0 {
            tmin = tmax;
            if tmin < 0.0 {
                return None;
            }
        }

        let point = ray.point_at(tmin);
        let normal = self.normal(&point);
        Some(Intersection { t: tmin, point, normal })
    }

    fn normal(&self, point: &Vector3D) -> Vector3D {
        let center = (self.min + self.max) * 0.5;
        let d = *point - center;
        let bias = 1.000001;
        
        if d.x.abs() > d.y.abs() && d.x.abs() > d.z.abs() {
            Vector3D::new(d.x.signum() * bias, 0.0, 0.0)
        } else if d.y.abs() > d.z.abs() {
            Vector3D::new(0.0, d.y.signum() * bias, 0.0)
        } else {
            Vector3D::new(0.0, 0.0, d.z.signum() * bias)
        }
    }
}