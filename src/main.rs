mod math;
mod core;
mod objects;
mod scene;
mod renderer;

use math::vector3d::Vector3D;
use core::color::Color;
use core::camera::Camera;
use objects::sphere::Sphere;
use objects::plane::Plane;
use objects::cylinder::Cylinder;
use objects::cube::Cube;
use scene::scene::Scene;
use scene::light::Light;
use renderer::renderer::Renderer;

use std::fs::File;
use std::io::Write;

fn main() {
    let width = 800;
    let height = 600;

    // Scene 1: A sphere (unchanged)
    let camera1 = Camera::new(
        Vector3D::new(0.0, 0.0, -5.0),
        Vector3D::new(0.0, 0.0, 0.0),
        Vector3D::new(0.0, 1.0, 0.0),
        90.0,
        width as f32 / height as f32,
    );
    let mut scene1 = Scene::new();
    scene1.add_object(Box::new(Sphere::new(Vector3D::new(0.0, 0.0, 0.0), 1.0)));
    scene1.add_light(Light::new(Vector3D::new(-5.0, 5.0, -5.0), Color::new(1.0, 1.0, 1.0), 1.0));
    render_scene(&scene1, &camera1, width, height, "sphere_scene.ppm");

    // Scene 2: A flat plane and a cube with lower brightness (modified)
    let camera2 = Camera::new(
        Vector3D::new(0.0, 4.0, -10.0),
        Vector3D::new(0.0, 0.0, 0.0),
        Vector3D::new(0.0, 1.0, 0.0),
        90.0,
        width as f32 / height as f32,
    );
    let mut scene2 = Scene::new();
    scene2.add_object(Box::new(Plane::new(Vector3D::new(0.0, -2.0, 0.0), Vector3D::new(0.0, 1.0, 0.0))));
    scene2.add_object(Box::new(Cube::new(
        Vector3D::new(-1.0, -1.0, -1.0),
        Vector3D::new(1.0, 1.0, 1.0)
    )));
    scene2.add_light(Light::new(Vector3D::new(-10.0, 10.0, -10.0), Color::new(0.5, 0.5, 0.5), 1.0));
    render_scene(&scene2, &camera2, width, height, "plane_cube_scene.ppm");

    // Scene 3: One of each object (modified)
    let camera3 = Camera::new(
        Vector3D::new(0.0, 5.0, -15.0),
        Vector3D::new(0.0, 0.0, 0.0),
        Vector3D::new(0.0, 1.0, 0.0),
        90.0,
        width as f32 / height as f32,
    );
    let mut scene3 = Scene::new();
    scene3.add_object(Box::new(Sphere::new(Vector3D::new(-4.0, 0.0, 2.0), 1.5)));
    scene3.add_object(Box::new(Cube::new(
        Vector3D::new(3.0, -1.0, -1.0),
        Vector3D::new(5.0, 1.0, 1.0)
    )));
    scene3.add_object(Box::new(Cylinder::new(
        Vector3D::new(0.0, -2.0, -3.0),
        Vector3D::new(0.0, 1.0, 0.0),
        0.75,
        3.0
    )));
    scene3.add_object(Box::new(Plane::new(Vector3D::new(0.0, -2.0, 0.0), Vector3D::new(0.0, 1.0, 0.0))));
    scene3.add_light(Light::new(Vector3D::new(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0), 1.0));
    render_scene(&scene3, &camera3, width, height, "all_objects_scene.ppm");

    // Scene 4: Same as scene 3 but with different camera position (modified)
    let camera4 = Camera::new(
        Vector3D::new(10.0, 8.0, -10.0),
        Vector3D::new(0.0, 0.0, 0.0),
        Vector3D::new(0.0, 1.0, 0.0),
        90.0,
        width as f32 / height as f32,
    );
    render_scene(&scene3, &camera4, width, height, "all_objects_different_perspective.ppm");

    println!("All scenes rendered successfully!");
}

fn render_scene(scene: &Scene, camera: &Camera, width: u32, height: u32, filename: &str) {
    let renderer = Renderer::new(width, height);
    let image = renderer.render(scene, camera);
    save_image(&image, width, height, filename);
    println!("Rendered scene saved as {}", filename);
}

fn save_image(image: &[Color], width: u32, height: u32, filename: &str) {
    let mut file = File::create(filename).expect("Failed to create file");

    writeln!(file, "P3").expect("Failed to write PPM header");
    writeln!(file, "{} {}", width, height).expect("Failed to write dimensions");
    writeln!(file, "255").expect("Failed to write max color value");

    for y in 0..height {
        for x in 0..width {
            let index = ((height - 1 - y) * width + x) as usize; // Inversion des coordonnées y
            let color = &image[index];
            let r = (color.r.min(1.0).max(0.0) * 100.0) as u8;
            let g = (color.g.min(1.0).max(0.0) * 200.0) as u8;
            let b = (color.b.min(1.0).max(0.0) * 255.0) as u8;
            write!(file, "{} {} {} ", r, g, b).expect("Failed to write pixel data");
        }
        writeln!(file).expect("Failed to write newline");
    }
}
