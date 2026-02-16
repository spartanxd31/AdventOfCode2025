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

fn part1(filename: &str) -> u32 {
    let mut count = 0;
    //I don't know if I need this but it might be useful for keeping track
    if let Ok(lines) = read_lines(filename) {
        let mut prev_line: Vec<char> = Vec::new();
        for (index, line) in lines.enumerate() {
            if index == 0 {
                prev_line = line.expect("This should be a string").chars().collect();
            } else {
                let current_line: Vec<char> =
                    line.expect("This should be a string").chars().collect();
                let mut modified = current_line.clone();
                for (i, ch) in current_line.iter().enumerate() {
                    if prev_line[i] == 'S' {
                        modified[i] = '|';
                    } else if prev_line[i] == '|' && modified[i] != '^' {
                        modified[i] = '|';
                    } else if modified[i] == '^' && prev_line[i] == '|' {
                        if modified[i - 1] == '.' {
                            modified[i - 1] = '|';
                        }
                        if (i + 1) < modified.len() && modified[i + 1] == '.' {
                            modified[i + 1] = '|';
                        }
                        count += 1;
                    }
                }

                prev_line = modified.clone();
            }
            let line_string: String = prev_line.clone().into_iter().collect();
            println!("{:?}", line_string)
        }
    }
    return count;
}
fn part2(filename: &str) -> u64 {
    let mut paths: Vec<u64> = vec![0; 1000];
    if let Ok(lines) = read_lines(filename) {
        let mut prev_line: Vec<char> = Vec::new();
        for (index, line) in lines.enumerate() {
            if index == 0 {
                prev_line = line.expect("This should be a string").chars().collect();
            } else {
                let current_line: Vec<char> =
                    line.expect("This should be a string").chars().collect();
                let mut modified = current_line.clone();
                for (i, ch) in current_line.iter().enumerate() {
                    if prev_line[i] == 'S' {
                        modified[i] = '|';
                        paths[i] += 1;
                    } else if prev_line[i] == '|' && modified[i] != '^' {
                        modified[i] = '|';
                    } else if modified[i] == '^' {
                        paths[i - 1] += paths[i];
                        if (i + 1) < modified.len() {
                            //paths += 1;
                            paths[i + 1] += paths[i];
                        }

                        paths[i] = 0;
                    }
                }
                print!(" ");
                for (i, p) in paths.clone().iter().enumerate() {
                    if i < 15 {
                        print!("{}", p);
                    }
                }
                println!("");
                prev_line = modified.clone();
            }

            let line_string: String = prev_line.clone().into_iter().collect();
            println!("{:?}", line_string)
        }
    }
    return paths.iter().sum();
}
fn main() {
    // let answer = part1("./input.txt");
    // println!("{}", answer);
    let answer2 = part2("./input.txt");
    println!("{}", answer2);
}
