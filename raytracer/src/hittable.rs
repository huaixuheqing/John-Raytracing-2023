use crate::aabb::Aabb;
use crate::material::Material;

use crate::ray::Ray;
use crate::vec3::Point3;
use crate::vec3::Vec3;
use crate::{degrees_to_radians, INFINITY};
use std::sync::Arc;

#[derive(Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub mat_ptr: Option<Arc<dyn Material>>,
    pub t: f64,
    pub u: f64,
    pub v: f64,
    pub front_face: bool,
}

impl Default for HitRecord {
    fn default() -> Self {
        Self::new()
    }
}

impl HitRecord {
    pub fn new() -> Self {
        Self {
            p: Point3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 0.0),
            t: 0.0,
            front_face: false,
            mat_ptr: None,
            u: 0.0,
            v: 0.0,
        }
    }

    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = r.direction() * *outward_normal < 0.0;
        if self.front_face {
            self.normal = *outward_normal;
        } else {
            self.normal = -*outward_normal;
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut Aabb) -> bool;
}

pub struct Translate {
    ptr: Option<Arc<dyn Hittable>>,
    offset: Vec3,
}

impl Translate {
    pub fn new(p: Option<Arc<dyn Hittable>>, displacement: &Vec3) -> Self {
        Self {
            ptr: p,
            offset: *displacement,
        }
    }
}

impl Hittable for Translate {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let moved_r = Ray::new(r.orig - self.offset, r.direction(), r.time());
        if !self.ptr.clone().unwrap().hit(&moved_r, t_min, t_max, rec) {
            return false;
        }
        rec.p += self.offset;
        rec.clone().set_face_normal(&moved_r, &rec.normal);

        true
    }

    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut Aabb) -> bool {
        if !self
            .ptr
            .clone()
            .unwrap()
            .bounding_box(time0, time1, output_box)
        {
            return false;
        }
        *output_box = Aabb::new(
            output_box.min() + self.offset,
            output_box.max() + self.offset,
        );

        true
    }
}

#[derive(Clone)]
pub struct RotateY {
    ptr: Option<Arc<dyn Hittable>>,
    sin_theta: f64,
    cos_theta: f64,
    hasbox: bool,
    bbox: Aabb,
}

impl RotateY {
    pub fn new(p: Option<Arc<dyn Hittable>>, angle: f64) -> Self {
        let radians = degrees_to_radians(angle);
        let sin_theta1 = radians.sin();
        let cos_theta1 = radians.cos();
        let mut bbox1 = Aabb::new(Point3::new(0.0, 0.0, 0.0), Point3::new(0.0, 0.0, 0.0));
        let hasbox1 = p.clone().unwrap().bounding_box(0.0, 1.0, &mut bbox1);

        let mut min = Point3::new(INFINITY, INFINITY, INFINITY);
        let mut max = Point3::new(-INFINITY, -INFINITY, -INFINITY);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = i as f64 * bbox1.max().x + (1.0 - i as f64) * bbox1.min().x;
                    let y = j as f64 * bbox1.max().y + (1.0 - j as f64) * bbox1.min().y;
                    let z = k as f64 * bbox1.max().z + (1.0 - k as f64) * bbox1.min().z;

                    let newx = cos_theta1 * x + sin_theta1 * z;
                    let newz = -sin_theta1 * x + cos_theta1 * z;

                    let tester = Vec3::new(newx, y, newz);

                    min.x = min.x.min(tester.x);
                    max.x = max.x.max(tester.x);
                    min.y = min.y.min(tester.y);
                    max.y = max.y.max(tester.y);
                    min.z = min.z.min(tester.z);
                    max.z = max.z.max(tester.z);
                }
            }
        }

        Self {
            ptr: p.clone(),
            sin_theta: sin_theta1,
            cos_theta: cos_theta1,
            hasbox: hasbox1,
            bbox: Aabb::new(min, max),
        }
    }
}

impl Hittable for RotateY {
    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut Aabb) -> bool {
        *output_box = (*self).clone().bbox;
        self.hasbox
    }

    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut origin = r.orig;
        let mut direction = r.direction();

        origin.x = self.cos_theta * r.orig[0] - self.sin_theta * r.orig[2];
        origin.z = self.sin_theta * r.orig[0] + self.cos_theta * r.orig[2];

        direction.x = self.cos_theta * r.direction()[0] - self.sin_theta * r.direction()[2];
        direction.z = self.sin_theta * r.direction()[0] + self.cos_theta * r.direction()[2];

        let rotated_r = Ray::new(origin, direction, r.time());

        if !self.ptr.clone().unwrap().hit(&rotated_r, t_min, t_max, rec) {
            return false;
        }

        let mut p = rec.p;
        let mut normal = rec.normal;

        p.x = self.cos_theta * rec.p[0] + self.sin_theta * rec.p[2];
        p.z = -self.sin_theta * rec.p[0] + self.cos_theta * rec.p[2];

        normal.x = self.cos_theta * rec.normal[0] + self.sin_theta * rec.normal[2];
        normal.z = -self.sin_theta * rec.normal[0] + self.cos_theta * rec.normal[2];

        rec.p = p;
        rec.set_face_normal(&rotated_r, &normal);

        true
    }
}
