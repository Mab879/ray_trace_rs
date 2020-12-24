use crate::vec3::Vec3;
use crate::Vec3 as Point3;
use crate::ray::Ray;

#[derive(Clone, Copy)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool
}

impl HitRecord {
    pub fn new() -> HitRecord {
        let p = Point3::new(0.0, 0.0, 0.0);
        let normal = Vec3::new(0.0, 0.0, 0.0);
        return HitRecord {p, normal, t: 0.0, front_face: false};
    }

    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = r.direction.dot(*outward_normal) < 0.0;
        if self.front_face 
        { self.normal = *outward_normal 
        } else { 
            self.normal = -*outward_normal
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, hit_record: &mut HitRecord) -> bool;
}