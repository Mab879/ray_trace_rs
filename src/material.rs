use crate::ray::Ray;
use crate::vec3::Vec3 as Color;
use crate::hittable::HitRecord;
use crate::vec3;
use crate::rtweekend::random_double;
use rand::thread_rng;

#[derive(Clone, Copy)]
pub enum Material {
    Lambertian { albendo: Color },
    Metal { albendo: Color, fuzz: f32 },
    Dielectric { ir: f32 },
}

impl Default for Material {
    fn default() -> Material {
        Material::Lambertian {
            albendo: Color::new(0.0, 0.0, 0.0),
        }
    }
}

impl Material {
    pub fn scatter(self, ray_in: &Ray, hit_record: HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        return match self {
            Material::Lambertian { albendo } => {
                Self::scatter_lambertian(&albendo, ray_in, hit_record, attenuation, scattered)
            }
            Material::Metal { albendo, fuzz } => {
                Self::scatter_metal(&albendo, fuzz, ray_in, hit_record, attenuation, scattered)
            }
            Material::Dielectric { ir } => {
                Self::scatter_dielectric(ir, ray_in, hit_record, attenuation, scattered)
            }
        }
    }

    fn scatter_lambertian(&albendo: &Color, _ray_in: &Ray, hit_record: HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        let mut scatter_direction = hit_record.normal + vec3::random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = hit_record.normal;
        }
        *scattered = Ray::new(hit_record.p, scatter_direction);
        *attenuation = albendo;
        return true;
    }

    fn scatter_metal(&albendo: &Color, fuzz: f32, _ray_in: &Ray, hit_record: HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        let reflected: vec3::Vec3 = _ray_in.direction.unit_vector().reflect(hit_record.normal);
        *scattered = Ray::new(hit_record.p, reflected + fuzz * vec3::random_in_unit_sphere());
        *attenuation = albendo;
        return scattered.direction.dot(hit_record.normal) > 0.0;
    }

    fn scatter_dielectric(ir: f32, _ray_in: &Ray, hit_record: HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        let mut rng = thread_rng();
        *attenuation = Color::new(1.0, 1.0, 1.0);
        let mut refraction_ratio = ir;
        if hit_record.front_face {
            refraction_ratio = 1.0/ir;
        }
        let unit_direction = _ray_in.direction.unit_vector();
        let cos_theta = (-unit_direction.dot(hit_record.normal)).min(1.0);
        let sin_theta = (1.0 - cos_theta*cos_theta).sqrt();
        let cannot_refract: bool = refraction_ratio * sin_theta > 1.0;
        let mut direction = unit_direction.refract(&hit_record.normal, refraction_ratio);
        let rand_num = random_double(&mut rng);
        if cannot_refract || Self::dielectric_reflectance(cos_theta, refraction_ratio) > rand_num  {
            direction = unit_direction.reflect(hit_record.normal);
        }
        *scattered = Ray::new(hit_record.p, direction);
        return true;
    }

    fn dielectric_reflectance(consine: f32, ref_idx: f32) -> f32 {
        let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        r0 = r0 * r0;
        return r0 + (1.0 - r0) * (1.0-consine).powf(5.0);
    }
}

