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

fn print_array(a: Array2D<char>) {
    for rows in a.rows_iter() {
        for element in rows {
            print!("{element}");
        }
        println!();
    }
}

fn check_bounds(a: &Array2D<char>, row: i32, col: i32) -> bool {
    if (row < (a.num_rows() as i32) && row >= 0) && (col < (a.num_columns() as i32) && col >= 0) {
        return true;
    }
    false
}

fn check_surrounding(a: &Array2D<char>, row: i32, col: i32) -> bool {
    let mut count = 0;
    if check_bounds(a, row - 1, col - 1) && a[((row - 1) as usize, (col - 1) as usize)] == '@' {
        count += 1;
    }
    if check_bounds(a, row + 1, col - 1) && a[((row + 1) as usize, (col - 1) as usize)] == '@' {
        count += 1;
    }
    if check_bounds(a, row + 1, col + 1) && a[((row + 1) as usize, (col + 1) as usize)] == '@' {
        count += 1;
    }
    if check_bounds(a, row - 1, col + 1) && a[((row - 1) as usize, (col + 1) as usize)] == '@' {
        count += 1;
    }

    if check_bounds(a, row, col + 1) && a[((row) as usize, (col + 1) as usize)] == '@' {
        count += 1;
    }
    if check_bounds(a, row, col - 1) && a[((row) as usize, (col - 1) as usize)] == '@' {
        count += 1;
    }

    if check_bounds(a, row - 1, col) && a[((row - 1) as usize, (col) as usize)] == '@' {
        count += 1;
    }
    if check_bounds(a, row + 1, col) && a[((row + 1) as usize, (col) as usize)] == '@' {
        count += 1;
    }

    if count < 4 {
        return true;
    }
    false
}

fn main() -> Result<(), Error> {
    let mut first_array: Vec<Vec<char>> = Vec::new();
    if let Ok(lines) = read_lines("./input.txt") {
        for mut line in lines.map_while(Result::ok) {
            let mut item: Vec<char> = line.chars().collect();
            first_array.push(item);
        }
    }
    let mut array = Array2D::from_rows(&first_array)?;

    let mut count = 0;
    let mut removed = 0;
    let mut done = false;
    while !done {
        let original_array = array.clone();
        for i in 0..array.num_rows() {
            for j in 0..array.num_columns() {
                let check = *array.get(i, j).unwrap();

                if check == '@' {
                    //check surrounding 8
                    let result = check_surrounding(&original_array, i as i32, j as i32);
                    if result {
                        count += 1;
                        array.set(i, j, 'X')?;
                    }
                } else if check == 'X' {
                    array.set(i, j, '.')?;
                }
            }
        }
        println!("{count}");
        removed += count;
        if count == 0 {
            done = true;
        }
        count = 0;
    }
    println!("{removed}");
    Ok(())
}
