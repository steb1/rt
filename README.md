Certainly! Here's a documentation explaining how to use this ray tracing project:

# Ray Tracer Project Documentation

## Overview

This ray tracer is a Rust-based command-line application that generates 3D rendered images. It allows you to create scenes with various objects (spheres, planes, cylinders, and cubes) and customize their positions, as well as adjust the camera and light source positions.

## Installation

1. Ensure you have Rust and Cargo installed on your system. If not, install them from [https://www.rust-lang.org/](https://www.rust-lang.org/).

2. Clone the repository or download the source code.

3. Navigate to the project directory in your terminal.

4. Build the project by running:
   ```
   cargo build --release
   ```

## Usage

To run the ray tracer, use the following command structure:

```
cargo run --release -- [OPTIONS]
```

### Options

The ray tracer accepts the following command-line options to customize the scene:

- `--camera-x <FLOAT>`: Camera X position (default: 0.0)
- `--camera-y <FLOAT>`: Camera Y position (default: 0.0)
- `--camera-z <FLOAT>`: Camera Z position (default: -5.0)
- `--light-x <FLOAT>`: Light X position (default: -5.0)
- `--light-y <FLOAT>`: Light Y position (default: 5.0)
- `--light-z <FLOAT>`: Light Z position (default: -5.0)
- `--sphere-x <FLOAT>`: Sphere X position (default: 0.0)
- `--sphere-y <FLOAT>`: Sphere Y position (default: 0.0)
- `--sphere-z <FLOAT>`: Sphere Z position (default: 0.0)
- `--cylinder-x <FLOAT>`: Cylinder X position (default: -2.0)
- `--cylinder-y <FLOAT>`: Cylinder Y position (default: 0.0)
- `--cylinder-z <FLOAT>`: Cylinder Z position (default: 0.0)
- `--cube-x <FLOAT>`: Cube X position (default: 1.0)
- `--cube-y <FLOAT>`: Cube Y position (default: -0.5)
- `--cube-z <FLOAT>`: Cube Z position (default: -0.5)
- `-o, --output <FILE>`: Output file name (default: "output.png")

### Examples

1. Render a scene with default settings:
   ```
   cargo run --release
   ```

2. Move the sphere to position (1, 1, 1):
   ```
   cargo run --release -- --sphere-x 1.0 --sphere-y 1.0 --sphere-z 1.0
   ```

3. Adjust camera position and save to a custom file:
   ```
   cargo run --release -- --camera-x 2.0 --camera-y 1.0 --camera-z -4.0 --output my_scene.png
   ```

4. Combine multiple object position changes:
   ```
   cargo run --release -- --sphere-x 1.0 --sphere-y 1.0 --sphere-z 1.0 --cylinder-x -1.0 --cylinder-y 0.5 --cube-x 2.0 --cube-y 0.0 --cube-z 1.0
   ```

## Scene Description

The default scene consists of:
- A sphere (radius: 1.0)
- A plane (positioned at y = -1.0, facing upwards)
- A cylinder (radius: 0.5, height: 2.0, oriented along the y-axis)
- A cube (1x1x1 units in size)

You can adjust the positions of these objects using the command-line options.

## Output

The rendered image will be saved as a PNG file. By default, it's named "output.png", but you can specify a custom name using the `--output` option.

## Troubleshooting

If you encounter issues with negative values in command-line arguments, ensure you're using the latest version of the code that includes `.allow_hyphen_values(true)` for each argument definition.

If you experience any other issues or have questions, please open an issue in the project repository.

## Extending the Project

To add new features or modify existing ones:

1. Object types: Add new structs in the `objects` module and implement the `Object` trait for them.
2. Rendering features: Modify the `Renderer` struct in the `renderer` module.
3. Scene composition: Adjust the scene setup in the `main` function.

Remember to update this documentation if you make significant changes to the project structure or usage.