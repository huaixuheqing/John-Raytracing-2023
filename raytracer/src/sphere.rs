use crate::vec3::Point3;
use crate::{Hittable, Vec3};

use crate::aabb::Aabb;
use crate::hittable::HitRecord;
use crate::material::Material;
use crate::ray::Ray;
use std::f64::consts::PI;
use std::sync::Arc;

pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
    pub mat_ptr: Option<Arc<dyn Material>>,
}

impl Sphere {
    pub fn new(cen: Point3, r: f64, m: Option<Arc<dyn Material>>) -> Self {
        Self {
            center: cen,
            radius: r,
            mat_ptr: m,
        }
    }
}

pub fn get_sphere_uv(p: &Point3, u: &mut f64, v: &mut f64) {
    let theta = (-p.y).acos();
    let phi = (-p.z).atan2(p.x) + PI;
    *u = phi / (2.0 * PI);
    *v = theta / PI;
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc = r.origin() - self.center;
        let a = r.direction().length_squared();
        let half_b = oc * r.direction();
        let c = oc.clone().length_squared() - self.radius * self.radius;
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
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, &outward_normal);
        get_sphere_uv(&outward_normal, &mut rec.u, &mut rec.v);
        rec.mat_ptr = self.mat_ptr.clone();

        true
    }

    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut Aabb) -> bool {
        *output_box = Aabb::new(
            self.center - Vec3::new(self.radius, self.radius, self.radius),
            self.center + Vec3::new(self.radius, self.radius, self.radius),
        );
        true
    }
}
