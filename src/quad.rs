use na::{Point3, Vector3};
use std::rc::Rc;

use crate::{
    aabb::Aabb, hittable::{HitRecord, Hittable}, hittable_list::HittableList, interval::Interval, material::Material, ray::Ray
};

pub struct Quad {
    q: Point3<f64>,
    u: Vector3<f64>,
    v: Vector3<f64>,
    w: Vector3<f64>,
    mat: Rc<dyn Material>,
    bbox: Aabb,
    normal: Vector3<f64>,
    d: f64,
}

impl Quad {
    pub fn new(q: Point3<f64>, u: Vector3<f64>, v: Vector3<f64>, mat: Rc<dyn Material>) -> Quad {
        let n = u.cross(&v);
        let normal = n.normalize();
        let d = normal.dot(&q.coords);
        let w = n / n.dot(&n);

        let mut quad = Quad {
            q,
            u,
            v,
            w,
            mat,
            bbox: Aabb::new(),
            normal,
            d,
        };

        quad.set_bounding_box();
        quad
    }

    fn set_bounding_box(&mut self) {
        // Compute the bounding box of all four vertices.
        let mut bbox_diagonal1 = Aabb::new();
        bbox_diagonal1.aabb(&self.q, &(self.q + self.u + self.v));
        let mut bbox_diagonal2 = Aabb::new();
        bbox_diagonal2.aabb(&(self.q + self.u), &(self.q + self.v));
        self.bbox = Aabb::aabb_from_boxes(&bbox_diagonal1, &bbox_diagonal2);
    }

    fn is_interior(alpha: f64, beta: f64, rec: &mut HitRecord) -> bool {
        let unit_interval = Interval::new(0.0, 1.0);
        if !unit_interval.contains(alpha) || !unit_interval.contains(beta) {
            return false;
        }

        rec.u = alpha;
        rec.v = beta;
        true
    }
}

impl Hittable for Quad {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let denom = self.normal.dot(r.direction());

        // if the direction of the ray is parallel to the plane
        if denom.abs() < 1e-8 {
            return false;
        }

        // false if the point t is outside the ray interval
        let t = (self.d - self.normal.dot(&r.origin().coords)) / denom;
        if !ray_t.contains(t) {
            return false;
        }

        // Determine if the hit point lies within the planar shape using its plane coordinates.
        let intersection = r.at(t);

        let planar_hit_vector = intersection - self.q;
        let alpha = self.w.dot(&planar_hit_vector.cross(&self.v));
        let beta = self.w.dot(&self.u.cross(&planar_hit_vector));

        if !Self::is_interior(alpha, beta, rec) {
            return false;
        }

        rec.t = t;
        rec.p = intersection;
        rec.mat = self.mat.clone();
        rec.set_face_normal(r, self.normal);
        true
    }

    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }
}


pub fn box_(a: Point3<f64>, b: Point3<f64>, mat: Rc<dyn Material>) -> Rc<dyn Hittable> {
    let mut sides = HittableList::new();

    let min = Point3::new(a.x.min(b.x), a.y.min(b.y), a.z.min(b.z));
    let max = Point3::new(a.x.max(b.x), a.y.max(b.y), a.z.max(b.z));

    let dx = Vector3::new(max.x - min.x, 0.0, 0.0);
    let dy = Vector3::new(0.0, max.y - min.y, 0.0);
    let dz = Vector3::new(0.0, 0.0, max.z - min.z);

    sides.add(Rc::new(Quad::new(Point3::new(min.x, min.y, max.z), dx, dy, mat.clone()))); // front
    sides.add(Rc::new(Quad::new(Point3::new(max.x, min.y, max.z), -dz, dy, mat.clone()))); // right
    sides.add(Rc::new(Quad::new(Point3::new(max.x, min.y, min.z), -dx, dy, mat.clone()))); // back
    sides.add(Rc::new(Quad::new(Point3::new(min.x, min.y, min.z), dz, dy, mat.clone()))); // left
    sides.add(Rc::new(Quad::new(Point3::new(min.x, max.y, max.z), dx, -dz, mat.clone()))); // top
    sides.add(Rc::new(Quad::new(Point3::new(min.x, min.y, min.z), dx, dz, mat.clone()))); // bottom

    Rc::new(sides)
}