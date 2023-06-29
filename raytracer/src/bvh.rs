use crate::{aabb, hittable, hittable_list, rtweekend, vec3, HitRecord, Point3, Ray};
use aabb::Aabb;
pub use hittable::Hittable;
pub use hittable_list::HittableList;
pub use rtweekend::random_i32;
use std::sync::Arc;

pub use vec3::Vec3;

#[derive(Clone)]
pub struct BvhNode {
    left: Option<Arc<dyn Hittable + Send + Sync>>,
    right: Option<Arc<dyn Hittable + Send + Sync>>,
    box1: Aabb,
}

impl BvhNode {
    pub fn box_compare(
        a: &Option<Arc<dyn Hittable + Send + Sync>>,
        b: &Option<Arc<dyn Hittable + Send + Sync>>,
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

    pub fn box_x_compare(
        a: &Option<Arc<dyn Hittable + Send + Sync>>,
        b: &Option<Arc<dyn Hittable + Send + Sync>>,
    ) -> bool {
        BvhNode::box_compare(a, b, 0)
    }

    pub fn box_y_compare(
        a: &Option<Arc<dyn Hittable + Send + Sync>>,
        b: &Option<Arc<dyn Hittable + Send + Sync>>,
    ) -> bool {
        BvhNode::box_compare(a, b, 1)
    }

    pub fn box_z_compare(
        a: &Option<Arc<dyn Hittable + Send + Sync>>,
        b: &Option<Arc<dyn Hittable + Send + Sync>>,
    ) -> bool {
        BvhNode::box_compare(a, b, 2)
    }

    pub fn new0() -> Self {
        Self {
            left: None,
            right: None,
            box1: Aabb::new(Point3::new(0.0, 0.0, 0.0), Point3::new(0.0, 0.0, 0.0)),
        }
    }

    pub fn new(
        src_objects: &mut [Option<Arc<dyn Hittable + Send + Sync>>],
        start: usize,
        end: usize,
        time0: f64,
        time1: f64,
    ) -> Self {
        let objects = &mut src_objects.to_owned();

        let axis = random_i32(0, 2);
        let comparator = if axis == 0 {
            BvhNode::box_x_compare
        } else if axis == 1 {
            BvhNode::box_y_compare
        } else {
            BvhNode::box_z_compare
        };

        let object_span = end - start;
        let mut tmp = BvhNode::new0();
        if object_span == 1 {
            tmp.left = objects[start].clone();
            tmp.right = objects[start].clone();
        } else if object_span == 2 {
            if comparator(&objects[start], &objects[start + 1]) {
                tmp.left = objects[start].clone();
                tmp.right = objects[start + 1].clone();
            } else {
                tmp.left = objects[start + 1].clone();
                tmp.right = objects[start].clone();
            }
        } else {
            objects[start..end].sort_by(|a, b| {
                if comparator(a, b) {
                    std::cmp::Ordering::Less
                } else {
                    std::cmp::Ordering::Greater
                }
            });

            let mid = start + object_span / 2;
            tmp.left = Some(Arc::new(BvhNode::new(objects, start, mid, time0, time1)));
            tmp.right = Some(Arc::new(BvhNode::new(objects, mid, end, time0, time1)));
        }

        let mut box_left: Aabb = Aabb::new(Point3::new(0.0, 0.0, 0.0), Point3::new(0.0, 0.0, 0.0));
        let mut box_right: Aabb = Aabb::new(Point3::new(0.0, 0.0, 0.0), Point3::new(0.0, 0.0, 0.0));

        if !tmp
            .left
            .clone()
            .unwrap()
            .bounding_box(time0, time1, &mut box_left)
            || !tmp
                .right
                .clone()
                .unwrap()
                .bounding_box(time0, time1, &mut box_right)
        {
            eprintln!("No bounding box in bvh_node constructor.\n");
        }

        let box2 = Aabb::surrounding_box(&box_left, &box_right);
        Self {
            left: tmp.left,
            right: tmp.right,
            box1: box2,
        }
    }

    pub fn new1(list: &mut HittableList, time0: f64, time1: f64) -> Self {
        let length = list.objects.len();
        BvhNode::new(&mut list.objects.clone(), 0, length, time0, time1)
    }
}

impl Hittable for BvhNode {
    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut Aabb) -> bool {
        *output_box = self.box1.clone();
        true
    }

    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        if !(*self).clone().box1.hit(r, t_min, t_max) {
            return false;
        }
        let hit_left = self.left.clone().unwrap().hit(r, t_min, t_max, &mut *rec);
        let hit_right =
            self.right
                .clone()
                .unwrap()
                .hit(r, t_min, if hit_left { rec.t } else { t_max }, rec);

        hit_left || hit_right
    }
}
