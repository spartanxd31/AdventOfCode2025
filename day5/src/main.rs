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
    //part2
    let mut parsed_ranges: Vec<(u64, u64)> = Vec::new();
    for range in ranges {
        let bounds: Vec<&str> = range.split("-").collect();
        let lower: u64 = bounds[0]
            .to_string()
            .parse()
            .expect("This should be a range");
        let upper: u64 = bounds[1]
            .to_string()
            .parse()
            .expect("This should be a range");
        parsed_ranges.push((lower, upper));
    }

    // Sort ranges by lower bound
    parsed_ranges.sort_by_key(|&(low, _)| low);

    let mut merged: Vec<(u64, u64)> = Vec::new();
    for (lower, upper) in parsed_ranges {
        if merged.is_empty() {
            merged.push((lower, upper));
        } else {
            let last_idx = merged.len() - 1;
            let (last_lower, last_upper) = merged[last_idx];

            if lower <= last_upper + 1 {
                merged[last_idx] = (last_lower, std::cmp::max(last_upper, upper));
            } else {
                merged.push((lower, upper));
            }
        }
    }

    let mut total = 0;
    for (lower, upper) in merged {
        total += upper - lower + 1;
    }
    println!("Part 2: {total}");
}
