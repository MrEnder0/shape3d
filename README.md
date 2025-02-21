
# Shape3D

Shape3D is an application for visualizing and manipulating 3D shapes. It provides various tools and features to interact with 3D objects, including rotation, scaling, and rendering modes.

## Features

- **3D Shape Visualization**: View and interact with 3D shapes in real-time.
- **Rotation and Scaling**: Rotate and scale shapes using intuitive controls.
- **Rendering Modes**: Multiple rendering modes including points, pre-defined lines, dynamic lines, and experimental rendering.
- **Shape Modification**: Add, remove, and modify points of the shapes.
- **Plugin Support**: Load and use dynamic plugins to extend functionality.
- **Autosave**: Automatically saves the current shape state on exit and loads it on startup.

## Getting Started

1. **Clone the repository**:
    ```sh
    git clone https://github.com/yourusername/shape3d.git
    cd shape3d
    ```

2. **Build the project**:
    ```sh
    just
    ```

3. **Run the application**:
    ```sh
    ./shape3d
    ```

## Usage

- **Rotation**: Use arrow keys and Q/E to rotate the shape.
- **Zoom**: Use the scroll wheel to zoom in and out.
- **Rendering Modes**: Select different rendering modes from the options menu.
- **Shape Modification**: Use the shape modifier window to add, remove, and modify points.

## Plugin Development

To develop a plugin for Shape3D, follow these steps:

1. Create a new Rust library project.
2. Implement the required functions and export them using `#[no_mangle]`.
3. Write a linker function inside the plugins.rs to explain input and outputs
4. Compile the library and place the resulting `.dll` or `.so` file in the appropriate directory.
5. Shape3D will automatically detect and load the plugin if it meets the required interface.
