use std::{cell::Cell, ops::Add};

use na::{Point3, Vector3};

use crate::{interval::Interval, ray::Ray};

#[derive(Clone)]
pub struct Aabb {
    pub x: Cell<Interval>,
    pub y: Cell<Interval>,
    pub z: Cell<Interval>,
}

impl Aabb {
    pub fn new() -> Aabb {
        Aabb {
            x: Cell::new(Interval::EMPTY),
            y: Cell::new(Interval::EMPTY),
            z: Cell::new(Interval::EMPTY),
        }
    }

    pub fn aabb(&mut self, a: &Point3<f64>, b: &Point3<f64>) {
        let x = Interval::new(a.x.min(b.x), a.x.max(b.x));
        let y = Interval::new(a.y.min(b.y), a.y.max(b.y));
        let z = Interval::new(a.z.min(b.z), a.z.max(b.z));

        self.pad_to_minimums();

        self.x.set(x);
        self.y.set(y);
        self.z.set(z);
    }

    pub fn aabb_from_boxes(box0: &Aabb, box1: &Aabb) -> Aabb {
        let x = Interval::from_intervals(&box0.x.get(), &box1.x.get());
        let y = Interval::from_intervals(&box0.y.get(), &box1.y.get());
        let z = Interval::from_intervals(&box0.z.get(), &box1.z.get());

        Aabb {
            x: Cell::new(x),
            y: Cell::new(y),
            z: Cell::new(z),
        }
    }

    pub fn axis_interval(&self, n: usize) -> Interval {
        match n {
            1 => self.y.get(),
            2 => self.z.get(),
            _ => self.x.get(),
        }
    }

    pub fn empty() -> Aabb {
        Aabb {
            x: Cell::new(Interval::EMPTY),
            y: Cell::new(Interval::EMPTY),
            z: Cell::new(Interval::EMPTY),
        }
    }

    pub fn hit(&self, r: &Ray, mut ray_t: Interval) -> bool {
        let ray_orig = r.origin();
        let ray_dir = r.direction();

        for axis in 0..3 {
            let ax = self.axis_interval(axis);
            let adinv = 1.0 / ray_dir[axis];

            let t0 = (ax.min - ray_orig[axis]) * adinv;
            let t1 = (ax.max - ray_orig[axis]) * adinv;

            let (t_min, t_max) = if t0 < t1 { (t0, t1) } else { (t1, t0) };

            ray_t.min = ray_t.min.max(t_min);
            ray_t.max = ray_t.max.min(t_max);

            if ray_t.max <= ray_t.min {
                return false;
            }
        }
        true
    }

    pub fn longest_axis(&self) -> usize {
        if self.x.get().size() > self.y.get().size() {
            if self.x.get().size() > self.z.get().size() {
                0
            } else {
                2
            }
        } else {
            if self.y.get().size() > self.z.get().size() {
                1
            } else {
                2
            }
        }
    }

    // Adjust the AABB so that no side is narrower than some delta, padding if necessary.
    fn pad_to_minimums(&self) {
        let delta = 0.0001;

        if self.x.get().size() < delta {
            self.x.set(self.x.get().expand(delta));
        }
        if self.y.get().size() < delta {
            self.y.set(self.y.get().expand(delta));
        }
        if self.z.get().size() < delta {
            self.z.set(self.z.get().expand(delta));
        }
    }
}

impl Add<&Vector3<f64>> for &Aabb {
    type Output = Aabb;

    fn add(self, offset: &Vector3<f64>) -> Aabb {
        Aabb {
            x: Cell::new(self.x.get() + offset.x),
            y: Cell::new(self.y.get() + offset.y),
            z: Cell::new(self.z.get() + offset.z),
        }
    }
}

impl Add<Aabb> for &Vector3<f64> {
    type Output = Aabb;

    fn add(self, bbox: Aabb) -> Aabb {
        &bbox + self
    }
}
