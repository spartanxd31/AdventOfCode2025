use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn check_intersection(low1: u64, low2: u64, upper1: u64, upper2: u64) -> bool {
    low1 < upper2 && low2 < upper1
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let mut ranges: Vec<String> = Vec::new();
    let mut ids: Vec<String> = Vec::new();
    let mut switch = false;
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines.map_while(Result::ok) {
            if switch {
                ids.push(line);
            } else if line == "" {
                switch = true;
            } else {
                ranges.push(line);
            }
        }
    }
    //Part 1
    let mut fresh = 0;
    for id in ids {
        let id_num: u64 = id.parse().expect("This should be a number");
        for range in &ranges {
            let bounds: Vec<&str> = range.split("-").collect();
            let upper_number: u64 = bounds[1]
                .to_string()
                .parse()
                .expect("This should be a number");
            let lower_number: u64 = bounds[0]
                .to_string()
                .parse()
                .expect("This should be a number");

            if id_num >= lower_number && id_num <= upper_number {
                fresh += 1;
            }
        }
    }
    println!("{fresh}");
}
