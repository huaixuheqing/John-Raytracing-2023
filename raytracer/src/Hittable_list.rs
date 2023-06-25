use crate::Hittable::{hit_record, hittable};
pub use crate::Ray::ray;
pub use crate::Sphere::sphere;
pub use std::sync::Arc;
pub use std::vec;

#[derive(Clone)]
pub struct hittable_list {
    objects: Vec<Option<Arc<dyn hittable>>>,
}

impl hittable_list {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Option<Arc<dyn hittable>>) {
        self.objects.push(object);
    }

    pub fn hit(&self, r: &ray, t_min: f64, t_max: f64, rec: &mut hit_record) -> bool {
        let mut temp_rec = hit_record::new();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for object in (*self).clone().objects {
            if object
                .clone()
                .unwrap()
                .hit(r, t_min, closest_so_far, &mut temp_rec)
            {
                hit_anything = true;
                closest_so_far = temp_rec.clone().t;
                *rec = temp_rec.clone();
            }
        }
        hit_anything
    }
}
