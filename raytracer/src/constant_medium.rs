use std::sync::Arc;
use crate::{Color1, HitRecord, Hittable, INFINITY, Material, random_f64, Ray, Texture, Vec3};
use crate::aabb::Aabb;
use crate::material::Isotropic;

#[derive(Clone)]
pub struct ConstantMedium {
    boundary: Option<Arc<dyn Hittable>>,
    phase_function: Option<Arc<dyn Material>>,
    neg_inv_density:f64,
}

impl ConstantMedium {
    pub fn new(b:Option<Arc<dyn Hittable>>, d:f64, a:Option<Arc<dyn Texture>>) -> Self {
        Self{
            boundary:b,
            neg_inv_density:-1.0/d,
            phase_function:Some(Arc::new(Isotropic::new1(a))),
        }
    }
    
    pub fn new1(b:Option<Arc<dyn Hittable>>, d:f64, c:Color1) -> Self {
        Self{
            boundary:b,
            neg_inv_density:-1.0/d,
            phase_function:Some(Arc::new(Isotropic::new(c))),
        }
    }
}

impl Hittable for ConstantMedium{
    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut Aabb) -> bool {
        self.boundary.clone().unwrap().bounding_box(time0,time1,output_box)
    }

    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let enableDebug = false;
        let debugging = enableDebug && random_f64() < 0.00001;

        let mut rec1 = HitRecord::new();
        let mut rec2 = HitRecord::new();
        if !self.boundary.clone().unwrap().hit(r,-INFINITY,INFINITY, &mut rec1) {
            return false;
        }
        if !self.boundary.clone().unwrap().hit(r,rec1.t + 0.0001,INFINITY,&mut rec2) {
            return false;
        }
        if debugging {
            eprintln!("\nt_min={}, t_max={}\n",rec1.t,rec2.t);
        }

        if rec1.t < t_min {rec1.t = t_min;}
        if rec2.t > t_max {rec2.t = t_max;}

        if rec1.t > rec2.t {return  false;}
        if rec1.t < 0.0{rec1.t = 0.0;}

        let ray_length = r.direction().length();
        let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
        let hit_distance = self.neg_inv_density * random_f64().ln();

        if hit_distance > distance_inside_boundary {return false;}

        rec.t = rec1.t + hit_distance / ray_length;
        rec.p = r.at(rec.t);

        if debugging {
            eprintln!("hit_distance = {}\nrec.t = {}\n",hit_distance,rec.t);
        }

        rec.normal = Vec3::new(1.0,0.0,0.0);
        rec.front_face = true;
        rec.mat_ptr = (*self).clone().phase_function;

        true
    }
}