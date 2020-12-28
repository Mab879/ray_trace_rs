use crate::Vec3 as Point3;
use crate::hittable::Hittable;
use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::material::Material;

#[derive(Clone)]
pub struct Sphere {
    center: Point3,
    radius: f32,
    material: Box<Material>,
    radius_2: f32
}


impl Sphere {
    pub fn new(center: Point3, radius: f32, material: Box<Material>) -> Sphere {
        let radius_2 = radius * radius;
        Sphere { center, radius, material, radius_2 }
    }

}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, hit_record: &mut HitRecord) -> bool {
        let oc = r.origin - self.center;
        let a: f32 = r.direction.length_squared();
        let half_b: f32 = oc.dot(r.direction);
        let c: f32 = oc.length_squared() - self.radius_2;
        let discrimiant = half_b * half_b - a*c;
        if discrimiant < 0.0 {
            return false;
        }
        let sqrtd = discrimiant.sqrt();
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return false;
            }
        }
        hit_record.t = root;
        hit_record.p = r.at(hit_record.t);
        hit_record.material = self.material.clone();
        let outward_normal = (hit_record.p - self.center) / self.radius;
        hit_record.set_face_normal(r, &outward_normal);
        return true;
    }
}