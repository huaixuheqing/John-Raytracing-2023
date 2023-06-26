use crate::{aabb, hittable, hittable_list, rtweekend, vec3, HitRecord, Point3, Ray};
use aabb::Aabb;
pub use hittable::Hittable;
pub use hittable_list::HittableList;
pub use rtweekend::random_i32;
use std::sync::Arc;

pub use vec3::Vec3;

#[derive(Clone)]
pub struct BvhNode {
    left: Option<Arc<dyn Hittable>>,
    right: Option<Arc<dyn Hittable>>,
    box1: Aabb,
}

pub fn sort(a: &mut Vec<Option<Arc<dyn Hittable>>>, start: i32, end: i32, axis: i32) {
    if start >= end - 1 {
        return;
    };
    let tmp = (*a)[start as usize].clone();
    let mut start1 = start;
    let mut end1 = end - 1;
    while start1 < end1 {
        while start1 < end1
            && BvhNode::box_compare(&tmp.clone(), &(*a)[end1 as usize].clone(), axis)
        {
            end1 -= 1;
        }
        if start1 < end1 {
            (*a)[start1 as usize] = (*a)[end1 as usize].clone();
        }
        while start1 < end1
            && BvhNode::box_compare(&(*a)[start1 as usize].clone(), &tmp.clone(), axis)
        {
            start1 += 1;
        }
        if start1 < end1 {
            (*a)[end1 as usize] = (*a)[start1 as usize].clone();
        }
    }
    (*a)[start1 as usize] = tmp;
    sort(a, start, start1, axis);
    sort(a, start1 + 1, end1, axis);
}

impl BvhNode {
    pub fn box_compare(
        a: &Option<Arc<dyn Hittable>>,
        b: &Option<Arc<dyn Hittable>>,
        axis: i32,
    ) -> bool {
        let mut box_a: Aabb = Aabb::new(Point3::new(0.0, 0.0, 0.0), Point3::new(0.0, 0.0, 0.0));
        let mut box_b: Aabb = Aabb::new(Point3::new(0.0, 0.0, 0.0), Point3::new(0.0, 0.0, 0.0));

        if !a.clone().unwrap().bounding_box(0.0, 0.0, &mut box_a)
            || !b.clone().unwrap().bounding_box(0.0, 0.0, &mut box_b)
        {
            eprintln!("No bounding box in bvh_node constructor.\n");
        }

        box_a.min()[axis as usize] < box_b.min()[axis as usize]
    }

    pub fn box_x_compare(a: &Option<Arc<dyn Hittable>>, b: &Option<Arc<dyn Hittable>>) -> bool {
        BvhNode::box_compare(a, b, 0)
    }

    pub fn box_y_compare(a: &Option<Arc<dyn Hittable>>, b: &Option<Arc<dyn Hittable>>) -> bool {
        BvhNode::box_compare(a, b, 1)
    }

    pub fn box_z_compare(a: &Option<Arc<dyn Hittable>>, b: &Option<Arc<dyn Hittable>>) -> bool {
        BvhNode::box_compare(a, b, 2)
    }

    pub fn new(
        src_objects: &mut Vec<Option<Arc<dyn Hittable>>>,
        start: usize,
        end: usize,
        time0: f64,
        time1: f64,
    ) -> Self {
        let objects = src_objects;

        let axis = random_i32(0, 2);
        let comparator = if axis == 0 {
            BvhNode::box_x_compare
        } else if axis == 1 {
            BvhNode::box_y_compare
        } else {
            BvhNode::box_z_compare
        };

        let object_span = end - start;
        let left1: Option<Arc<dyn Hittable>>;
        let right1: Option<Arc<dyn Hittable>>;
        if object_span == 1 {
            left1 = objects[start].clone();
            right1 = objects[start].clone();
        } else if object_span == 2 {
            if comparator(&objects[start], &objects[start + 1]) {
                left1 = objects[start].clone();
                right1 = objects[start + 1].clone();
            } else {
                left1 = objects[start + 1].clone();
                right1 = objects[start].clone();
            }
        } else {
            sort(objects, start as i32, end as i32, axis);

            let mid = start + object_span / 2;
            left1 = Some(Arc::new(BvhNode::new(objects, start, mid, time0, time1)));
            right1 = Some(Arc::new(BvhNode::new(objects, mid, end, time0, time1)));
        }

        let mut box_left: Aabb = Aabb::new(Point3::new(0.0, 0.0, 0.0), Point3::new(0.0, 0.0, 0.0));
        let mut box_right: Aabb = Aabb::new(Point3::new(0.0, 0.0, 0.0), Point3::new(0.0, 0.0, 0.0));

        if !left1
            .clone()
            .unwrap()
            .bounding_box(time0, time1, &mut box_left)
            || !right1
                .clone()
                .unwrap()
                .bounding_box(time0, time1, &mut box_right)
        {
            eprintln!("No bounding box in bvh_node constructor.\n");
        }

        let box2 = Aabb::surrounding_box(&box_left, &box_right);
        Self {
            left: left1,
            right: right1,
            box1: box2,
        }
    }

    pub fn new1(list: &mut HittableList, time0: f64, time1: f64) -> Self {
        let length = list.objects.len();
        BvhNode::new(&mut list.objects, 0, length, time0, time1)
    }
}

impl Hittable for BvhNode {
    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut Aabb) -> bool {
        *output_box = (*self).clone().box1;
        true
    }

    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        if (*self).clone().box1.hit(r, t_min, t_max) {
            return false;
        }
        let hit_left = self.left.clone().unwrap().hit(r, t_min, t_max, &mut *rec);
        let hit_right = if hit_left {
            self.right.clone().unwrap().hit(r, t_min, rec.t, &mut *rec)
        } else {
            self.right.clone().unwrap().hit(r, t_min, t_max, &mut *rec)
        };

        hit_left || hit_right
    }
}
