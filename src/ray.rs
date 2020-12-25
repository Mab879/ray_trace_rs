use crate::vec3::Vec3;
use Vec3 as Point3;

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Ray {
        Ray { origin, direction }
    }

    pub fn at(&self, t: f32) -> Point3 {
        return self.origin + t * self.direction;
    }
}