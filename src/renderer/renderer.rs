use crate::math::vector3d::Vector3D;
use crate::core::color::Color;
use crate::core::ray::Ray;
use crate::scene::light::Light;
use crate::scene::scene::Scene;
use crate::core::camera::Camera;
use crate::objects::object::{Intersection, Object};
use std::f32::INFINITY;

pub struct Renderer {
    pub width: u32,
    pub height: u32,
}

impl Renderer {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    pub fn render(&self, scene: &Scene, camera: &Camera) -> Vec<Color> {
        let mut image = vec![Color::new(0.0, 0.0, 0.0); (self.width * self.height) as usize];

        for y in 0..self.height {
            for x in (0..self.width).rev() {
                let u = x as f32 / self.width as f32;
                let v = y as f32 / self.height as f32;
                let ray = camera.generate_ray(u, v);
                let color = self.trace_ray(ray, scene, 0);
                image[(y * self.width + x) as usize] = color;
            }
        }

        image
    }

    fn trace_ray(&self, ray: Ray, scene: &Scene, depth: u32) -> Color {
        if depth > 5 {
            return Color::new(0.0, 0.0, 0.0);
        }

        let clone_ray = ray.clone();

        if let Some((intersection, object)) = self.nearest_intersection(ray, scene) {

            self.shade(clone_ray, scene, &intersection, object, depth)
        } else {
            Color::new(0.0, 0.0, 0.0) // Background color
        }
    }

    fn nearest_intersection<'a>(&self, ray: Ray, scene: &'a Scene) -> Option<(Intersection, &'a Box<dyn Object>)> {
        let mut nearest: Option<(Intersection, &Box<dyn Object>)> = None;
        let mut min_distance = INFINITY;

        for object in &scene.objects {
            if let Some(intersection) = object.intersect(&ray) {
                if intersection.t < min_distance {
                    min_distance = intersection.t;
                    nearest = Some((intersection, object));
                }
            }
        }

        nearest
    }

    fn shade(&self, ray: Ray, scene: &Scene, intersection: &Intersection, object: &Box<dyn Object>, depth: u32) -> Color {
        let mut color = Color::new(0.1, 0.1, 0.1); // Ambient light

        for light in &scene.lights {
            let light_dir = (light.position - intersection.point).normalize();
            let shadow_ray = Ray::new(intersection.point + light_dir * 0.1, light_dir);

            if !self.is_in_shadow(shadow_ray, scene, light) {
                let normal = object.normal(&intersection.point);
                let diffuse = normal.dot(&light_dir).max(0.0);
                color = color + light.color * light.intensity * diffuse;

                // Add specular highlight
                // let view_dir = (ray.origin - intersection.point).normalize();
                // let reflect_dir = light_dir - normal * 2.0 * light_dir.dot(&normal);
                // let specular = view_dir.dot(&reflect_dir).max(0.0).powf(32.0);
                // color = color + light.color * light.intensity * specular * 0.5;
            }
        }

        color
    }

    fn is_in_shadow(&self, shadow_ray: Ray, scene: &Scene, light: &Light) -> bool {
        for object in &scene.objects {
            if let Some(intersection) = object.intersect(&shadow_ray) {
                if intersection.t < (light.position - shadow_ray.origin).length() {
                    return true;
                }
            }
        }
        false
    }
}