use crate::ray::Ray;
use crate::vec3::Vec3 as Color;
use crate::hittable::HitRecord;
use crate::vec3;

#[derive(Clone, Copy)]
pub enum Material {
    Lambertian { albendo: Color },
    Metal { albendo: Color, fuzz: f32 }
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
        let mut reflected: vec3::Vec3 = _ray_in.direction.unit_vector().reflect(hit_record.normal);
        *scattered = Ray::new(hit_record.p, reflected + fuzz * vec3::random_in_unit_sphere());
        *attenuation = albendo;
        return scattered.direction.dot(hit_record.normal) > 0.0;

    }
}

