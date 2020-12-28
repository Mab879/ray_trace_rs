mod vec3;
mod ray;
mod sphere;
mod hittable;
mod hittable_list;
mod rtweekend;
mod camera;
mod material;

use vec3::Vec3 as Color;
use vec3::Vec3 as Point3;
use crate::vec3::Vec3;
use crate::sphere::Sphere;
use crate::hittable_list::HittableList;
use crate::hittable::HitRecord;
use crate::hittable::Hittable;
use crate::camera::Camera;
use crate::rtweekend::{clamp, random_double_range};
use crate::rtweekend::random_double;
use rand::prelude::thread_rng;
use ray::Ray;
use crate::material::Material;
use crate::material::Material::{Lambertian, Metal, Dielectric};

const ASPECT_RATIO: f32 = 16.0/9.0;
const WIDTH: i32 = 1200;
const HEIGHT: i32 = (WIDTH as f32 / ASPECT_RATIO) as i32;
const SAMPLES_PER_PIXEL: i32 = 501;
const MAX_DEPTH: i32 = 50;

fn random_scene() -> HittableList {
    let mut world = HittableList::new();
    let mut rng = thread_rng();
    let ground_material = Box::<Material>::new(Lambertian {albendo: Color::new(0.5, 0.5, 0.5)});
    world.add(Box::<Sphere>::new(Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, ground_material)));
    let check_point = Point3::new(4.0, 0.2, 0.0);

    for a in -11..11 {
        for b in -11..11 {
            let material_choice = random_double(&mut rng);
            let center = Point3::new((a as f32) - 0.9 * random_double(&mut rng), 0.2, (b as f32) - 0.9 * random_double(&mut rng));
            if (center - check_point).length() > 0.9 {
                let sphere_material: Box<Material>;
                if material_choice < 0.8 {
                    let albendo = vec3::random() * vec3::random();
                    sphere_material = Box::<Material>::new(Lambertian { albendo });
                    world.add(Box::<Sphere>::new(Sphere::new(center, 0.2, sphere_material)));
                } else if material_choice < 0.95 {
                    let albendo = vec3::random() * vec3::random();
                    let fuzz = random_double_range(&mut rng, 0.0, 0.5);
                    sphere_material = Box::<Material>::new(Metal { albendo, fuzz });
                    world.add(Box::<Sphere>::new(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    sphere_material = Box::<Material>::new(Dielectric { ir: 1.5 });
                    world.add(Box::<Sphere>::new(Sphere::new(center, 0.2, sphere_material)))
                }
            }
        }
    }
    let material_1 = Box::<Material>::new(Dielectric {ir: 1.5});
    world.add(Box::<Sphere>::new(Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, material_1)));

    let material_2 =  Box::<Material>::new( Lambertian { albendo: Color::new(0.4, 0.2, 0.1) });
    world.add(Box::<Sphere>::new(Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, material_2)));

    let material_3 = Box::<Material>::new(Metal { albendo: Color::new(0.7, 0.6, 0.5), fuzz: 0.0 });
    world.add(Box::<Sphere>::new(Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, material_3)));
    return world;
}

pub fn write_color(pixel: Color) {
    let scale = 1.0/(SAMPLES_PER_PIXEL as f32);
    let r = (pixel.x * scale).sqrt();
    let g = (pixel.y * scale).sqrt();
    let b = (pixel.z * scale).sqrt();
    println!("{0} {1} {2}", ((clamp(r, 0.0, 0.999) * 256.0) as i32), ((clamp(g, 0.0, 0.999) * 256.0) as i32), ((clamp(b, 0.0, 0.999) * 256.0) as i32));
}

fn ray_color(ray: ray::Ray, world: &HittableList, depth: i32) -> Color {
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    let mut rec = HitRecord::new();
    if world.hit(&ray, 0.001, rtweekend::INFINTY, &mut rec) {
        let mut scattered: Ray = Ray {
            origin: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0
            },
            direction: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0
            }
        };
        let mut attenuation: Color = Color::new(0.0, 0.0, 0.0);
        if rec.material.scatter(&ray, rec, &mut attenuation, &mut scattered) {
            return attenuation * ray_color(scattered, world, depth-1);
        }
        return Color::new(0.0, 0.0, 0.0);
    }
    let unit_direction = &ray.direction.unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);
    return (1.0-t) * Color { x: 1.0, y: 1.0, z: 1.0 } + t * Color { x: 0.5, y: 0.7, z: 1.0 };
}



fn main() {
    // World
    let world: HittableList = random_scene();


    // Camera
    let look_from = Point3::new(-13.0, 2.0, 3.0);
    let look_at  = Point3::new(0.0,0.0,0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let vfo_v: f32 = 20.0;
    let aperture = 0.1;
    let focus_dist  = 10.0;

    let camera=  Camera::new(look_from, look_at, vup, vfo_v, ASPECT_RATIO, aperture, focus_dist);

    print!("P3\n{0} {1}\n255\n", WIDTH, HEIGHT);
    let mut j = HEIGHT-1;
    let mut ran = thread_rng();
    while j >= 0 {
        eprintln!("Lines remaining {0}", j);
        for i in 0..WIDTH {
            let mut pixle_color = Color::new(0.0, 0.0, 0.0);
            for _s in 0..SAMPLES_PER_PIXEL {
                let u = ((i as f32) + random_double(&mut ran))  / (WIDTH-1) as f32;
                let v = ((j as f32) + random_double(&mut ran)) / (HEIGHT-1) as f32;
                let r = camera.get_ray(u, v);
                pixle_color += ray_color(r, &world, MAX_DEPTH);
            }
            write_color(pixle_color);
        }
        j = j -1;
    }
}
