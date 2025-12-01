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
// NOTE: the password is the number of times the dial left points at 0 after any rotation in the
// sequence
fn part1() -> i32 {
    let mut pass = 0;
    let mut dial: i32 = 50;

    if let Ok(lines) = read_lines("./test.txt") {
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

    pass
}

//NOTE: You remember from the training seminar that "method 0x434C49434B" means you're actually supposed to count the number of times any click causes the dial to point at 0, regardless of whether it happens during a rotation or at the end of one.
fn part2() -> i32 {
    let mut pass = 0;
    let mut dial: i32 = 50;

    // let mut rem: i32;
    if let Ok(lines) = read_lines("./input.txt") {
        for mut line in lines.map_while(Result::ok) {
            let num_str = line.split_off(1);
            let num = num_str.trim().parse::<i32>().unwrap();

            let moves;

            if line == "R" {
                let start = dial;
                let end = dial + num;

                moves = end / 100 - start / 100;

                dial = end.rem_euclid(100);
            } else {
                let start = dial;
                let end = dial - num;

                //stupif off by one error
                moves = (end - 1).div_euclid(100) - (start - 1).div_euclid(100);

                dial = end.rem_euclid(100);
            }

            pass += moves.abs();
        }
    }
    println!("{pass}");
    pass
}

fn main() {
    let p1_answer = part1();
    println!("part 1 answer: {p1_answer}");
    let p2_answer = part2();

    print!("part 2 answer: {p2_answer}");
}
