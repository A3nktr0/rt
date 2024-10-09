use std::rc::Rc;
use na::{Point3, Vector3};
use crate::{
    aabb::Aabb,
    hittable::{HitRecord, Hittable},
    interval::Interval,
    material::Material,
    ray::Ray,
};

pub struct Cylinder {
    base_center: Point3<f64>,
    height: f64,
    radius: f64,
    mat: Rc<dyn Material>,
    bbox: Aabb,
}

impl Cylinder {
    pub fn new(base_center: Point3<f64>, height: f64, radius: f64, mat: Rc<dyn Material>) -> Cylinder {
        let mut bbox = Aabb::new();
        bbox.aabb(
            &Point3::new(base_center.x - radius, base_center.y, base_center.z - radius),
            &Point3::new(base_center.x + radius, base_center.y + height, base_center.z + radius),
        );

        Cylinder {
            base_center,
            height,
            radius,
            mat,
            bbox,
        }
    }
}
impl Hittable for Cylinder {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let oc = r.origin() - self.base_center;
        let a = r.direction().x * r.direction().x + r.direction().z * r.direction().z;
        let b = 2.0 * (oc.x * r.direction().x + oc.z * r.direction().z);
        let c = oc.x * oc.x + oc.z * oc.z - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;

        let mut hit_anything = false;

        // Check for intersection with the cylindrical surface
        if discriminant >= 0.0 {
            let sqrt_discriminant = discriminant.sqrt();
            let mut root = (-b - sqrt_discriminant) / (2.0 * a);
            if !ray_t.surrounds(root) {
                root = (-b + sqrt_discriminant) / (2.0 * a);
                if ray_t.surrounds(root) {
                    let hit_point = r.at(root);
                    if hit_point.y >= self.base_center.y && hit_point.y <= self.base_center.y + self.height {
                        rec.t = root;
                        rec.p = hit_point;
                        rec.normal = Vector3::new(rec.p.x - self.base_center.x, 0.0, rec.p.z - self.base_center.z).normalize();
                        rec.set_face_normal(r, rec.normal);
                        rec.mat = self.mat.clone();
                        hit_anything = true;
                    }
                }
            } else {
                let hit_point = r.at(root);
                if hit_point.y >= self.base_center.y && hit_point.y <= self.base_center.y + self.height {
                    rec.t = root;
                    rec.p = hit_point;
                    rec.normal = Vector3::new(rec.p.x - self.base_center.x, 0.0, rec.p.z - self.base_center.z).normalize();
                    rec.set_face_normal(r, rec.normal);
                    rec.mat = self.mat.clone();
                    hit_anything = true;
                }
            }
        }

        // Check for intersection with the bottom face
        let t_bottom = (self.base_center.y - r.origin().y) / r.direction().y;
        if ray_t.surrounds(t_bottom) {
            let hit_point = r.at(t_bottom);
            let dist2 = (hit_point.x - self.base_center.x).powi(2) + (hit_point.z - self.base_center.z).powi(2);
            if dist2 <= self.radius * self.radius {
                rec.t = t_bottom;
                rec.p = hit_point;
                rec.normal = Vector3::new(0.0, -1.0, 0.0);
                rec.set_face_normal(r, rec.normal);
                rec.mat = self.mat.clone();
                hit_anything = true;
            }
        }

        // Check for intersection with the top face
        let t_top = (self.base_center.y + self.height - r.origin().y) / r.direction().y;
        if ray_t.surrounds(t_top) {
            let hit_point = r.at(t_top);
            let dist2 = (hit_point.x - self.base_center.x).powi(2) + (hit_point.z - self.base_center.z).powi(2);
            if dist2 <= self.radius * self.radius {
                rec.t = t_top;
                rec.p = hit_point;
                rec.normal = Vector3::new(0.0, 1.0, 0.0);
                rec.set_face_normal(r, rec.normal);
                rec.mat = self.mat.clone();
                hit_anything = true;
            }
        }

        hit_anything
    }

    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }
}