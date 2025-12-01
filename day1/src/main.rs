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

// TODO:get rotations from txt file and process them one by one updating the where the dial of the
// lock currently is
// NOTE: the password is the number of times the dial left points at 0 after any rotation in the
// sequence
fn main() {
    let mut pass = 0;
    let mut dial: i32 = 50;

    if let Ok(lines) = read_lines("./input.txt") {
        for mut line in lines.map_while(Result::ok) {
            // println!("{}", line);
            let num = line.split_off(1).parse::<i32>().unwrap();
            // println!("rotate {}, num {}", line, num);
            if line == "R" {
                dial = dial + num;
                dial = dial.rem_euclid(100);
            } else {
                dial = dial - num;
                dial = dial.rem_euclid(100);
            }
            // println!("{dial}");
            if dial == 0 {
                pass = pass + 1;
            }
        }
    }
    println!("{pass}");
}
