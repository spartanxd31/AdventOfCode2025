use std::env;
use std::fs;
use std::vec;
fn main() {
    let mut contents =
        fs::read_to_string("./input.txt").expect("Should have been able to read the file");

    // to get rid of the new line which is for somewhere there.
    contents.pop();
    let ranges: Vec<&str> = contents.split(",").collect();
    println!("{:?}", ranges);
    for range in ranges {
        let bounds: Vec<&str> = range.split("-").collect();
        let lower: u64 = bounds[0].parse().expect("Not a valid number");
        let upper: u64 = bounds[1].parse().expect("Not a valid number");
        println!("{lower}, {upper}");
    }
}
