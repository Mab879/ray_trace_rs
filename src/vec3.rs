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

overload!((a: Vec3) + (b: Vec3) -> Vec3 { Vec3 { x: a.x + b.x, y: a.y + b.y, z: a.x + b.z }});
overload!((a: &mut Vec3) += (b: Vec3)  { a.x += b.x; a.y += b.y; a.z += b.z});
overload!((a: &mut Vec3) *= (b: Vec3) { a.x *= b.x; a.y *= b.y; a.z *= b.z; });
overload!((a: &mut Vec3) *= (b: f32) { a.x *= b; a.y *= b; a.z *= b; });
overload!((a: &mut Vec3) /= (b: f32) { a.x *= 1.0/b; a.y *= 1.0/b; a.z *= 1.0/b });
overload!((a: Vec3) - (b: Vec3) -> Vec3 { Vec3 { x: a.x - b.x, y: a.y - b.y, z: a.x - b.z }});
overload!((a: Vec3) * (b: Vec3) -> Vec3 { Vec3 { x: a.x * b.x, y: a.y * b.y, z: a.x * b.z }});
overload!((a: Vec3) * (b: f32) -> Vec3 { Vec3 { x: a.x * b, y: a.y * b, z: a.x * b }});
overload!((a: Vec3) / (b: Vec3) -> Vec3 { Vec3 { x: a.x / b.x, y: a.y / b.y, z: a.x / b.z }});
overload!(- (a: Vec3) -> Vec3 {  Vec3 { x: -a.x, y: -a.y, z: -a.x }  });

pub fn dot(u: Vec3, v: Vec3) -> f32{
   return u.x * v.x + u.y * v.y + u.z * v.z;  
}

pub fn cross(u: Vec3, v: Vec3) -> Vec3 {
   return Vec3{x: u.y * v.z - u.z * v.y, 
               y: u.z * v.x - u.x * v.z, 
               z: u.x * v.x - u.y * v.x };
}

pub fn write_color(pixel: Color) {
      println!("{0} {1} {2}", ((pixel.x * 255.9999) as i32), ((pixel.y * 255.9999) as i32), ((pixel.z * 255.9999) as i32));
}

#[cfg(test)]
mod tests {
   use crate::vec3::Vec3;
   use crate::vec3::dot;
   use crate::vec3::cross;
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
      assert_eq!(dot(b, b), 12.0);
      assert_eq!(cross(b, b), e);
      assert_eq!(cross(g, c), h);
   }
}