use rand::Rng;

pub extern crate nalgebra as na;

pub use na::{Point3, Vector3};

pub mod camera;
pub mod color;
pub mod hittable;
pub mod hittable_list;
pub mod interval;
pub mod ray;
pub mod sphere;
pub mod vec3;
pub mod material;
pub mod aabb;
pub mod bvh;
pub mod quad;
pub mod cylinder;

// Returns the degrees equivalent of radians.
pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * std::f64::consts::PI / 180.0
}

// Returns a random real in [0,1).
pub fn random_double() -> f64 {
    rand::thread_rng().gen_range(0.0..1.0)
}

// Returns a random real in [min,max).
pub fn random_double_range(min: f64, max: f64) -> f64 {
    min + (max - min) * random_double()
}

