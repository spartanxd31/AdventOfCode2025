use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

struct Coords {
    x: u64,
    y: u64,
    z: u64,
}

struct Electrical {
    loc: Vec<Coords>,
    dis: u64,
}

fn main() {
    println!("Hello, world!");
}
