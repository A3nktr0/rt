use na::{Point3, Vector3};
use nalgebra as na;

pub struct Ray {
    pub orig: Point3<f64>,
    pub dir: Vector3<f64>,
}

impl Ray {
    pub fn new(orig: Point3<f64>, dir: Vector3<f64>) -> Ray {
        Ray { orig, dir }
    }

    pub fn origin(&self) -> &Point3<f64> {
        &self.orig
    }

    pub fn direction(&self) -> &Vector3<f64> {
        &self.dir
    }

    pub fn at(&self, t: f64) -> Point3<f64> {
        self.orig + t * self.dir
    }
}
