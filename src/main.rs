use std::fs::File;
use std::error::Error;
use std::path::Path;

fn main() -> Result<(), std::io::Error> {
    let nx = 200;
    let ny = 100;

    let path = Path::new("hello.ppm");

    let mut file = File::create(&path)?;

    Ok(())
}
