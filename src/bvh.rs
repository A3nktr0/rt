use crate::aabb::Aabb;
use crate::hittable::{HitRecord, Hittable};
use crate::hittable_list::HittableList;
use crate::interval::Interval;
use crate::ray::Ray;
use std::rc::Rc;

// Bounding Volume Hierarchy
pub struct BVHNode {
    left: Rc<dyn Hittable>,
    right: Rc<dyn Hittable>,
    bbox: Aabb,
}

impl BVHNode {
    pub fn new_from_list(list: &HittableList) -> BVHNode {
        BVHNode::new(list.objects.clone(), 0, list.objects.len())
    }

    fn new(mut objects: Vec<Rc<dyn Hittable>>, start: usize, end: usize) -> BVHNode {
        let mut bbox = Aabb::empty();

        for i in start..end {
            bbox = Aabb::aabb_from_boxes(&bbox, objects[i].bounding_box());
        }

        let axis = bbox.longest_axis();

        let comparator = |a: &Rc<dyn Hittable>, b: &Rc<dyn Hittable>| {
            let box_a = a.bounding_box();
            let box_b = b.bounding_box();
            box_a
                .axis_interval(axis)
                .min
                .partial_cmp(&box_b.axis_interval(axis).min)
                .unwrap()
        };

        let left;
        let right;

        let object_span = end - start;
        match object_span {
            1 => {
                left = objects[start].clone();
                right = objects[start].clone();
            }
            2 => {
                left = objects[start].clone();
                right = objects[start + 1].clone();
            }
            _ => {
                objects[start..end].sort_by(comparator);
                let mid = start + object_span / 2;

                left = Rc::new(BVHNode::new(objects.clone(), start, mid + 1));
                right = Rc::new(BVHNode::new(objects.clone(), mid, end));
            }
        };

        BVHNode { left, right, bbox }
    }
}

impl Hittable for BVHNode {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        if !self.bbox.hit(&r, ray_t) {
            return false;
        }

        let hit_left = self.left.hit(&r, ray_t, rec);
        let hit_right = self.right.hit(
            &r,
            Interval::new(ray_t.min, if hit_left { rec.t } else { ray_t.max }),
            rec,
        );

        hit_left || hit_right
    }

    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }
}
