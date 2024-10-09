use std::rc::Rc;

use na::Vector3;
use nalgebra::Point3;

use crate::{
    aabb::Aabb,
    hittable::{HitRecord, Hittable},
    interval::Interval,
    material::Material,
    ray::Ray,
};

pub struct Sphere {
    center: Point3<f64>,
    radius: f64,
    mat: Rc<dyn Material>,
    bbox: Aabb,
}

impl Sphere {
    pub fn new(center: Point3<f64>, radius: f64, mat: Rc<dyn Material>) -> Sphere {
        let rvec = Vector3::new(radius, radius, radius);
        let mut bbox = Aabb::new();
        bbox.aabb(&(&center - &rvec), &(&center + &rvec));

        Sphere {
            center,
            radius: radius.max(0.0),
            mat,
            bbox,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let oc = self.center - r.origin();
        let a = r.dir.norm_squared();
        let h = r.dir.dot(&oc);
        let c = oc.norm_squared() - self.radius * self.radius;
        let discriminant = h * h - a * c;

        if discriminant < 0.0 {
            return false;
        }

        let sqrt_discriminant = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (h - sqrt_discriminant) / a;
        if !ray_t.surrounds(root) {
            root = (h + sqrt_discriminant) / a;
            if !ray_t.surrounds(root) {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(root);
        rec.normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, rec.normal);
        rec.mat = self.mat.clone();

        true
    }

    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }
}
