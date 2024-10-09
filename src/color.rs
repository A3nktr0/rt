use na::Vector3;
use nalgebra as na;

use crate::interval::Interval;

pub fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0.0 {
        linear_component.sqrt()
    } else {
        0.0
    }
}

pub fn write_color(color: &Vector3<f64>) {
    let mut r = color[0];
    let mut g = color[1];
    let mut b = color[2];

    // Apply a linear to gamma conversion for gamma 2.
    r = linear_to_gamma(r);
    g = linear_to_gamma(g);
    b = linear_to_gamma(b);

    // Translate the [0,1] component values to the byte range [0,255].
    let intensity = Interval::new(0.000, 0.999);
    let rbyte = (256.0 * intensity.clamp(r)) as i32;
    let gbyte = (256.0 * intensity.clamp(g)) as i32;
    let bbyte = (256.0 * intensity.clamp(b)) as i32;

    println!("{} {} {}", rbyte, gbyte, bbyte);
}


pub type Color = Vector3<f64>;
