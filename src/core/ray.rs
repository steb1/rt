use crate::math::vector3d::Vector3D;
#[derive(Clone)]
pub struct Ray {
    pub origin: Vector3D,
    pub direction: Vector3D,
}
impl Ray {
    pub fn new(origin: Vector3D, direction: Vector3D) -> Self {
        Self {
            origin,
            direction: direction.normalize(),
        }
    }

    pub fn point_at(&self, t: f32) -> Vector3D {
        self.origin + self.direction * t
    }
}