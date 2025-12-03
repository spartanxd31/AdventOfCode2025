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
    println!("{sum}");
}
