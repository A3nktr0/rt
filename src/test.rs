extern crate rt;
use rt::bvh::BVHNode;
use rt::camera::Camera;
use rt::color::Color;
use rt::cylinder::Cylinder;
use rt::hittable::{Hittable, RotateY, Translate};
use rt::hittable_list::HittableList;
use rt::material::{Dielectric, DiffuseLight, Lambertian, Metal};
use rt::na::Point3;
use rt::quad::{box_, Quad};
use rt::sphere::Sphere;
use rt::Vector3;
use std::rc::Rc;

fn main() {
    // World
    let mut world = HittableList::new();

    // let material_ground = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    // let material_center = Rc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    // // let material_left = Rc::new(Metal::new(Color::new(0.8, 0.8, 0.8), 0.3));
    // let material_left = Rc::new(Dielectric::new(1.50));
    // let material_bubble = Rc::new(Dielectric::new(1.00 / 1.50));
    // let material_right = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 1.0));

    // world.add(Rc::new(Sphere::new(
    //     Point3::new(0.0, -100.5, -1.0),
    //     100.0,
    //     material_ground,
    // )));
    // world.add(Rc::new(Sphere::new(
    //     Point3::new(0.0, 0.0, -1.2),
    //     0.5,
    //     material_center,
    // )));
    // world.add(Rc::new(Sphere::new(
    //     Point3::new(-1.0, 0.0, -1.0),
    //     0.5,
    //     material_left,
    // )));
    // world.add(Rc::new(Sphere::new(
    //     Point3::new(-1.0, 0.0, -1.0),
    //     0.4,
    //     material_bubble,
    // )));
    // world.add(Rc::new(Sphere::new(
    //     Point3::new(1.0, 0.0, -1.0),
    //     0.5,
    //     material_right,
    // )));

    // let left_red = Rc::new(Lambertian::new(Color::new(1.0, 0.2, 0.2)));
    // let back_green = Rc::new(Lambertian::new(Color::new(0.2, 1.0, 0.2)));
    // let right_blue = Rc::new(Lambertian::new(Color::new(0.2, 0.2, 1.0)));
    // let upper_orange = Rc::new(Lambertian::new(Color::new(1.0, 0.5, 0.0)));
    // let lower_teal = Rc::new(Lambertian::new(Color::new(0.2, 0.8, 0.8)));

    // world.add(Rc::new(Quad::new(
    //     Point3::new(-3.0, -2.0, 5.0),
    //     Vector3::new(0.0, 0.0, -4.0),
    //     Vector3::new(0.0, 4.0, 0.0),
    //     left_red.clone(),
    // )));

    // world.add(Rc::new(Quad::new(
    //     Point3::new(-2.0, -2.0, 0.0),
    //     Vector3::new(4.0, 0.0, 0.0),
    //     Vector3::new(0.0, 4.0, 0.0),
    //     back_green.clone(),
    // )));

    // world.add(Rc::new(Quad::new(
    //     Point3::new(3.0, -2.0, 1.0),
    //     Vector3::new(0.0, 0.0, 4.0),
    //     Vector3::new(0.0, 4.0, 0.0),
    //     right_blue.clone(),
    // )));

    // world.add(Rc::new(Quad::new(
    //     Point3::new(-2.0, 3.0, 1.0),
    //     Vector3::new(4.0, 0.0, 0.0),
    //     Vector3::new(0.0, 0.0, 4.0),
    //     upper_orange.clone(),
    // )));

    // world.add(Rc::new(Quad::new(
    //     Point3::new(-2.0, -3.0, 5.0),
    //     Vector3::new(4.0, 0.0, 0.0),
    //     Vector3::new(0.0, 0.0, -4.0),
    //     lower_teal.clone(),
    // )));

    // world.add(Rc::new(Sphere::new(
    //     Point3::new(0.0, -100.5, -1.0),
    //     100.0,
    //     material_ground,
    // )));
    // world.add(Rc::new(Sphere::new(
    //     Point3::new(0.0, 0.0, -1.2),
    //     0.5,
    //     material_center,
    // )));

    // let difflight = Rc::new(DiffuseLight::new(Color::new(4.0, 4.0, 4.0)));

    // world.add(Rc::new(Sphere::new(
    //     Point3::new(0.0, 2.0, -1.2),
    //     0.5,
    //     difflight.clone(),
    // )));
    // world.add(Rc::new(Quad::new(
    //     Point3::new(1.0, -0.5, -2.0),
    //     Vector3::new(0.0, 0.0, 1.0),
    //     Vector3::new(0.0, 1.0, 0.0),
    //     difflight,
    // )));

    // Define Cornell Box
    let red = Rc::new(Lambertian::new(Color::new(0.65, 0.05, 0.05)));
    let white = Rc::new(Lambertian::new(Color::new(0.73, 0.73, 0.73)));
    let green = Rc::new(Lambertian::new(Color::new(0.12, 0.45, 0.15)));
    let light = Rc::new(DiffuseLight::new(Color::new(30.0, 30.0, 30.0)));
    world.add(Rc::new(Quad::new(
        Point3::new(555.0, 0.0, 0.0),
        Vector3::new(0.0, 555.0, 0.0),
        Vector3::new(0.0, 0.0, 555.0),
        red.clone(),
    )));
    world.add(Rc::new(Quad::new(
        Point3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 555.0, 0.0),
        Vector3::new(0.0, 0.0, 555.0),
        green.clone(),
    )));
    world.add(Rc::new(Quad::new(
        Point3::new(343.0, 554.0, 332.0),
        Vector3::new(-130.0, 0.0, 0.0),
        Vector3::new(0.0, 0.0, -105.0),
        light.clone(),
    )));
    world.add(Rc::new(Quad::new(
        Point3::new(0.0, 0.0, 0.0),
        Vector3::new(555.0, 0.0, 0.0),
        Vector3::new(0.0, 0.0, 555.0),
        white.clone(),
    )));
    world.add(Rc::new(Quad::new(
        Point3::new(555.0, 555.0, 555.0),
        Vector3::new(-555.0, 0.0, 0.0),
        Vector3::new(0.0, 0.0, -555.0),
        white.clone(),
    )));
    world.add(Rc::new(Quad::new(
        Point3::new(0.0, 0.0, 555.0),
        Vector3::new(555.0, 0.0, 0.0),
        Vector3::new(0.0, 555.0, 0.0),
        white.clone(),
    )));

    // world.add(box_(Point3::new(130.0, 0.0, 65.0), Point3::new(295.0, 165.0, 230.0), white.clone()));
    // world.add(box_(Point3::new(265.0, 0.0, 295.0), Point3::new(430.0, 330.0, 460.0), white.clone()));

    // world.add(Rc::new(Sphere::new(
    //     Point3::new(250.0, 90.0, 190.0),
    //     90.0,
    //     Rc::new(Dielectric::new(1.5)),
    // )));

    let aluminium = Rc::new(Metal::new(Color::new(0.8, 0.85, 0.88), 0.0));

    let mut box1: Rc<dyn Hittable> = box_(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 330.0, 165.0),
        aluminium.clone(),
    );
    box1 = Rc::new(RotateY::new(box1, 15.0));
    box1 = Rc::new(Translate::new(box1, Vector3::new(245.0, 0.0, 295.0)));
    world.add(box1);

    let mut box2: Rc<dyn Hittable> = box_(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 165.0, 165.0),
        aluminium,
    );
    box2 = Rc::new(RotateY::new(box2, -18.0));
    box2 = Rc::new(Translate::new(box2, Vector3::new(130.0, 0.0, 65.0)));
    world.add(box2);

    // let glass = Rc::new(Dielectric::new(1.5));
    // world.add(Rc::new(Sphere::new(
    //     Point3::new(190.0, 90.0, 190.0),
    //     90.0,
    //     glass,
    // )));

    // let cylinder = Rc::new(Cylinder::new(
    //     Point3::new(130.0, 0.0, 130.0),
    //     100.0,
    //     50.0,
    //     Rc::new(Lambertian::new(Color::new(0.0, 0.0, 1.0))),
    // ));
    // world.add(cylinder);

    let bvh_world = BVHNode::new_from_list(&world);

    // Image
    // const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const ASPECT_RATIO: f64 = 1.0;
    const IMG_WIDTH: usize = 600;

    // Camera
    let mut cam = Camera::new(ASPECT_RATIO, IMG_WIDTH);
    cam.samples_per_pixel = 100;
    cam.max_depth = 20;
    // cam.background = Color::new(0.70, 0.80, 1.0);
    cam.background = Color::new(0.0, 0.0, 0.0);
    cam.vfov = 40.0;
    cam.defocus_angle = 0.0;
    cam.focus_dist = 10.0;

    cam.lookfrom = Point3::new(278.0, 278.0, -800.0);
    cam.lookat = Point3::new(278.0, 278.0, 0.0);
    cam.vup = Vector3::new(0.0, 1.0, 0.0);

    // cam.lookfrom = Point3::new(-2.0, 2.0, 1.0);
    // cam.lookat = Point3::new(0.0, 0.0, -1.0);
    // cam.vup = Vector3::new(0.0, 1.0, 0.0);

    // cam.lookfrom = Point3::new(0.0, 0.0, 0.0);
    // cam.lookat = Point3::new(0.0, 0.0, -1.0);
    // cam.vup = Vector3::new(0.0, 1.0, 0.0);

    // cam.lookfrom = Point3::new(0.0, 0.0, 9.0);
    // cam.lookat = Point3::new(0.0, 0.0, 0.0);
    // cam.vup = Vector3::new(0.0, 1.0, 0.0);

    cam.render(&bvh_world);
    // cam.render(&world);
}

// fn main(){
//     let mut world = HittableList::new();

//     let ground_material = Rc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
//     world.add(Rc::new(Sphere::new(
//         Point3::new(0.0, -1000.0, 0.0),
//         1000.0,
//         ground_material,
//     )));

//     for a in -11..11 {
//         for b in -11..11 {
//             let choose_mat = random_double();
//             let center = Point3::new(
//                 a as f64 + 0.9 * random_double(),
//                 0.2,
//                 b as f64 + 0.9 * random_double(),
//             );

//             if (center - Point3::new(4.0, 0.2, 0.0)).norm() > 0.9 {
//                 let sphere_material: Rc<dyn Material>;

//                 if choose_mat < 0.8 {
//                     // diffuse
//                     let albedo = Color::new(random_double(), random_double(), random_double());
//                     sphere_material = Rc::new(Lambertian::new(albedo));
//                     world.add(Rc::new(Sphere::new(center, 0.2, sphere_material)));
//                 } else if choose_mat < 0.95 {
//                     // metal
//                     let albedo = random_with_limits(0.5, 1.0);
//                     let fuzz = random_double_range(0.0, 0.5);
//                     sphere_material = Rc::new(Metal::new(albedo, fuzz));
//                     world.add(Rc::new(Sphere::new(center, 0.2, sphere_material)));
//                 } else {
//                     // glass
//                     sphere_material = Rc::new(Dielectric::new(1.5));
//                     world.add(Rc::new(Sphere::new(center, 0.2, sphere_material)));
//                 }
//             }
//         }
//     }

//     let material1 = Rc::new(Dielectric::new(1.5));
//     world.add(Rc::new(Sphere::new(
//         Point3::new(0.0, 1.0, 0.0),
//         1.0,
//         material1,
//     )));

//     let material2 = Rc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
//     world.add(Rc::new(Sphere::new(
//         Point3::new(-4.0, 1.0, 0.0),
//         1.0,
//         material2,
//     )));

//     let material3 = Rc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
//     world.add(Rc::new(Sphere::new(
//         Point3::new(4.0, 1.0, 0.0),
//         1.0,
//         material3,
//     )));

//     let bvh_world = BVHNode::new_from_list(&world);

//         const ASPECT_RATIO: f64 = 16.0 / 9.0;
//     const IMG_WIDTH: usize = 300;

//     let mut cam = Camera::new(ASPECT_RATIO, IMG_WIDTH);
//     cam.aspect_ratio = 16.0 / 9.0;
//     cam.img_width = 400;
//     cam.samples_per_pixel = 100;
//     cam.max_depth = 50;
//     cam.vfov = 20.0;
//     cam.lookfrom = Point3::new(13.0, 2.0, 3.0);
//     cam.lookat = Point3::new(0.0, 0.0, 0.0);
//     cam.vup = Vector3::new(0.0, 1.0, 0.0);
//     cam.defocus_angle = 0.6;
//     cam.focus_dist = 10.0;

//     cam.render(&bvh_world);
//     // cam.render(&world);

// }
