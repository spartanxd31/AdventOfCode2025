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
#[derive(PartialEq, Copy, Clone, Debug)]
struct Coords {
    x: f64,
    y: f64,
    z: f64,
}

impl Coords {
    fn from_str(s: &str) -> Option<Self> {
        let parts: Vec<f64> = s.split(',').filter_map(|p| p.trim().parse().ok()).collect();

        if parts.len() == 3 {
            Some(Coords {
                x: parts[0],
                y: parts[1],
                z: parts[2],
            })
        } else {
            None
        }
    }
}

#[derive(PartialEq, Copy, Clone, Debug)]
struct WirePairs {
    item1: Coords,
    item2: Coords,
    dis: f64,
}

fn distance(a: Coords, b: Coords) -> f64 {
    let dx = (a.x - b.x).powf(2.0);
    let dy = (a.y - b.y).powf(2.0);
    let dz = (a.z - b.z).powf(2.0);
    (dx + dy + dz).sqrt()
}

fn main() {
    if let Ok(lines) = read_lines("./test.txt") {
        let coords_list: Vec<_> = lines
            .filter_map(|line| Coords::from_str(&line.ok()?))
            .collect();
        let junction_boxs: Vec<Vec<WirePairs>> = Vec::new();
        let mut pairs: Vec<WirePairs> = Vec::new();
        for ca in &coords_list {
            for cb in coords_list.clone() {
                let some_pair = WirePairs {
                    item1: *ca,
                    item2: cb,
                    dis: distance(*ca, cb),
                };
                pairs.push(some_pair);
            }
        }
        println!("{}", pairs.len());
        pairs.sort_by(|a, b| a.dis.partial_cmp(&b.dis).expect("Something"));

        if junction_boxs.contains(pairs[0]) {
            println!("Yes")
        } else {
            println!("no");
        }
    }
}
