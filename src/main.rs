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
use crate::rtweekend::clamp;
use crate::rtweekend::random_double;
use rand::prelude::thread_rng;
use crate::vec3::random_unit_vector;
use ray::Ray;
use crate::material::Material;

const ASPECT_RATIO: f32 = 16.0/9.0;
const WIDTH: i32 = 400;
const HEIGHT: i32 = (WIDTH as f32 / ASPECT_RATIO) as i32;
const SAMPLES_PER_PIXEL: i32 = 100;
const MAX_DEPTH: i32 = 50;

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


// auto material_ground = make_shared<lambertian>(color(0.8, 0.8, 0.0));
// auto material_center = make_shared<lambertian>(color(0.7, 0.3, 0.3));
// auto material_left   = make_shared<metal>(color(0.8, 0.8, 0.8));
// auto material_right  = make_shared<metal>(color(0.8, 0.6, 0.2));
//
// world.add(make_shared<sphere>(point3( 0.0, -100.5, -1.0), 100.0, material_ground));
// world.add(make_shared<sphere>(point3( 0.0,    0.0, -1.0),   0.5, material_center));
// world.add(make_shared<sphere>(point3(-1.0,    0.0, -1.0),   0.5, material_left));
// world.add(make_shared<sphere>(point3( 1.0,    0.0, -1.0),   0.5, material_right));

fn main() {
    // World
    let mut world: HittableList = HittableList::new();

    let material_ground= Box::<Material>::new(Material::Lambertian {albendo: Color::new(0.8, 0.8, 0.0)});
    let material_center = Box::<Material>::new(Material::Lambertian { albendo: Color::new (0.7, 0.3, 0.3)});
    let material_left = Box::<Material>::new(Material::Metal {albendo: Color::new(0.8, 0.8, 0.8), fuzz: 0.3});
    let material_right = Box::<Material>::new(Material::Metal { albendo: Color::new(0.8, 0.6, 0.2), fuzz: 1.0});

    world.add(Box::<Sphere>::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, material_ground)));
    world.add(Box::<Sphere>::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5, material_center)));
    world.add(Box::<Sphere>::new(Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5, material_left)));
    world.add(Box::<Sphere>::new(Sphere::new(Point3::new(1.0, 0.0, -1.0), 0.5, material_right)));


    // Camera
    let camera = Camera::new();

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
