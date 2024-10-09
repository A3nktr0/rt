use na::Vector3;

use crate::{random_double, random_double_range};

pub fn random() -> Vector3<f64> {
    Vector3::new(random_double(), random_double(), random_double())
}

pub fn random_with_limits(min: f64, max: f64) -> Vector3<f64> {
    Vector3::new(
        random_double_range(min, max),
        random_double_range(min, max),
        random_double_range(min, max),
    )
}

pub fn random_in_unit_sphere() -> Vector3<f64> {
    loop {
        let p = random_with_limits(-1.0, 1.0);
        if p.norm_squared() < 1.0 {
            return p;
        }
    }
}

pub fn random_unit_vector() -> Vector3<f64> {
    random_in_unit_sphere().normalize()
}

pub fn random_on_hemisphere(normal: Vector3<f64>) -> Vector3<f64> {
    let on_unit_sphere = random_unit_vector();
    if on_unit_sphere.dot(&normal) > 0.0 {
        on_unit_sphere
    } else {
        -on_unit_sphere
    }
}

pub fn reflect(v: Vector3<f64>, n: Vector3<f64>) -> Vector3<f64> {
    v - 2.0 * v.dot(&n) * n
}

pub fn refract(uv: Vector3<f64>, n: Vector3<f64>, etai_over_etat: f64) -> Vector3<f64> {
    let cos_theta = (-uv).dot(&n).min(1.0);
    let r_out_perp = etai_over_etat * (uv + cos_theta * n);
    let r_out_parallel = (-((1.0 - r_out_perp.norm_squared()).abs()).sqrt()) * n;
    r_out_perp + r_out_parallel
}

pub fn random_in_unit_disk() -> Vector3<f64> {
    loop {
        let p = Vector3::new(
            random_double_range(-1.0, 1.0),
            random_double_range(-1.0, 1.0),
            0.0,
        );
        if p.norm_squared() < 1.0 {
            return p;
        }
    }
}
pub trait Vector3Ext {
    fn near_zero(&self) -> bool;
}

impl Vector3Ext for Vector3<f64> {
    fn near_zero(&self) -> bool {
        let s = 1e-8;
        self.iter().all(|&x| x.abs() < s)
    }
}

/************************************************/
// Unecessary code, just for learning purposes,
//could be replaced by the nalgebra crate
/************************************************/

// use std::fmt;
// use std::ops::{Add, Div, Mul, Sub};

// #[derive(Copy, Clone, Debug, PartialEq)]
// pub struct Vec3 {
//     e: [f32; 3],
// }

// impl Vec3 {
//     pub fn new(e0: f32, e1: f32, e2: f32) -> Self {
//         Self { e: [e0, e1, e2] }
//     }

//     pub fn x(&self) -> f32 {
//         self.e[0]
//     }

//     pub fn y(&self) -> f32 {
//         self.e[1]
//     }

//     pub fn z(&self) -> f32 {
//         self.e[2]
//     }

//     pub fn length(&self) -> f32 {
//         self.length_squared().sqrt()
//     }

//     pub fn length_squared(&self) -> f32 {
//         self.e.iter().map(|x| x * x).sum()
//     }

// pub fn random() -> Vec3 {
//     Vec3::new(random_double(), random_double(), random_double())
// }

// pub fn random_range(min: f32, max: f32) -> Vec3 {
//     Vec3::new(
//         random_double_range(min, max),
//         random_double_range(min, max),
//         random_double_range(min, max),
//     )
// }

//     pub fn update(&mut self, e0: f32, e1: f32, e2: f32) {
//         self.e = [e0, e1, e2];
//     }
// }

// impl fmt::Display for Vec3 {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{} {} {}", self.e[0], self.e[1], self.e[2])
//     }
// }

// impl Add for Vec3 {
//     type Output = Vec3;

//     fn add(self, other: Vec3) -> Vec3 {
//         Vec3::new(
//             self.e[0] + other.e[0],
//             self.e[1] + other.e[1],
//             self.e[2] + other.e[2],
//         )
//     }
// }

// impl Sub for Vec3 {
//     type Output = Vec3;

//     fn sub(self, other: Vec3) -> Vec3 {
//         Vec3::new(
//             self.e[0] - other.e[0],
//             self.e[1] - other.e[1],
//             self.e[2] - other.e[2],
//         )
//     }
// }

// impl Mul for Vec3 {
//     type Output = Vec3;

//     fn mul(self, other: Vec3) -> Vec3 {
//         Vec3::new(
//             self.e[0] * other.e[0],
//             self.e[1] * other.e[1],
//             self.e[2] * other.e[2],
//         )
//     }
// }

// impl Mul<f32> for Vec3 {
//     type Output = Vec3;

//     fn mul(self, t: f32) -> Vec3 {
//         Vec3::new(self.e[0] * t, self.e[1] * t, self.e[2] * t)
//     }
// }

// impl Div<f32> for Vec3 {
//     type Output = Vec3;

//     fn div(self, t: f32) -> Vec3 {
//         self * (1.0 / t)
//     }
// }

// pub fn dot(u: Vec3, v: Vec3) -> f32 {
//     u.e[0] * v.e[0] + u.e[1] * v.e[1] + u.e[2] * v.e[2]
// }

// pub fn cross(u: Vec3, v: Vec3) -> Vec3 {
//     Vec3::new(
//         u.e[1] * v.e[2] - u.e[2] * v.e[1],
//         u.e[2] * v.e[0] - u.e[0] * v.e[2],
//         u.e[0] * v.e[1] - u.e[1] * v.e[0],
//     )
// }

// pub fn unit_vector(v: Vec3) -> Vec3 {
//     v / v.length()
// }

// pub type Point3 = Vec3;
