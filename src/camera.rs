use std::io::Write;

use na::{Point3, Vector3};

use crate::{
    color::{write_color, Color},
    degrees_to_radians,
    hittable::{HitRecord, Hittable},
    interval::Interval,
    random_double,
    ray::Ray,
    vec3::random_in_unit_disk,
};

pub struct Camera {
    pub aspect_ratio: f64,
    pub img_width: usize,
    img_height: usize,

    camera_center: Point3<f64>,
    pixel_delta_horizontal: Vector3<f64>,
    pixel_delta_vertical: Vector3<f64>,
    pixel00_loc: Point3<f64>,
    pub samples_per_pixel: usize,
    pixel_samples_scale: f64,
    pub max_depth: usize,
    pub focal_length: f64,

    pub vfov: f64, // vertical field-of-view
    pub lookfrom: Point3<f64>,
    pub lookat: Point3<f64>,
    pub vup: Vector3<f64>,

    pub defocus_angle: f64, // Variation angle of rays through each pixel
    pub focus_dist: f64,    // Distance from the camera to the focus plane
    defocus_disk_horizontal: Vector3<f64>,
    defocus_disk_vertical: Vector3<f64>,

    u: Vector3<f64>,
    v: Vector3<f64>,
    w: Vector3<f64>,

    pub background: Color,
    pub brightness: f64,
}

impl Camera {
    pub fn new(aspect_ratio: f64, img_width: usize) -> Camera {
        Camera {
            aspect_ratio,
            img_width,
            img_height: ((img_width as f64 / aspect_ratio) as usize).max(1),
            camera_center: Point3::new(0.0, 0.0, 0.0),
            pixel_delta_horizontal: Vector3::new(0.0, 0.0, 0.0),
            pixel_delta_vertical: Vector3::new(0.0, 0.0, 0.0),
            pixel00_loc: Point3::new(0.0, 0.0, 0.0),
            samples_per_pixel: 10,
            pixel_samples_scale: 1.0,
            max_depth: 10,
            focal_length: 1.0,
            vfov: 90.0,
            lookfrom: Point3::new(0.0, 0.0, 0.0),
            lookat: Point3::new(0.0, 0.0, -1.0),
            vup: Vector3::new(0.0, 1.0, 0.0),

            defocus_angle: 0.0,
            focus_dist: 10.0,
            defocus_disk_horizontal: Vector3::new(0.0, 0.0, 0.0),
            defocus_disk_vertical: Vector3::new(0.0, 0.0, 0.0),

            u: Vector3::new(0.0, 0.0, 0.0),
            v: Vector3::new(0.0, 0.0, 0.0),
            w: Vector3::new(0.0, 0.0, 0.0),

            background: Color::new(0.0, 0.0, 0.0),
            brightness: 1.0,
        }
    }

    pub fn render(&mut self, world: &dyn Hittable) {
        self.initialize();

        // Render
        print!("P3\n{} {}\n255\n", self.img_width, self.img_height);

        for j in 0..self.img_height {
            eprint!("\rScanlines remaining: {} \r", self.img_height - j);
            std::io::stderr().flush().unwrap();

            for i in 0..self.img_width {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                for _ in 0..self.samples_per_pixel {
                    let r = self.get_ray(i as f64, j as f64);
                    pixel_color += self.ray_color(&r, self.max_depth, world);
                }
                pixel_color *= self.pixel_samples_scale * self.brightness;
                write_color(&pixel_color);
            }
        }

        eprintln!("\rDone.                           \n");
        std::io::stderr().flush().unwrap();
    }

    pub fn initialize(&mut self) {
        // Calculate the image height and ensure that it's at least 1
        self.img_height = ((self.img_width as f64 / self.aspect_ratio) as usize).max(1);

        self.pixel_samples_scale = 1.0 / self.samples_per_pixel as f64;

        // Camera (eye point)
        self.camera_center = self.lookfrom;

        let theta = degrees_to_radians(self.vfov);
        let h = (theta / 2.0).tan();

        let viewport_height = 2.0 * h * self.focus_dist;
        let viewport_width = viewport_height * (self.img_width as f64 / self.img_height as f64);

        self.w = (self.lookfrom - self.lookat).normalize();
        self.u = self.vup.cross(&self.w).normalize();
        self.v = self.w.cross(&self.u);

        // Calculate the vector across the horizontal and down the vertical viewport edges
        let viewport_horizontal = self.u * viewport_width;
        let viewport_vertical = -self.v * viewport_height;

        //Calculate the horizontal and vertical delta vectors from pixel to pixel
        self.pixel_delta_horizontal = viewport_horizontal / self.img_width as f64;
        self.pixel_delta_vertical = viewport_vertical / self.img_height as f64;

        // Calculate the location of the upper left pixel
        let viewport_upper_left = self.camera_center
            - (self.focus_dist * self.w)
            - viewport_horizontal / 2.0
            - viewport_vertical / 2.0;

        // Calculate the location of the pixel at (0, 0)
        let defocus_radius = self.focus_dist * (degrees_to_radians(self.defocus_angle) / 2.0).tan();
        self.defocus_disk_horizontal = self.u * defocus_radius;
        self.defocus_disk_vertical = self.v * defocus_radius;

        self.pixel00_loc =
            viewport_upper_left + 0.5 * (self.pixel_delta_horizontal + self.pixel_delta_vertical);
    }

    // Construct a camera ray originating from the defocus disk and directed at a randomly
    // sampled point around the pixel location i, j.
    fn get_ray(&self, i: f64, j: f64) -> Ray {
        let offset = Self::sample_square();
        let pixel_sample = self.pixel00_loc
            + ((i + offset.x) * self.pixel_delta_horizontal)
            + ((j + offset.y) * self.pixel_delta_vertical);

        let ray_origin = if self.defocus_angle <= 0.0 {
            self.camera_center
        } else {
            self.defocus_disk_sample()
        };

        let ray_direction = pixel_sample - ray_origin;

        Ray::new(ray_origin, ray_direction)
    }

    fn defocus_disk_sample(&self) -> Point3<f64> {
        let p = random_in_unit_disk();
        self.camera_center + p.x * self.defocus_disk_horizontal + p.y * self.defocus_disk_vertical
    }

    fn sample_square() -> Vector3<f64> {
        Vector3::new(random_double() - 0.5, random_double() - 0.5, 0.0)
    }

    fn ray_color(&mut self, r: &Ray, depth: usize, world: &dyn Hittable) -> Color {
        if depth <= 0 {
            return Color::new(0.0, 0.0, 0.0);
        }

        let mut rec = HitRecord::default();

        if !world.hit(r, Interval::new(0.001, f64::INFINITY), &mut rec) {
            return self.background;
        }

        let color_from_emission = rec.mat.emitted(rec.u, rec.v, &rec.p);

        let mut scattered = Ray::new(Point3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 0.0));
        let mut attenuation = Color::new(0.0, 0.0, 0.0);
        if rec.mat.scatter(r, &rec, &mut attenuation, &mut scattered) {
            let color_from_scatter =
                attenuation.component_mul(&self.ray_color(&scattered, depth - 1, world));
            return color_from_emission + color_from_scatter;
        }
        return color_from_emission;

        // if world.hit(r, Interval::new(0.001, f64::INFINITY), &mut rec) {
        //     let mut scattered = Ray::new(Point3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 0.0));
        //     let mut attenuation = Color::new(0.0, 0.0, 0.0);
        //     if rec.mat.scatter(r, &rec, &mut attenuation, &mut scattered) {
        //         return attenuation.component_mul(&self.ray_color(&scattered, depth - 1, world));
        //     }
        //     return Color::new(0.0, 0.0, 0.0);
        // }

        // let unit_direction = r.direction().normalize();
        // let a = 0.5 * (unit_direction.y + 1.0);
        // (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
    }
}
