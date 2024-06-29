use crate::objects::object::Object;
use crate::scene::light::Light;

pub struct Scene {
    pub objects: Vec<Box<dyn Object>>,
    pub lights: Vec<Light>,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            lights: Vec::new(),
        }
    }

    pub fn add_object(&mut self, object: Box<dyn Object>) {
        self.objects.push(object);
    }

    pub fn add_light(&mut self, light: Light) {
        self.lights.push(light);
    }
}