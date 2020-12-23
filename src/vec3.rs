extern crate overload;
use overload::overload;
use std::ops;
use Vec3 as Color;


#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Vec3 {
   pub x: f32,
   pub y: f32,
   pub z: f32
}

overload!((a: Vec3) + (b: Vec3) -> Vec3 { Vec3 { x: a.x + b.x, y: a.y + b.y, z: a.z + b.z }});
overload!((a: &mut Vec3) += (b: Vec3)  { a.x += b.x; a.y += b.y; a.z += b.z});
overload!((a: &mut Vec3) *= (b: Vec3) { a.x *= b.x; a.y *= b.y; a.z *= b.z; });
overload!((a: &mut Vec3) *= (b: f32) { a.x *= b; a.y *= b; a.z *= b; });
overload!((a: &mut Vec3) /= (b: f32) { a.x *= 1.0/b; a.y *= 1.0/b; a.z *= 1.0/b });
overload!((a: Vec3) - (b: Vec3) -> Vec3 { Vec3 { x: a.x - b.x, y: a.y - b.y, z: a.z - b.z }});
overload!((a: Vec3) * (b: Vec3) -> Vec3 { Vec3 { x: a.x * b.x, y: a.y * b.y, z: a.z * b.z }});
overload!((a: Vec3) * (b: f32) -> Vec3 { Vec3 { x: a.x * b, y: a.y * b, z: a.z * b }});
overload!((b: f32) * (a: Vec3)  -> Vec3 { Vec3 { x: a.x * b, y: a.y * b, z: a.z * b }});
overload!((a: Vec3) / (b: Vec3) -> Vec3 { Vec3 { x: a.x / b.x, y: a.y / b.y, z: a.z / b.z }});
overload!((a: Vec3) / (b: f32) -> Vec3 { Vec3 { x: a.x * (1.0/b), y: a.y * (1.0/b), z: a.z * (1.0/b) }});
overload!(- (a: Vec3) -> Vec3 {  Vec3 { x: -a.x, y: -a.y, z: -a.z }  });

pub fn write_color(pixel: Color) {
      println!("{0} {1} {2}", ((pixel.x * 255.9999) as i32), ((pixel.y * 255.9999) as i32), ((pixel.z * 255.9999) as i32));
}

impl Vec3 {
       pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn length_squared(&self) -> f32 {
       return &self.x * &self.x + &self.y * &self.y + &self.z * &self.z
    }

    pub fn length(&self) -> f32 {
       return *&self.length_squared().sqrt();
    }

    pub fn unit_vector(&self) -> Vec3 {
      *self / *&self.length()
   }

   pub fn dot(&self, v: Vec3) -> f32{
      return self.x * v.x + self.y * v.y + self.z * self.z;  
   }

   pub fn cross(&self, v: Vec3) -> Vec3 {
      return Vec3{x: self.y * v.z - self.z * v.y, 
                  y: self.z * v.x - self.x * v.z, 
                  z: self.x * v.y - self.y * v.x };
   }
}



#[cfg(test)]
mod tests {
   use crate::vec3::Vec3;

   #[test]
   fn tests() {
      let mut a = Vec3 {x: 1.0, y: 1.0, z: 1.0};
      let b = Vec3 {x: 2.0, y: 2.0, z: 2.0}; 
      let c = Vec3 {x: 3.0, y: 3.0, z: 3.0};
      let d = Vec3 {x: 0.0, y: 0.0, z: 0.0};
      let e = Vec3 {x: 0.0, y: 0.0, z: 0.0};
      let f = Vec3 {x: 4.0, y: 4.0, z: 4.0};
      let g = Vec3 {x: 2.0, y: 1.0, z: 2.0};
      let h = Vec3 {x: -3.0, y: 0.0, z: 3.0};
      assert_eq!(a * -1.0, -a);
      assert_eq!(d, e);
      assert_eq!(a + a, b);
      assert_eq!(a + a, b);
      assert_eq!(a+b, c);
      assert_eq!(a * b, b);
      assert_eq!(b * b, f);
      assert_eq!(e * a, e);
      a *= 2.0;
      assert_eq!(a, b);
      a *= 0.5;
      assert_eq!(a * 2.0, b);
      assert_eq!(2.0 * a, b);
      assert_eq!(b.dot(b), 12.0);
      assert_eq!(b.cross(b), e);
      assert_eq!(g.cross(c), h);
   }
}