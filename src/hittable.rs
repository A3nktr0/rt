use std::rc::Rc;

use crate::aabb::Aabb;
use crate::degrees_to_radians;
use crate::interval::Interval;
use crate::material::{DefaultMaterial, Material};
use crate::na::{Point3, Vector3};
use crate::ray::Ray;

#[derive(Clone)]
pub struct HitRecord {
    pub p: Point3<f64>,
    pub normal: Vector3<f64>,
    pub t: f64,
    pub front_face: bool,
    pub mat: Rc<dyn Material>,
    pub u: f64,
    pub v: f64,
}

impl HitRecord {
    pub fn default() -> HitRecord {
        HitRecord {
            p: Point3::new(0.0, 0.0, 0.0),
            normal: Vector3::new(0.0, 0.0, 0.0),
            t: 0.0,
            front_face: false,
            mat: Rc::new(DefaultMaterial::new()),
            u: 0.0,
            v: 0.0,
        }
    }

    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vector3<f64>) {
        self.front_face = r.direction().dot(&outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool;
    fn bounding_box(&self) -> &Aabb;
}

pub struct Translate {
    object: Rc<dyn Hittable>,
    offset: Vector3<f64>,
    bbox: Aabb,
}

impl Translate {
    pub fn new(object: Rc<dyn Hittable>, offset: Vector3<f64>) -> Translate {
        // let bbox = object.bounding_box() + offset;
        let bbox = object.bounding_box() + &offset;
        Translate {
            object,
            offset,
            bbox,
        }
    }
}

impl Hittable for Translate {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let offset_r = Ray::new(r.origin() - self.offset, *r.direction());
        if !self.object.hit(&offset_r, ray_t, rec) {
            return false;
        }
        rec.p += self.offset;
        true
    }

    fn bounding_box(&self) -> &Aabb {
        // self.object.bounding_box()
        &self.bbox
    }
}

pub struct RotateY {
    object: Rc<dyn Hittable>,
    sin_theta: f64,
    cos_theta: f64,
    bbox: Aabb,
}

impl RotateY {
    pub fn new(object: Rc<dyn Hittable>, angle: f64) -> RotateY {
        let radians = degrees_to_radians(angle);
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        let bbox = object.bounding_box();

        let mut min = Point3::new(f64::INFINITY, f64::INFINITY, f64::INFINITY);
        let mut max = Point3::new(-f64::INFINITY, -f64::INFINITY, -f64::INFINITY);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = i as f64 * bbox.x.get().max + (1 - i) as f64 * bbox.x.get().min;
                    let y = j as f64 * bbox.y.get().max + (1 - j) as f64 * bbox.y.get().min;
                    let z = k as f64 * bbox.z.get().max + (1 - k) as f64 * bbox.z.get().min;

                    let newx = cos_theta * x + sin_theta * z;
                    let newz = -sin_theta * x + cos_theta * z;

                    let tester = Vector3::new(newx, y, newz);

                    for c in 0..3 {
                        min[c] = min[c].min(tester[c]);
                        max[c] = max[c].max(tester[c]);
                    }
                }
            }
        }
        let mut bbox = Aabb::new();
        bbox.aabb(&min, &max);

        RotateY {
            object,
            sin_theta,
            cos_theta,
            bbox,
        }
    }
}

impl Hittable for RotateY {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let origin = Point3::new(
            self.cos_theta * r.origin().x - self.sin_theta * r.origin().z,
            r.origin().y,
            self.sin_theta * r.origin().x + self.cos_theta * r.origin().z,
        );

        let direction = Vector3::new(
            self.cos_theta * r.direction().x - self.sin_theta * r.direction().z,
            r.direction().y,
            self.sin_theta * r.direction().x + self.cos_theta * r.direction().z,
        );

        let rotated_r = Ray::new(origin, direction);

        if !self.object.hit(&rotated_r, ray_t, rec) {
            return false;
        }

        rec.p = Point3::new(
            self.cos_theta * rec.p.x + self.sin_theta * rec.p.z,
            rec.p.y,
            -self.sin_theta * rec.p.x + self.cos_theta * rec.p.z,
        );

        rec.normal = Vector3::new(
            self.cos_theta * rec.normal.x + self.sin_theta * rec.normal.z,
            rec.normal.y,
            -self.sin_theta * rec.normal.x + self.cos_theta * rec.normal.z,
        );

        true
    }

    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }
}

pub struct RotateX {
    object: Rc<dyn Hittable>,
    sin_theta: f64,
    cos_theta: f64,
    bbox: Aabb,
}

impl RotateX {
    pub fn new(object: Rc<dyn Hittable>, angle: f64) -> RotateX {
        let radians = degrees_to_radians(angle);
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        let bbox = object.bounding_box();

        let mut min = Point3::new(f64::INFINITY, f64::INFINITY, f64::INFINITY);
        let mut max = Point3::new(-f64::INFINITY, -f64::INFINITY, -f64::INFINITY);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = i as f64 * bbox.x.get().max + (1 - i) as f64 * bbox.x.get().min;
                    let y = j as f64 * bbox.y.get().max + (1 - j) as f64 * bbox.y.get().min;
                    let z = k as f64 * bbox.z.get().max + (1 - k) as f64 * bbox.z.get().min;

                    let newy = cos_theta * y + sin_theta * z;
                    let newz = -sin_theta * y + cos_theta * z;

                    let tester = Vector3::new(x, newy, newz);

                    for c in 0..3 {
                        min[c] = min[c].min(tester[c]);
                        max[c] = max[c].max(tester[c]);
                    }
                }
            }
        }
        let mut bbox = Aabb::new();
        bbox.aabb(&min, &max);

        RotateX {
            object,
            sin_theta,
            cos_theta,
            bbox,
        }
    }
}

impl Hittable for RotateX {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let origin = Point3::new(
            r.origin().x,
            self.cos_theta * r.origin().y - self.sin_theta * r.origin().z,
            self.sin_theta * r.origin().y + self.cos_theta * r.origin().z,
        );

        let direction = Vector3::new(
            r.direction().x,
            self.cos_theta * r.direction().y - self.sin_theta * r.direction().z,
            self.sin_theta * r.direction().y + self.cos_theta * r.direction().z,
        );

        let rotated_r = Ray::new(origin, direction);

        if !self.object.hit(&rotated_r, ray_t, rec) {
            return false;
        }

        rec.p = Point3::new(
            rec.p.x,
            self.cos_theta * rec.p.y + self.sin_theta * rec.p.z,
            -self.sin_theta * rec.p.y + self.cos_theta * rec.p.z,
        );

        rec.normal = Vector3::new(
            rec.normal.x,
            self.cos_theta * rec.normal.y + self.sin_theta * rec.normal.z,
            -self.sin_theta * rec.normal.y + self.cos_theta * rec.normal.z,
        );

        true
    }

    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }
}

pub struct RotateZ {
    object: Rc<dyn Hittable>,
    sin_theta: f64,
    cos_theta: f64,
    bbox: Aabb,
}

impl RotateZ {
    pub fn new(object: Rc<dyn Hittable>, angle: f64) -> RotateZ {
        let radians = degrees_to_radians(angle);
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        let bbox = object.bounding_box();

        let mut min = Point3::new(f64::INFINITY, f64::INFINITY, f64::INFINITY);
        let mut max = Point3::new(-f64::INFINITY, -f64::INFINITY, -f64::INFINITY);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = i as f64 * bbox.x.get().max + (1 - i) as f64 * bbox.x.get().min;
                    let y = j as f64 * bbox.y.get().max + (1 - j) as f64 * bbox.y.get().min;
                    let z = k as f64 * bbox.z.get().max + (1 - k) as f64 * bbox.z.get().min;

                    let newx = cos_theta * x + sin_theta * y;
                    let newy = -sin_theta * x + cos_theta * y;

                    let tester = Vector3::new(newx, newy, z);

                    for c in 0..3 {
                        min[c] = min[c].min(tester[c]);
                        max[c] = max[c].max(tester[c]);
                    }
                }
            }
        }
        let mut bbox = Aabb::new();
        bbox.aabb(&min, &max);

        RotateZ {
            object,
            sin_theta,
            cos_theta,
            bbox,
        }
    }
}

impl Hittable for RotateZ {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let origin = Point3::new(
            self.cos_theta * r.origin().x - self.sin_theta * r.origin().y,
            self.sin_theta * r.origin().x + self.cos_theta * r.origin().y,
            r.origin().z,
        );

        let direction = Vector3::new(
            self.cos_theta * r.direction().x - self.sin_theta * r.direction().y,
            self.sin_theta * r.direction().x + self.cos_theta * r.direction().y,
            r.direction().z,
        );

        let rotated_r = Ray::new(origin, direction);

        if !self.object.hit(&rotated_r, ray_t, rec) {
            return false;
        }

        rec.p = Point3::new(
            self.cos_theta * rec.p.x + self.sin_theta * rec.p.y,
            -self.sin_theta * rec.p.x + self.cos_theta * rec.p.y,
            rec.p.z,
        );

        rec.normal = Vector3::new(
            self.cos_theta * rec.normal.x + self.sin_theta * rec.normal.y,
            -self.sin_theta * rec.normal.x + self.cos_theta * rec.normal.y,
            rec.normal.z,
        );

        true
    }

    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }
}
