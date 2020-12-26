use crate::Vec3 as Point3;
use crate::Vec3;
use crate::ray::Ray;
use crate::rtweekend;

pub struct Camera {
    origin: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Vec3,
}


impl Camera {
    pub fn new(look_from: Point3, look_at: Point3, vup: Vec3,  vfoV: f32, aspect_ratio: f32) -> Camera {
        let theta = rtweekend::degrees_to_radians(vfoV);
        let h = (theta/2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (look_from - look_at).unit_vector();
        let u= vup.cross(w).unit_vector();
        let v = w.cross(u);

        let origin = look_from;
        let horizontal= viewport_width * u;
        let vertical = viewport_height * v;
        let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - w;


        return Camera { origin, horizontal, vertical, lower_left_corner };
   }

   pub fn get_ray(&self, u: f32, v: f32) -> Ray {
       return Ray::new(self.origin, self.lower_left_corner + u*self.horizontal + v*self.vertical - self.origin  );
   }
}