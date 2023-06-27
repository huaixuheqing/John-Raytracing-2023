use crate::aabb::Aabb;

use crate::{HitRecord, Hittable, Material, Point3, Ray, Vec3};
use std::sync::Arc;

#[derive(Clone)]
pub struct XyRect {
    mp: Option<Arc<dyn Material>>,
    x0: f64,
    x1: f64,
    y0: f64,
    y1: f64,
    k: f64,
}

impl XyRect {
    pub fn new(
        _x0: f64,
        _x1: f64,
        _y0: f64,
        _y1: f64,
        _k: f64,
        mat: Option<Arc<dyn Material>>,
    ) -> Self {
        Self {
            x0: _x0,
            x1: _x1,
            y0: _y0,
            y1: _y1,
            k: _k,
            mp: mat,
        }
    }
}

impl Hittable for XyRect {
    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut Aabb) -> bool {
        *output_box = Aabb::new(
            Point3::new(self.x0, self.y0, self.k - 0.0001),
            Point3::new(self.x1, self.y1, self.k + 0.0001),
        );
        true
    }

    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let t = (self.clone().k - r.orig.z) / r.direction().z;
        if t < t_min || t > t_max {
            return false;
        }
        let x = r.orig.x + t * r.direction().x;
        let y = r.orig.y + t * r.direction().y;
        if x < self.clone().x0 || x > self.clone().x1 || y < self.clone().y0 || y > self.clone().y1
        {
            return false;
        }
        rec.u = (x - self.clone().x0) / (self.clone().x1 - self.clone().x0);
        rec.v = (y - self.clone().y0) / (self.clone().y1 - self.clone().y0);
        rec.t = t;
        let outward_normal = Vec3::new(0.0, 0.0, 1.0);
        rec.set_face_normal(r, &outward_normal);
        rec.mat_ptr = (*self).clone().mp;
        rec.p = r.at(t);
        true
    }
}

#[derive(Clone)]
pub struct XzRect {
    mp: Option<Arc<dyn Material>>,
    x0: f64,
    x1: f64,
    z0: f64,
    z1: f64,
    k: f64,
}

impl XzRect {
    pub fn new(
        _x0: f64,
        _x1: f64,
        _z0: f64,
        _z1: f64,
        _k: f64,
        mat: Option<Arc<dyn Material>>,
    ) -> Self {
        Self {
            x0: _x0,
            x1: _x1,
            z0: _z0,
            z1: _z1,
            k: _k,
            mp: mat,
        }
    }
}

impl Hittable for XzRect {
    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut Aabb) -> bool {
        *output_box = Aabb::new(
            Point3::new(self.x0, self.k - 0.0001, self.z0),
            Point3::new(self.x1, self.k + 0.0001, self.z1),
        );
        true
    }

    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let t = (self.clone().k - r.orig.y) / r.direction().y;
        if t < t_min || t > t_max {
            return false;
        }
        let x = r.orig.x + t * r.direction().x;
        let z = r.orig.z + t * r.direction().z;
        if x < self.clone().x0 || x > self.clone().x1 || z < self.clone().z0 || z > self.clone().z1
        {
            return false;
        }
        rec.u = (x - self.clone().x0) / (self.clone().x1 - self.clone().x0);
        rec.v = (z - self.clone().z0) / (self.clone().z1 - self.clone().z0);
        rec.t = t;
        let outward_normal = Vec3::new(0.0, 1.0, 0.0);
        rec.set_face_normal(r, &outward_normal);
        rec.mat_ptr = (*self).clone().mp;
        rec.p = r.at(t);
        true
    }
}

#[derive(Clone)]
pub struct YzRect {
    mp: Option<Arc<dyn Material>>,
    y0: f64,
    y1: f64,
    z0: f64,
    z1: f64,
    k: f64,
}

impl YzRect {
    pub fn new(
        _y0: f64,
        _y1: f64,
        _z0: f64,
        _z1: f64,
        _k: f64,
        mat: Option<Arc<dyn Material>>,
    ) -> Self {
        Self {
            y0: _y0,
            y1: _y1,
            z0: _z0,
            z1: _z1,
            k: _k,
            mp: mat,
        }
    }
}

impl Hittable for YzRect {
    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut Aabb) -> bool {
        *output_box = Aabb::new(
            Point3::new(self.k - 0.0001, self.y0, self.z0),
            Point3::new(self.k + 0.0001, self.y1, self.z1),
        );
        true
    }

    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let t = (self.clone().k - r.orig.x) / r.direction().x;
        if t < t_min || t > t_max {
            return false;
        }
        let y = r.orig.y + t * r.direction().y;
        let z = r.orig.z + t * r.direction().z;
        if y < self.clone().y0 || y > self.clone().y1 || z < self.clone().z0 || z > self.clone().z1
        {
            return false;
        }
        rec.u = (y - self.clone().y0) / (self.clone().y1 - self.clone().y0);
        rec.v = (z - self.clone().z0) / (self.clone().z1 - self.clone().z0);
        rec.t = t;
        let outward_normal = Vec3::new(1.0, 0.0, 0.0);
        rec.set_face_normal(r, &outward_normal);
        rec.mat_ptr = (*self).clone().mp;
        rec.p = r.at(t);
        true
    }
}
