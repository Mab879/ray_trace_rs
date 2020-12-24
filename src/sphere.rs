use crate::Vec3 as Point3;
use crate::hittable::Hittable;
use crate::hittable::HitRecord;
use crate::ray::Ray;

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Sphere {
    center: Point3,
    radius: f32,
}


impl Sphere {
    pub fn new(center: Point3, radius: f32) -> Sphere {
        Sphere { center, radius }
    }

}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, hit_record: &mut HitRecord) -> bool {
        let oc = r.origin - self.center;
        let a: f32 = r.direction.dot(r.direction);
        let half_b: f32 = 2.0 * oc.dot(r.direction);
        let c: f32 = oc.dot(oc) - self.radius * self.radius;
        let discrimiant = half_b * half_b - 4.0*a*c;
        if discrimiant < 0.0 {
            return false;
        }
        let sqrtd = discrimiant.sqrt();
        let root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            return false;
        }
        hit_record.t = root;
        hit_record.p = r.at(hit_record.t);
        let outward_normal = (hit_record.p - self.center) / self.radius;
        hit_record.set_face_normal(r, &outward_normal);
        hit_record.normal = (hit_record.p - self.center) / self.radius;
        return true;
    }
}