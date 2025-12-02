use core::num;
use std::env;
use std::fs;
use std::vec;
fn main() {
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
                    // sum += num_string.parse::<u64>().expect("Should be an u64");
                    sum += i;
                }
            }
        }
    }
    println!("{sum}");
}
