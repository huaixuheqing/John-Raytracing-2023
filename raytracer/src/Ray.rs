pub use crate::vec3::Point3;
pub use crate::vec3::Vec3;

#[derive(Copy, Clone)]
pub struct ray {
    pub orig: Point3,
    pub dir: Vec3,
}

impl ray {
    pub fn new(origin: Point3, direction: Vec3) -> ray {
        ray {
            orig: Point3::new(origin.x, origin.y, origin.z),
            dir: Vec3::new(direction.x, direction.y, direction.z),
        }
    }
    pub fn new1() -> ray {
        ray {
            orig: Point3::new(0.0, 0.0, 0.0),
            dir: Vec3::new(0.0, 0.0, 0.0),
        }
    }
    pub fn origin(&self) -> Point3 {
        self.orig
    }
    pub fn direction(&self) -> Vec3 {
        self.dir
    }
    pub fn at(&self, t: f64) -> Point3 {
        self.orig + self.dir * t
    }
}
