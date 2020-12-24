use std::vec::Vec;
use crate::hittable::Hittable;
use crate::hittable::HitRecord;
use crate::ray::Ray;

pub struct HittableList {
    pub objcets: Vec<Box<dyn Hittable>>
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, hit_record: &mut HitRecord) -> bool {
        let mut temp_rec: HitRecord = HitRecord::new();
        let mut hit_aynthing = false;
        let mut closest_so_far = t_max;
        for obj in &self.objcets {
            if obj.hit(r, t_min, closest_so_far, &mut temp_rec) {
                hit_aynthing = true;
                closest_so_far = temp_rec.t;
                hit_record = temp_rec.Copy();
            }
        }
        return hit_aynthing;
    }
}