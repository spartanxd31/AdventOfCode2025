use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use array2d::{Array2D, Error};

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() -> Result<(), Error> {
    let mut first_array: Vec<Vec<String>> = Vec::new();
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines.map_while(Result::ok) {
            let item = line.split_whitespace().map(|s| s.to_string()).collect();
            first_array.push(item);
        }
    }

    // let something = first_array[0][0].clone();
    // println!("{something}");
    let mut answers: u64 = 0;
    let mut op: String = "".to_string();
    let row_len = first_array[0].len();
    println!("Row length = {row_len}");
    //Need to get the last row and column numbers
    for j in (0..(row_len)).rev() {
        let mut op_result: u64 = 0;
        for i in (0..(first_array.len())).rev() {
            println!("{i}, {j}");
            let element = first_array[i][j].clone();
            if element == "*" || element == "+" {
                op = element;
            } else {
                let num = element.parse::<u64>().expect("this should be a number");
                if op == "+" {
                    op_result += num;
                } else if op == "*" {
                    if op_result != 0 {
                        op_result *= num;
                    } else {
                        op_result = num;
                    }
                }
            }
        }
        // println!("{op_result}");
        answers += op_result;
    }
    println!("{answers}");

    Ok(())
}
