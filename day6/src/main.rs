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
fn reverse(input: &str) -> String {
    input.chars().rev().collect()
}
fn main() -> Result<(), Error> {
    let mut first_array: Vec<Vec<String>> = Vec::new();
    if let Ok(lines) = read_lines("./test.txt") {
        for line in lines.map_while(Result::ok) {
            let item = line.split_whitespace().map(|s| s.to_string()).collect();
            first_array.push(item);
        }
    }

    // let something = first_array[0][0].clone();
    // println!("{something}");
    // let mut answers: u64 = 0;
    // let mut op: String = "".to_string();
    // let row_len = first_array[0].len();
    // println!("Row length = {row_len}");
    // //Need to get the last row and column numbers
    // for j in (0..(row_len)).rev() {
    //     let mut op_result: u64 = 0;
    //     for i in (0..(first_array.len())).rev() {
    //         println!("{i}, {j}");
    //         let element = first_array[i][j].clone();
    //         if element == "*" || element == "+" {
    //             op = element;
    //         } else {
    //             let num = element.parse::<u64>().expect("this should be a number");
    //             if op == "+" {
    //                 op_result += num;
    //             } else if op == "*" {
    //                 if op_result != 0 {
    //                     op_result *= num;
    //                 } else {
    //                     op_result = num;
    //                 }
    //             }
    //         }
    //     }
    //     // println!("{op_result}");
    //     answers += op_result;
    // }
    // println!("{answers}");
    //
    let mut answers: u64 = 0;
    let mut op: String = "".to_string();
    let row_len = first_array[0].len();
    //Need to get the last row and column numbers
    for j in (0..(row_len)).rev() {
        let mut op_result: u64 = 0;
        let mut selected: Vec<String> = Vec::new();
        for i in (0..(first_array.len())).rev() {
            let element = first_array[i][j].clone();
            if element == "*" || element == "+" {
                op = element;
            } else {
                selected.push(element);
            }
        }

        let mut selected_numbers: Vec<Vec<u32>> = Vec::new();
        //Make it go right to left
        for mut n in selected {
            while n.len() != 4 {
                n.push('0');
            }
            n = reverse(n.as_str());
            let digits: Vec<u32> = n.chars().filter_map(|c| c.to_digit(10)).collect();
            println!("{n}");
            selected_numbers.push(digits);
        }

        for something in &selected_numbers {
            println!("{:?}", something);
        }

        // Convert columns to numbers (MSB at bottom)
        let num_cols = selected_numbers[0].len();
        let mut column_numbers = Vec::new();

        for col in 0..num_cols {
            let mut number = 0u64;
            let mut multiplier = 1u64;

            // Read from top to bottom, building number with MSB at bottom
            for row in 0..selected_numbers.len() {
                number += selected_numbers[row][col] as u64 * multiplier;
                multiplier *= 10;
            }

            column_numbers.push(number);
        }

        println!("Column numbers: {:?}", column_numbers);

        for col in column_numbers {
            if op == "+" {
                op_result += col;
                println!("{op_result}, after adding {col}");
            } else if op == "*" {
                if op_result != 0 {
                    op_result *= col;
                    println!("{op_result}, after multiplying by {col}");
                } else {
                    op_result = col;
                    println!("{op_result}, first value (no multiplication yet)");
                }
            }
        }

        //Do the math here
        // for i in 0..selected.len() {}
        // println!("{op_result}");
        answers += op_result;
    }
    println!("{answers}");

    Ok(())
}
