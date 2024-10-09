# Ray Tracer Documentation
### overview

This ray tracer allows you to create and render 3D scenes with various materials, shapes, and lighting. You can customize the scene by adding different elements, adjusting the camera settings, and changing the brightness.

### Usage

#### Creating Elements

##### Materials
You can create different types of materials using the provided functions:

- Standard Materials:
  `create_standard_material(color: StandardColor)`: Creates a standard material with the specified color.
    - `color`: The color of the material. Options: `Red`, `Green`, `Blue`, `White`, `Black`, `Grey`, `Yellow`, `Purple`.

*example*:
```rust
let white = create_standard_material(StandardColor::White);
```


- Metallic Materials:
`create_standard_metal(metal: StandardMetal, fuzz: f64)`: Creates a metallic material with the specified metal and fuzziness.
    - `metal`: The type of metal. Options: `Gold`, `Silver`, `Aluminium`, `Copper`.
    - `fuzz`: The amount of randomness in the reflection of light on the metallic surface. A lower value produces a more focused reflection, while a higher value creates a more scattered reflection.

*example*:
```rust
let gold = create_standard_metal(StandardMetal::Gold, 0.3);
```

- Glass Materials:
    `create_standard_glass(glass: StandardGlasses)`: Creates a glass material with the specified glass type.
    - `glass`: The type of glass. Options: `Glass`, `Diamond`, `Water`.

*example*:
```rust
let glass = create_standard_glass(StandardGlasses::Glass);
```

- Light Materials:
  `create_light_material(color: LightColor)`: Creates a light material with the specified color.
    - `color`: The color of the light source. Options: `White`, `Yellow`, `Red`, `Green`, `Blue`, `Purple`.

*example*:
```rust
let light = create_light_material(LightColor::White);
```

##### Shapes
You can add different shapes to the scene:

- Spheres:
`Sphere::new(center: Point3, radius: f64, material: Material)`: Creates a sphere with the specified center, radius, and material.
    - `center`: The center of the sphere in 3D space.
    - `radius`: The radius of the sphere.
    - `material`: The material assigned to the sphere.

*example*:
```rust
world.add(Rc::new(Sphere::new(Point3::new(200.0, 100.0, 300.0), 100.0, glass)));
```

- Cubes:
`box_(min: Point3, max: Point3, material: Material)`: Creates a cube with the specified minimum point, maximum point, and material.
    - `min`: The minimum point of the cube in 3D space.
    - `max`: The maximum point of the cube in 3D space.
    - `material`: The material assigned to the cube.

*example*:
```rust
let cube = box_(Point3::new(0.0, 0.0, 0.0), Point3::new(100.0, 100.0, 100.0), red);
world.add(Rc::new(cube));
```

- Cylinders:
  `Cylinder::new(base_center: Point3, height: f64, radius: f64, material: Material)`: Creates a cylinder with the specified base center, height, radius, and material.
    - `base_center`: The center of the base of the cylinder in 3D space.
    - `height`: The height of the cylinder.
    - `radius`: The radius of the cylinder.
    - `material`: The material assigned to the cylinder.

*example*:
```rust
let cylinder = Rc::new(Cylinder::new(Point3::new(100.0, 0.0, 100.0), 200.0, 50.0, blue));
world.add(cylinder);
```

- Places:
    `Place::new(center: Point3, width: f64, height: f64, depth: f64, material: Material)`: Creates a place with the specified center, width, height, depth, and material.
    - `center`: The center of the place in 3D space.
    - `width`: The width of the place.
    - `height`: The height of the place.
    - `depth`: The depth of the place.
    - `material`: The material assigned to the place.

*example*:
```rust
let plane = Rc::new(Quad::new(Point3::new(0.0, 0.0, 0.0), Vector3::new(800.0, 0.0, 0.0), Vector3::new(0.0, 600.0, 0.0), green));
world.add(plane);
```

#### Setup the Camera
##### Changing Brightness:
You can adjust the brightness of the rendered image by setting the brightness property of the Camera object:

```rust
cam.brightness = 1.0; // Default brightness
cam.brightness = 0.5; // Dimmer
cam.brightness = 2.0; // Brighter
```

##### Moving the Camera

To change the position and orientation of the camera, you can modify the following properties:

- `lookfrom`: Sets the position of the camera in 3D space.
- `lookat`: Specifies the point that the camera is looking at.
- `vup`: Defines the "up" direction for the camera.

*example*:

```rust
cam.lookfrom = Point3::new(378.0, 278.0, -800.0); // Set the camera position
cam.lookat = Point3::new(378.0, 278.0, 0.0); // Set the point the camera is looking at
cam.vup = Vector3::new(0.0, 1.0, 0.0); // Set the "up" direction for the camera
```

### Example
Here is an example of setting up a simple scene with a red sphere and a blue plane:

```rust
fn main() {
    let mut cam = Camera::new(4.0 / 3.0, 800);
    cam.samples_per_pixel = 100;
    cam.max_depth = 20;
    cam.background = Color::new(0.0, 0.0, 0.0);
    cam.brightness = 1.0;
    cam.vfov = 40.0;
    cam.lookfrom = Point3::new(378.0, 278.0, -800.0);
    cam.lookat = Point3::new(378.0, 278.0, 0.0);
    cam.vup = Vector3::new(0.0, 1.0, 0.0);

    let mut world: HittableList = HittableList::new();
    let red = create_standard_material(StandardColor::Red);
    let blue = create_standard_material(StandardColor::Blue);

    world.add(Rc::new(Sphere::new(Point3::new(200.0, 100.0, 300.0), 100.0, red)));
    let plane = Rc::new(Quad::new(Point3::new(0.0, 0.0, 0.0), Vector3::new(800.0, 0.0, 0.0), Vector3::new(0.0, 600.0, 0.0), blue));
    world.add(plane);

    let bvh_world = BVHNode::new_from_list(&world);
    cam.render(&bvh_world);
}
```
This will render an image with a red sphere and a blue plane, with the camera positioned at (378.0, 278.0, -800.0) looking at (378.0, 278.0, 0.0).