use std::fs::File;
use std::path::Path;
use std::io::Write;
use hello_graphics::raytracer::Vec3;

fn main() -> Result<(), std::io::Error> {
    let nx = 200;
    let ny = 100;

    let path = Path::new("hello.ppm");

    let mut file = File::create(&path)?;
    write!(file, "P3\n")?;
    write!(file, "{} {}\n", nx, ny)?;
    write!(file, "{}\n", 255)?;

    let j = ny - 1;
    let i = nx;

    for n in (0..j).rev() {
        for m in 0..i {
            let vec3 = Vec3 { 
                            e1: m as f32 / nx as f32,  
                            e2: n as f32 / ny as f32,
                            e3: 0.2};

            let ir: i32 = (255.99 * vec3.e1) as i32;
            let ig: i32 = (255.99 * vec3.e2) as i32;
            let ib: i32 = (255.99 * vec3.e3) as i32;

            write!(file, "{} {} {}\n", ir, ig, ib)?;
        }
    }

    Ok(())
}
