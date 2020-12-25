use crate::Vec3 as Point3;
use crate::Vec3;
use crate::ray::Ray;

pub struct Camera {
    origin: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Vec3,
}


impl Camera {
    pub fn new() -> Camera {
        let ascpect_ratio = 16.0/9.0;
        let viewport_height = 2.0;
        let viewport_width = ascpect_ratio * viewport_height;
        let focal_legth = 1.0;

        let origin = Point3::new(0.0, 0.0, 0.0);
        let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
        let vertical = Vec3::new(0.0, viewport_height, 0.0);
        let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - Vec3::new(0.0,0.0,focal_legth);


        return Camera { origin, horizontal, vertical, lower_left_corner };
   }

   pub fn get_ray(&self, u: f32, v: f32) -> Ray {
       return Ray::new(self.origin, self.lower_left_corner + u*self.horizontal + v*self.vertical - self.origin  );
   }
}