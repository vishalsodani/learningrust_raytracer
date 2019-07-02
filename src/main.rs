use std::fs::File;
use std::error::Error;
use std::path::Path;
use std::io::Write;

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
            let r: f32 = ( m as f32 / nx as f32 ) as f32;
            let g: f32 = (n as f32 / ny as f32) as f32;
            let b: f32 = 0.2;

            println!("{}", r);

            let ir: i32 = (255.99 * r) as i32;
            let ig: i32 = (255.99 * g) as i32;
            let ib: i32 = (255.99 * b) as i32;

            write!(file, "{} {} {}\n", ir, ig, ib)?;
        }
    }

    Ok(())
}
