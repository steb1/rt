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

use clap::{App, Arg};
use image::{ImageBuffer, Rgb};

fn main() {
    let matches = App::new("Ray Tracer")
        .version("1.0")
        .author("Your Name")
        .about("A simple ray tracer")
        .arg(Arg::new("camera_x")
            .long("camera-x")
            .value_name("FLOAT")
            .default_value("0.0")
            .allow_hyphen_values(true)
            .help("Camera X position"))
        .arg(Arg::new("camera_y")
            .long("camera-y")
            .value_name("FLOAT")
            .default_value("0.0")
            .allow_hyphen_values(true)
            .help("Camera Y position"))
        .arg(Arg::new("camera_z")
            .long("camera-z")
            .value_name("FLOAT")
            .default_value("-5.0")
            .allow_hyphen_values(true)
            .help("Camera Z position"))
        .arg(Arg::new("light_x")
            .long("light-x")
            .value_name("FLOAT")
            .default_value("-5.0")
            .allow_hyphen_values(true)
            .help("Light X position"))
        .arg(Arg::new("light_y")
            .long("light-y")
            .value_name("FLOAT")
            .default_value("5.0")
            .allow_hyphen_values(true)
            .help("Light Y position"))
        .arg(Arg::new("light_z")
            .long("light-z")
            .value_name("FLOAT")
            .default_value("-5.0")
            .allow_hyphen_values(true)
            .help("Light Z position"))
        .arg(Arg::new("sphere_x")
            .long("sphere-x")
            .value_name("FLOAT")
            .default_value("-4.0")
            .allow_hyphen_values(true)
            .help("Sphere X position"))
        .arg(Arg::new("sphere_y")
            .long("sphere-y")
            .value_name("FLOAT")
            .default_value("-0.8")
            .allow_hyphen_values(true)
            .help("Sphere Y position"))
        .arg(Arg::new("sphere_z")
            .long("sphere-z")
            .value_name("FLOAT")
            .default_value("0.0")
            .allow_hyphen_values(true)
            .help("Sphere Z position"))
        .arg(Arg::new("cylinder_x")
            .long("cylinder-x")
            .value_name("FLOAT")
            .default_value("0.0")
            .allow_hyphen_values(true)
            .help("Cylinder X position"))
        .arg(Arg::new("cylinder_y")
            .long("cylinder-y")
            .value_name("FLOAT")
            .default_value("-1.0")
            .allow_hyphen_values(true)
            .help("Cylinder Y position"))
        .arg(Arg::new("cylinder_z")
            .long("cylinder-z")
            .value_name("FLOAT")
            .default_value("0.0")
            .allow_hyphen_values(true)
            .help("Cylinder Z position"))
        .arg(Arg::new("cube_x")
            .long("cube-x")
            .value_name("FLOAT")
            .default_value("4.0")
            .allow_hyphen_values(true)
            .help("Cube X position"))
        .arg(Arg::new("cube_y")
            .long("cube-y")
            .value_name("FLOAT")
            .default_value("-1.0")
            .allow_hyphen_values(true)
            .help("Cube Y position"))
        .arg(Arg::new("cube_z")
            .long("cube-z")
            .value_name("FLOAT")
            .default_value("-0.5")
            .allow_hyphen_values(true)
            .help("Cube Z position"))
        .arg(Arg::new("output")
            .short('o')
            .long("output")
            .value_name("FILE")
            .default_value("output.png")
            .help("Output file name"))
        .get_matches();

    let camera_x = matches.value_of("camera_x").unwrap().parse::<f32>().unwrap();
    let camera_y = matches.value_of("camera_y").unwrap().parse::<f32>().unwrap();
    let camera_z = matches.value_of("camera_z").unwrap().parse::<f32>().unwrap();

    let light_x = matches.value_of("light_x").unwrap().parse::<f32>().unwrap();
    let light_y = matches.value_of("light_y").unwrap().parse::<f32>().unwrap();
    let light_z = matches.value_of("light_z").unwrap().parse::<f32>().unwrap();

    let sphere_x = matches.value_of("sphere_x").unwrap().parse::<f32>().unwrap();
    let sphere_y = matches.value_of("sphere_y").unwrap().parse::<f32>().unwrap();
    let sphere_z = matches.value_of("sphere_z").unwrap().parse::<f32>().unwrap();

    let cylinder_x = matches.value_of("cylinder_x").unwrap().parse::<f32>().unwrap();
    let cylinder_y = matches.value_of("cylinder_y").unwrap().parse::<f32>().unwrap();
    let cylinder_z = matches.value_of("cylinder_z").unwrap().parse::<f32>().unwrap();

    let cube_x = matches.value_of("cube_x").unwrap().parse::<f32>().unwrap();
    let cube_y = matches.value_of("cube_y").unwrap().parse::<f32>().unwrap();
    let cube_z = matches.value_of("cube_z").unwrap().parse::<f32>().unwrap();

    let output_file = matches.value_of("output").unwrap();

    let width = 800;
    let height = 600;

    let camera = Camera::new(
        Vector3D::new(camera_x, camera_y, camera_z),
        Vector3D::new(0.0, 0.0, 0.0),
        Vector3D::new(0.0, 1.0, 0.0),
        90.0,
        width as f32 / height as f32,
    );

    let mut scene = Scene::new();

    scene.add_object(Box::new(Sphere::new(Vector3D::new(sphere_x, sphere_y + 1.0, sphere_z), 1.0)));
    scene.add_object(Box::new(Plane::new(Vector3D::new(0.0, -1.0, 0.0), Vector3D::new(0.0, 1.0, 0.0))));
    scene.add_object(Box::new(Cylinder::new(
        Vector3D::new(cylinder_x, cylinder_y, cylinder_z),
        Vector3D::new(0.0, 1.0, 0.0),
        0.5,
        2.0
    )));
    scene.add_object(Box::new(Cube::new(
        Vector3D::new(cube_x, cube_y, cube_z),
        Vector3D::new(cube_x + 1.0, cube_y + 1.0, cube_z + 1.0)
    )));

    scene.add_light(Light::new(Vector3D::new(light_x, light_y, light_z), Color::new(1.0, 1.0, 1.0), 1.0));

    let renderer = Renderer::new(width, height);
    let image = renderer.render(&scene, &camera);

    save_image(&image, width, height, output_file);
    println!("Rendering complete! Image saved as {}", output_file);
}

fn save_image(image: &[Color], width: u32, height: u32, filename: &str) {
    let mut imgbuf = ImageBuffer::new(width, height);

    for y in 0..height {
        for x in 0..width {
            let index = ((height - 1 - y) * width + x) as usize; // Inversion des coordonnées y
            let color = &image[index];
            let pixel = imgbuf.get_pixel_mut(x, y);
            *pixel = Rgb([
                (color.r.min(1.0).max(0.0) * 255.0) as u8,
                (color.g.min(1.0).max(0.0) * 255.0) as u8,
                (color.b.min(1.0).max(0.0) * 255.0) as u8,
            ]);
        }
    }

    imgbuf.save(filename).expect("Failed to save image");
}
