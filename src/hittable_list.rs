use std::vec::Vec;
use crate::hittable::Hittable;
use crate::hittable::HitRecord;
use crate::ray::Ray;

// https://github.com/akinnane/RayTracingInOneWeekend
pub struct HittableList {
    objects: Vec<Box<dyn Hittable + Send + Sync>>,
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let mut temp_rec: HitRecord = HitRecord::new();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;
        for obj in &self.objects {
            if obj.hit(r, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec.clone();
            } 
        }
        return hit_anything;
    }
}

impl HittableList {
    pub fn new() -> HittableList {
        return HittableList { objects: Vec::new() }
    }

    pub fn add(&mut self, object: Box<dyn Hittable + Send + Sync>) {
        self.objects.push(object);
    }
}