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

fn part1() -> u64 {
    let mut sum: u64 = 0;
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines.map_while(Result::ok) {
            let mut max_num: u64 = 0;

            let b: Vec<char> = line.chars().collect();
            for i in 0..b.len() {
                //This second loop needs to change so that j is always ahead of i and it always
                //goes to the end of the line
                for j in i..b.len() {
                    let num = (b[i].to_string() + &b[j].to_string())
                        .parse::<u64>()
                        .expect("This should be a u64");
                    if num > max_num && i != j {
                        max_num = num;
                    }
                }
            }
            sum += max_num;
        }
    }
    sum
}
fn part2() -> u64 {
    let mut sum: u64 = 0;
    if let Ok(lines) = read_lines("./input.txt") {
        let mut max_num: u64;

        for line in lines.map_while(Result::ok) {
            let b: Vec<char> = line.chars().collect();
            let total_length: u64 = b.len() as u64;
            let mut big_num: String = "".to_string();
            let mut search_start: u64 = 0;
            let num_left = 12;
            for i in 0..num_left {
                let remaining: u64 = num_left - 1 - i;
                let search_end_index: u64 = total_length - 1 - remaining;
                let mut local_max: u64 = b[search_start as usize].to_digit(10).unwrap() as u64;
                let mut max_location = search_start;
                for t in (search_start)..=(search_end_index) {
                    let some_num = b[t as usize]
                        .to_string()
                        .parse::<u64>()
                        .expect("Shoud be u64");
                    if some_num > local_max {
                        local_max = some_num;
                        max_location = t;
                    }
                }
                big_num += &local_max.to_string();
                search_start = max_location + 1;
            }

            max_num = big_num.parse::<u64>().expect("Should be a u64");
            sum += max_num;
        }
    }
    sum
}
fn main() {
    let p1answer = part1();
    println!("{p1answer}");
    let p2answer = part2();
    println!("{p2answer}");
}
