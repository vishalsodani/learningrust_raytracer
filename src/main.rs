use hello_graphics::raytracer::Ray;
use hello_graphics::raytracer::Vec3;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main() -> Result<(), std::io::Error> {
    let nx = 200;
    let ny = 100;

    let path = Path::new("hello.ppm");

    let mut file = File::create(&path)?;
    writeln!(file, "P3")?;
    writeln!(file, "{} {}", nx, ny)?;
    writeln!(file, "{}", 255)?;

    let lower_left_corner: Vec3 = Vec3 {
        e1: -2.0,
        e2: -1.0,
        e3: -1.0,
    };
    let horizontal: Vec3 = Vec3 {
        e1: 4.0,
        e2: 0.0,
        e3: 0.0,
    };
    let vertical: Vec3 = Vec3 {
        e1: 0.0,
        e2: 2.0,
        e3: 0.0,
    };
    let origin: Vec3 = Vec3 {
        e1: 0.0,
        e2: 0.0,
        e3: 0.0,
    };

    let j = ny - 1;
    let i = nx;

    for n in (0..j).rev() {
        for m in 0..i {
            let u: f32 = m as f32 / nx as f32;
            let v: f32 = n as f32 / ny as f32;
            let r: Ray = Ray {
                A: origin,
                B: lower_left_corner + (horizontal * u) + (vertical * v),
            };
            let col: Vec3 = r.color();
            let ir: i32 = (255.99 * col.e1) as i32;
            let ig: i32 = (255.99 * col.e2) as i32;
            let ib: i32 = (255.99 * col.e3) as i32;

            writeln!(file, "{} {} {}", ir, ig, ib)?;
        }
    }

    Ok(())
}
