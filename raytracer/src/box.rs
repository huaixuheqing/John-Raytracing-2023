use std::sync::Arc;
use crate::{HitRecord, Hittable, HittableList, Material, Point3, Ray};
use crate::aabb::Aabb;
use crate::aarect::{XyRect, XzRect, YzRect};

pub struct Box1 {
    box_min:Point3,
    box_max:Point3,
    sides:HittableList,
}

impl Box1 {
    pub fn new(p0:&Point3, p1:&Point3, ptr:Option<Arc<dyn Material>>) -> Self {
        let mut sides1 = HittableList::new();
        sides1.add(Some(Arc::new(XyRect::new(p0.x,p1.x,p0.y,p1.y,p1.z, ptr.clone()))));
        sides1.add(Some(Arc::new(XyRect::new(p0.x,p1.x,p0.y,p1.y,p0.z, ptr.clone()))));

        sides1.add(Some(Arc::new(XzRect::new(p0.x,p1.x,p0.z,p1.z,p1.y, ptr.clone()))));
        sides1.add(Some(Arc::new(XzRect::new(p0.x,p1.x,p0.z,p1.z,p0.y, ptr.clone()))));

        sides1.add(Some(Arc::new(YzRect::new(p0.y,p1.y,p0.z,p1.z,p1.x, ptr.clone()))));
        sides1.add(Some(Arc::new(YzRect::new(p0.y,p1.y,p0.z,p1.z,p0.x, ptr.clone()))));

        Self{
            box_max:p1.clone(),
            box_min:p0.clone(),
            sides:sides1,
        }
    }
}

impl Hittable for Box1 {
    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut Aabb) -> bool {
        *output_box = Aabb::new(self.box_min,self.box_max);
        true
    }

    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        self.sides.hit(r,t_min,t_max,rec)
    }
}