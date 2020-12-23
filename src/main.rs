mod vec3;
mod ray;
use vec3::Vec3 as Color;
use vec3::Vec3 as Point3;
use crate::vec3::Vec3;

fn ray_color(ray: ray::Ray) -> Color {
    let unit_direction = ray.direction.unit_vector();
    let t = 0.5 * (unit_direction.y +1.0);
    return (1.0-t) * Color { x: 1.0, y: 1.0, z: 1.0 } + t * Color { x: 0.5, y: 0.7, z: 1.0 };
}

fn main() {
    const ASPECT_RATIO: f32 = 16.0/9.0;


    const WIDTH: i32 = 400;
    const HEIGHT: i32 = (WIDTH as f32 / ASPECT_RATIO) as i32;


    let viewport_height: f32 = 2.0;
    let viewport_width = (ASPECT_RATIO * viewport_height) as i32;
    let focal_legth = 1.0;

    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Point3 { x: viewport_width as f32, y: 0.0, z: 0.0 };
    let vertical = Point3 {x: 0.0,  y: viewport_height, z: 0.0};
    let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - Vec3 { x: 0.0, y: 0.0, z: focal_legth };

    print!("P3\n{0} {1}\n255\n", WIDTH, HEIGHT);
    let mut j = HEIGHT;
    eprintln!("Lines remaining {0}", j);
    while j > 0 {
        eprintln!("Lines remaining {0}", j);
        for i in 0..WIDTH {
            let u = i as f32 / (WIDTH-1) as f32;
            let v = j as f32 / (HEIGHT-1) as f32;
            let r = ray::Ray { origin: origin, direction:  lower_left_corner + u*horizontal + v*vertical - origin};
            let c = ray_color(r);
            vec3::write_color(c);
        }
        j = j -1;
    }
}
