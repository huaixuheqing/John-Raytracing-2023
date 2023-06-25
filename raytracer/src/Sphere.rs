use crate::hittable;
use crate::vec3::Point3;
use crate::vec3::Vec3;
use crate::Hittable::hit_record;
use crate::Material::material;
use crate::Ray::ray;
use std::sync::Arc;

pub struct sphere {
    pub center: Point3,
    pub radius: f64,
    pub mat_ptr: Option<Arc<dyn material>>,
}

impl sphere {
    pub fn new(cen: Point3, r: f64, m: Option<Arc<dyn material>>) -> Self {
        Self {
            center: cen,
            radius: r,
            mat_ptr: m,
        }
    }
}

impl hittable for sphere {
    fn hit(&self, r: &ray, t_min: f64, t_max: f64, rec: &mut hit_record) -> bool {
        let oc = r.origin() - (*self).center.clone();
        let a = r.direction().length_squared();
        let half_b = oc.clone() * r.direction();
        let c = oc.clone().length_squared() - (*self).radius * (*self).radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return false;
        }
        let sqrtd = discriminant.sqrt();

        //Find the nearest root that lies in the acceptable range.
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return false;
            }
        }
        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = (rec.p - (*self).center) / (*self).radius;
        rec.set_face_normal(r, &outward_normal);
        rec.mat_ptr = (*self).mat_ptr.clone();

        return true;
    }
}
