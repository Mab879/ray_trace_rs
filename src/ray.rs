use crate::vec3::Vec3;
use Vec3 as Point3;

#[derive(PartialEq, Debug, Clone, Copy)]
struct Ray {
    origin: Point3,
    direction: Vec3,
}

impl Ray {
    pub fn at(&self, t: f32) -> Point3 {
        return self.origin + t * self.direction;
    }
}