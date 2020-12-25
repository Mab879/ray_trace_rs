use rand::prelude::*;


pub const INFINTY: f32 = std::f32::INFINITY;
pub const PI: f32 = std::f32::consts::PI;

pub fn degrees_to_radians(degrees: f32) -> f32 {
    return degrees * PI / 180.0;
}

pub fn random_double(rng: &mut ThreadRng) -> f32 {
    return rng.gen_range(0.0, 1.0);
}

pub fn random_double_range(rng: &mut ThreadRng, min: f32, max:f32) -> f32 {
   return rng.gen_range(min, max);
}

pub fn clamp(x: f32, min: f32, max: f32) -> f32 {
    if x < min { return min; }
    if x > max { return max; }
    return x
}