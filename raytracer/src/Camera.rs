use crate::{rtweekend, vec3, Ray};

pub use rtweekend::degrees_to_radians;
pub use vec3::Point3;
pub use vec3::Vec3;
pub use Ray::ray;

pub struct camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f64,
}

impl camera {
    pub fn new(
        lookfrom: &Point3,
        lookat: &Point3,
        vup: &Vec3,
        vfov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Self {
        let theta = degrees_to_radians(vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w1 = (*lookfrom - *lookat).unit_vector();
        let u1 = Vec3::cross(vup, &w1).unit_vector();
        let v1 = Vec3::cross(&w1, &u1);

        let origin1 = *lookfrom;
        let horizontal1 = u1 * viewport_width * focus_dist;
        let vertical1 = v1 * viewport_height * focus_dist;
        Self {
            origin: origin1,
            horizontal: horizontal1,
            vertical: vertical1,
            lower_left_corner: origin1
                - horizontal1 / 2.0
                - vertical1 / 2.0
                - w1 * focus_dist,
            w: w1,
            u: u1,
            v: v1,
            lens_radius: aperture / 2.0,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> ray {
        let rd = Vec3::random_in_unit_disk() * self.lens_radius;
        let offest = self.u * rd.x + self.v * rd.y;

        ray::new(
            self.origin + offest,
            self.lower_left_corner + self.horizontal * s + self.vertical * t
                - self.origin
                - offest,
        )
    }
}
