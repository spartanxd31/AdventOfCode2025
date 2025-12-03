use core::num;
use std::env;
use std::fs;
use std::vec;

fn part1() -> u64 {
    let mut contents =
        fs::read_to_string("./input.txt").expect("Should have been able to read the file");

    // to get rid of the new line which is for somewhere there.
    contents.pop();
    let ranges: Vec<&str> = contents.split(",").collect();
    let mut sum: u64 = 0;

    for range in ranges {
        let bounds: Vec<&str> = range.split("-").collect();
        let lower: u64 = bounds[0].parse().expect("Not a valid number");
        let upper: u64 = bounds[1].parse().expect("Not a valid number");
        for i in lower..(upper + 1) {
            let num_string = i.to_string();
            // split in half
            let num_len = num_string.len();
            if num_len.rem_euclid(2) == 0 {
                let (pair_one, pair_two) = num_string.split_at(num_len / 2);
                if pair_one == pair_two {
                    sum += i;
                }
            }
        }
    }
    sum
}

fn part2() -> u64 {
    let mut contents =
        fs::read_to_string("./input.txt").expect("Should have been able to read the file");

    // to get rid of the new line which is for somewhere there.
    contents.pop();
    let ranges: Vec<&str> = contents.split(",").collect();
    let mut sum: u64 = 0;

    for range in ranges {
        let bounds: Vec<&str> = range.split("-").collect();
        let lower: u64 = bounds[0].parse().expect("Not a valid number");
        let upper: u64 = bounds[1].parse().expect("Not a valid number");
        for i in lower..(upper + 1) {
            let num_string = i.to_string();
            let length = num_string.len();
            let pair_match: bool = false;
            for j in 2..(length + 1) {
                //Divides evenly
                if length.rem_euclid(j) == 0 {
                    let pairs: Vec<String> = num_string
                        .chars()
                        .collect::<Vec<_>>()
                        .chunks(length / j)
                        .map(|chunk| chunk.iter().collect())
                        .collect();

                    let pair_match = pairs.windows(2).all(|x| x[0] == x[1]);
                    if pair_match {
                        sum += i;
                        break;
                    }
                }
                if pair_match {
                    break;
                }
            }
        }
    }
    sum
}
fn main() {
    let p1answer = part1();
    println!("part1 answer: {p1answer}");
    let p2answer = part2();
    println!("part1 answer: {p2answer}");
}
