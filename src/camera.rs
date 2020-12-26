use crate::Vec3 as Point3;
use crate::Vec3;
use crate::ray::Ray;
use crate::rtweekend;
use crate::vec3::random_in_unit_disk;

pub struct Camera {
    origin: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f32
}


impl Camera {
    pub fn new(look_from: Point3, look_at: Point3, vup: Vec3, vfo_v: f32, aspect_ratio: f32, aperture: f32, focus_dist: f32) -> Camera {
        let theta = rtweekend::degrees_to_radians(vfo_v);
        let h = (theta/2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (look_from - look_at).unit_vector();
        let u= vup.cross(w).unit_vector();
        let v = w.cross(u);

        let origin = look_from;
        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;
        let lower_left_corner = origin - horizontal/2.0  - vertical/2.0 - focus_dist*w;
        let lens_radius = aperture / 2.0;



        return Camera { origin, horizontal, vertical, lower_left_corner, w, u, v, lens_radius };
   }

   pub fn get_ray(&self, s: f32, t: f32) -> Ray {
       let rd = self.lens_radius * random_in_unit_disk();
       let offset = self.u * rd.x + self.v * rd.y;
        return Ray::new(self.origin + offset, self.lower_left_corner + s*self.horizontal + t*self.vertical - self.origin -  offset);

   }
}