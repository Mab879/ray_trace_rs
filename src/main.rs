mod vec3;
mod ray;
use vec3::Vec3 as Color;
use vec3::Vec3 as Point3;
use crate::vec3::Vec3;

fn hit_sphere(center: &Point3, radius: f32, r: &ray::Ray) -> f32 {
    let oc = r.origin - *center;
    let a: f32 = r.direction.dot(r.direction);
    let half_b: f32 = 2.0 * oc.dot(r.direction);
    let c: f32 = oc.dot(oc) - radius*radius;
    let discrimiant = half_b * half_b - 4.0*a*c;
    if discrimiant < 0.0 {
        return -1.0
    } else
    {
        return (-half_b - discrimiant.sqrt()) / (2.0*a);
    }
}


fn ray_color(ray: ray::Ray) -> Color {
    let t = hit_sphere(&Point3::new(0.0, 0.0, -1.0), 0.5, &ray);
    if t > 0.0  {
        let n = (ray.at(t) - Vec3::new(0.0, 0.0, -1.0)).unit_vector();
        return 0.5 * Color::new(n.x + 1.0,n.y + 1.0, n.z + 1.0);
    }
    let unit_direction = &ray.direction.unit_vector();
    let t = 0.5 * (unit_direction.y +1.0);
    return (1.0-t) * Color { x: 1.0, y: 1.0, z: 1.0 } + t * Color { x: 0.5, y: 0.7, z: 1.0 };
}


fn main() {
    const ASPECT_RATIO: f32 = 16.0/9.0;
    const WIDTH: i32 = 400;
    const HEIGHT: i32 = (WIDTH as f32 / ASPECT_RATIO) as i32;


    let viewport_height: f32 = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_legth = 1.0;

    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Point3 { x: viewport_width as f32, y: 0.0, z: 0.0 };
    let vertical = Point3 {x: 0.0,  y: viewport_height, z: 0.0};
    let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - Vec3 { x: 0.0, y: 0.0, z: focal_legth };
    print!("P3\n{0} {1}\n255\n", WIDTH, HEIGHT);
    let mut j = HEIGHT-1;
    eprintln!("Lines remaining {0}", j);
    while j >= 0 {
        eprintln!("Lines remaining {0}", j);
        for i in 0..WIDTH {
            let u = i as f32 / (WIDTH-1) as f32;
            let v = j as f32 / (HEIGHT-1) as f32;
            let r = ray::Ray { origin: origin, direction:  lower_left_corner + u*horizontal + v*vertical - origin};
            //eprintln!("rd {1}\n o {0}", origin, r.direction);
            let c = ray_color(r);
            vec3::write_color(c);
        }
        j = j -1;
    }
}
