mod vec3;
mod ray;
use vec3::Vec3 as Color;


fn main() {
    const WIDTH: i32 = 1024;
    const HEIGHT: i32 = 1024;
    print!("P3\n{0} {1}\n255\n", WIDTH, HEIGHT);
    let mut j = WIDTH;
    eprintln!("Lines remaining {0}", j);
    while j > 0 {
        eprintln!("Lines remaining {0}", j);
        for i in 0..HEIGHT {
            let c = Color { x: (i as f32)/((WIDTH-1) as f32), y: (j as f32)/((HEIGHT-1) as f32), z: 0.25};
            vec3::write_color(c);
        }
        j = j -1;
    }
}
