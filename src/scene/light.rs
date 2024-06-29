use crate::math::vector3d::Vector3D;
use crate::core::color::Color;

pub struct Light {
    pub position: Vector3D,
    pub color: Color,
    pub intensity: f32,
}

impl Light {
    pub fn new(position: Vector3D, color: Color, intensity: f32) -> Self {
        Self { position, color, intensity }
    }
}