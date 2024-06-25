use crate::math::vector3d::Vector3D;
use crate::core::ray::Ray;
use std::f32::consts::PI;

pub struct Camera {
    pub position: Vector3D,
    pub look_at: Vector3D,
    pub up: Vector3D,
    pub fov: f32,
    pub aspect_ratio: f32,
}

impl Camera {
    pub fn new(position: Vector3D, look_at: Vector3D, up: Vector3D, fov: f32, aspect_ratio: f32) -> Self {
        Self {
            position,
            look_at,
            up,
            fov,
            aspect_ratio,
        }
    }

    pub fn generate_ray(&self, s: f32, t: f32) -> Ray {
        let theta = self.fov * PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = self.aspect_ratio * half_height;

        let w = (self.position - self.look_at).normalize();
        let u = self.up.cross(&w).normalize();
        let v = w.cross(&u);

        let lower_left_corner = self.position - u * half_width - v * half_height - w;
        let horizontal = u * 2.0 * half_width;
        let vertical = v * 2.0 * half_height;

        Ray::new(
            self.position,
            lower_left_corner + horizontal * s + vertical * t - self.position,
        )
    }
}