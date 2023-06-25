use crate::vec3::Point3;
use crate::vec3::Vec3;
use crate::Ray::ray;
use std::sync::Arc;
use crate::Material::material;

#[derive(Clone)]
pub struct hit_record {
    pub p: Point3,
    pub normal: Vec3,
    pub mat_ptr: Option<Arc<dyn material>>,
    pub t: f64,
    pub front_face: bool,
}

impl hit_record {
    pub fn new() -> Self {
        Self {
            p: Point3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 0.0),
            t: 0.0,
            front_face: false,
            mat_ptr: None,
        }
    }

    pub fn set_face_normal(&mut self, r: &ray, outward_normal: &Vec3) {
        self.front_face = (r.direction() * outward_normal.clone() < 0.0);
        if self.front_face {
            self.normal = outward_normal.clone();
        } else {
            self.normal = -outward_normal.clone();
        }
    }
}

pub trait hittable {
    fn hit(&self, r: &ray, t_min: f64, t_max: f64, rec: &mut hit_record) -> bool;
}
