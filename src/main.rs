use std::env;
use std::rc::Rc;

use rt::{
    bvh::BVHNode,
    camera::Camera,
    color::Color,
    cylinder::Cylinder,
    hittable::{Hittable, RotateX, RotateY, RotateZ, Translate},
    hittable_list::HittableList,
    material::{
        create_light_material, create_standard_glass, create_standard_material,
        create_standard_metal, LightColor, StandardColor, StandardGlasses,
        StandardMetal,
    },
    quad::{box_, Quad},
    sphere::Sphere,
    Point3, Vector3,
};

fn main() {
    // Image settings
    const ASPECT_RATIO: f64 = 4.0 / 3.0;
    const IMAGE_WIDTH: usize = 800;

    // Camera settings
    let mut cam = Camera::new(ASPECT_RATIO, IMAGE_WIDTH);
    cam.samples_per_pixel = 1000;
    cam.max_depth = 20;
    cam.background = Color::new(0.0, 0.0, 0.0);
    cam.brightness = 1.0;
    cam.vfov = 40.0;
    cam.defocus_angle = 0.0;
    cam.focus_dist = 10.0;
    cam.lookfrom = Point3::new(378.0, 278.0, -800.0);
    cam.lookat = Point3::new(378.0, 278.0, 0.0);
    cam.vup = Vector3::new(0.0, 1.0, 0.0);

    // Scene initialization
    let mut world: HittableList = HittableList::new();
    cornell_box(&mut world);

    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => {
            custom_scene(&mut world);
        }
        2 => match args[1].as_str() {
            "--first" | "-f" => {
                first_scene(&mut world);
            }
            "--second" | "-s" => {
                second_scene(&mut world);
                cam.brightness = 0.5;
            }
            "--third" | "-t" => {
                third_scene(&mut world);
            }

            "--fourth" | "-fo" => {
                third_scene(&mut world);
                cam.vfov = 50.0;
                cam.lookfrom = Point3::new(755.0, 555.0, 555.0);
                cam.lookat = Point3::new(0.0, 0.0, 0.0);
                cam.vup = Vector3::new(0.0, 1.0, 0.0);
            }
            _ => {
                eprintln!("Invalid scene name");
                std::process::exit(1);
            }
        },
        _ => {
            eprintln!("Invalid number of arguments");
            std::process::exit(1);
        }
    }

    // Setting
    let bvh_world = BVHNode::new_from_list(&world);
    cam.render(&bvh_world);
}

fn cornell_box(world: &mut HittableList) {
    let red = create_standard_material(StandardColor::Red);
    let green = create_standard_material(StandardColor::Green);
    let white = create_standard_material(StandardColor::White);
    let light = create_light_material(LightColor::White);

    // Box
    world.add(Rc::new(Quad::new(
        Point3::new(755.0, 0.0, 0.0),
        Vector3::new(0.0, 555.0, 0.0),
        Vector3::new(0.0, 0.0, 555.0),
        red,
    )));
    world.add(Rc::new(Quad::new(
        Point3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 555.0, 0.0),
        Vector3::new(0.0, 0.0, 555.0),
        green,
    )));

    world.add(Rc::new(Quad::new(
        Point3::new(0.0, 0.0, 0.0),
        Vector3::new(755.0, 0.0, 0.0),
        Vector3::new(0.0, 0.0, 555.0),
        white.clone(),
    )));
    world.add(Rc::new(Quad::new(
        Point3::new(755.0, 555.0, 555.0),
        Vector3::new(-755.0, 0.0, 0.0),
        Vector3::new(0.0, 0.0, -555.0),
        white.clone(),
    )));
    world.add(Rc::new(Quad::new(
        Point3::new(0.0, 0.0, 555.0),
        Vector3::new(755.0, 0.0, 0.0),
        Vector3::new(0.0, 555.0, 0.0),
        white.clone(),
    )));

    // Light
    world.add(Rc::new(Quad::new(
        Point3::new(625.0, 554.0, 332.0),
        Vector3::new(-500.0, 0.0, 0.0),
        Vector3::new(0.0, 0.0, -105.0),
        light,
    )));
}

fn custom_scene(world: &mut HittableList) {
    let glass = create_standard_glass(StandardGlasses::Glass);
    let diamond = create_standard_glass(StandardGlasses::Diamond);
    let gold = create_standard_metal(StandardMetal::Gold, 0.3);
    let silver = create_standard_metal(StandardMetal::Silver, 0.5);
    let aluminium = create_standard_metal(StandardMetal::Aluminium, 0.1);
    let copper = create_standard_metal(StandardMetal::Copper, 0.8);
    let blue = create_standard_material(StandardColor::Blue);
    let light = create_light_material(LightColor::White);
    let yellow = create_standard_material(StandardColor::Yellow);
    let purple = create_standard_material(StandardColor::Purple);

    // Spheres
    world.add(Rc::new(Sphere::new(
        Point3::new(200.0, 100.0, 300.0),
        100.0,
        aluminium.clone(),
    )));
    world.add(Rc::new(Sphere::new(
        Point3::new(375.0, 150.0, 300.0),
        50.0,
        diamond,
    )));

    world.add(Rc::new(Sphere::new(
        Point3::new(600.0, 75.0, 400.0),
        75.0,
        glass,
    )));
    world.add(Rc::new(Sphere::new(
        Point3::new(150.0, 10.0, 150.0),
        10.0,
        light,
    )));
    world.add(Rc::new(Sphere::new(
        Point3::new(700.0, 20.0, 50.0),
        20.0,
        silver,
    )));
    world.add(Rc::new(Sphere::new(
        Point3::new(100.0, 50.0, 200.0),
        50.0,
        copper,
    )));
    world.add(Rc::new(Sphere::new(
        Point3::new(280.0, 25.0, 50.0),
        25.0,
        yellow,
    )));


    // Cubes
    let mut cube1: Rc<dyn Hittable> = box_(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(100.0, 100.0, 100.0),
        gold,
    );
    cube1 = Rc::new(RotateY::new(cube1, 45.0));
    cube1 = Rc::new(Translate::new(cube1, Vector3::new(300.0, 0.0, 300.0)));
    world.add(cube1);

    let mut cube2: Rc<dyn Hittable> = box_(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(50.0, 50.0, 50.0),
        aluminium.clone(),
    );
    cube2 = Rc::new(RotateY::new(cube2, 30.0));
    cube2 = Rc::new(Translate::new(cube2, Vector3::new(500.0, 0.0, 100.0)));
    world.add(cube2);

    let mut cube3: Rc<dyn Hittable> = box_(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(50.0, 100.0, 50.0),
        blue,
    );
    cube3 = Rc::new(RotateY::new(cube3, 30.0));
    cube3 = Rc::new(Translate::new(cube3, Vector3::new(500.0, 0.0, 100.0)));
    world.add(cube3);

    

    // Cylinders
    let cylinder1: Rc<dyn Hittable> = Rc::new(Cylinder::new(
        Point3::new(650.0, 0.0, 150.0),
        200.0,
        20.0,
        purple,
    ));
    world.add(cylinder1);

    let mut cylinder2: Rc<dyn Hittable> = Rc::new(Cylinder::new(
        Point3::new(0.0, 0.0, 0.0),
        100.0,
        25.0,
        aluminium.clone(),
    ));

    cylinder2 = Rc::new(RotateX::new(cylinder2, 90.0));
    cylinder2 = Rc::new(RotateY::new(cylinder2, 45.0));
    cylinder2 = Rc::new(Translate::new(cylinder2, Vector3::new(400.0, 25.0, 100.0)));
    world.add(cylinder2);

}

fn first_scene(world: &mut HittableList) {
    // Sphere
    world.add(Rc::new(Sphere::new(
        Point3::new(400.0, 90.0, 190.0),
        90.0,
        create_standard_material(StandardColor::Grey),
    )));
}

fn second_scene(world: &mut HittableList) {
    let grey = create_standard_material(StandardColor::Grey);
    let blue = create_standard_material(StandardColor::Blue);

    // Flat plane
    let mut plane: Rc<dyn Hittable> = Rc::new(Quad::new(
        Point3::new(-100.0, 50.0, 0.0),
        Vector3::new(200.0, 0.0, 0.0),
        Vector3::new(0.0, 200.0, 0.0),
        blue,
    ));
    plane = Rc::new(RotateY::new(plane, -75.0));
    plane = Rc::new(RotateX::new(plane, 15.0));
    plane = Rc::new(RotateZ::new(plane, -30.0));

    plane = Rc::new(Translate::new(plane, Vector3::new(200.0, 100.0, 200.0)));
    world.add(plane);

    // Cube
    let mut cube: Rc<dyn Hittable> = box_(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 165.0, 165.0),
        grey,
    );
    cube = Rc::new(RotateY::new(cube, 15.0));
    cube = Rc::new(Translate::new(cube, Vector3::new(330.0, 0.0, 255.0)));
    world.add(cube);
}

fn third_scene(world: &mut HittableList) {
    let grey = create_standard_material(StandardColor::Grey);
    let blue = create_standard_material(StandardColor::Blue);
    let yellow = create_standard_material(StandardColor::Yellow);
    let purple = create_standard_material(StandardColor::Purple);

    // Sphere
    world.add(Rc::new(Sphere::new(
        Point3::new(600.0, 90.0, 100.0),
        90.0,
        grey,
    )));

    // Cube
    let mut cube: Rc<dyn Hittable> = box_(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 165.0, 165.0),
        blue,
    );
    cube = Rc::new(RotateY::new(cube, 15.0));
    cube = Rc::new(Translate::new(cube, Vector3::new(330.0, 0.0, 255.0)));
    world.add(cube);

    // Cylinder
    let cylinder: Rc<dyn Hittable> = Rc::new(Cylinder::new(
        Point3::new(300.0, 0.0, 50.0),
        200.0,
        30.0,
        yellow,
    ));
    world.add(cylinder);

    // Flat plane
    let mut plane: Rc<dyn Hittable> = Rc::new(Quad::new(
        Point3::new(-100.0, 50.0, 0.0),
        Vector3::new(200.0, 0.0, 0.0),
        Vector3::new(0.0, 200.0, 0.0),
        purple,
    ));
    plane = Rc::new(RotateY::new(plane, -75.0));
    plane = Rc::new(RotateX::new(plane, 15.0));
    plane = Rc::new(RotateZ::new(plane, -30.0));
    plane = Rc::new(Translate::new(plane, Vector3::new(200.0, 100.0, 200.0)));
    world.add(plane);
}
