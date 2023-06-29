use crate::aabb::Aabb;
use crate::hittable::{HitRecord, Hittable};
use crate::{material, Ray};
use crate::{vec3, Point3};
use material::Material;
use std::sync::Arc;
pub use vec3::Vec3;

pub struct MovingSphere {
    center0: Point3,
    center1: Point3,
    time0: f64,
    time1: f64,
    radius: f64,
    mat_ptr: Option<Arc<dyn Material + Send + Sync>>,
}

impl MovingSphere {
    pub fn new(
        cen0: Point3,
        cen1: Point3,
        _time0: f64,
        _time1: f64,
        r: f64,
        m: Option<Arc<dyn Material + Send + Sync>>,
    ) -> Self {
        Self {
            center0: cen0,
            center1: cen1,
            time0: _time0,
            time1: _time1,
            radius: r,
            mat_ptr: m,
        }
    }

    pub fn center(&self, time: f64) -> Point3 {
        self.center0
            + (self.center1 - self.center0) * ((time - self.time0) / self.time1 - self.time0)
    }
}

impl Hittable for MovingSphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc = r.origin() - (*self).center(r.time());
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
        let outward_normal = (rec.p - self.center(r.time())) / self.radius;
        rec.set_face_normal(r, &outward_normal);
        rec.mat_ptr = self.mat_ptr.clone();

        true
    }

    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut Aabb) -> bool {
        let box0 = Aabb::new(
            (*self).center(_time0) - Vec3::new(self.radius, self.radius, self.radius),
            (*self).center(_time0) + Vec3::new(self.radius, self.radius, self.radius),
        );
        let box1 = Aabb::new(
            (*self).center(_time1) - Vec3::new(self.radius, self.radius, self.radius),
            (*self).center(_time1) + Vec3::new(self.radius, self.radius, self.radius),
        );
        *output_box = Aabb::surrounding_box(&box0, &box1);
        true
    }
}
