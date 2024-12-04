use std::env;
use std::fs;

// advent of code 2024 : day 4
// build with: cargo build
// run with  : cargo run --bin day_4 ./src/day_4/intput.txt
fn find_in_a_row_forwards(input: &str) -> i32 {
    let mut res: i32 = 0;

    let chars_vec: Vec<char> = input.chars().collect();

    let chars_in_line = chars_vec.len();
    for i in 0..chars_in_line - 3 {
        if chars_vec[i] == 'X' {
            if chars_vec[i + 1] == 'M' {
                if chars_vec[i + 2] == 'A' {
                    if chars_vec[i + 3] == 'S' {
                        res = res + 1;
                    }
                }
            }
        }
    }
    res
}

fn find_in_a_row_backwards(input: &str) -> i32 {
    let mut res: i32 = 0;

    let chars_vec: Vec<char> = input.chars().collect();

    let chars_in_line = chars_vec.len();
    for i in (3..chars_in_line - 1).rev() {
        if chars_vec[i] == 'X' {
            if chars_vec[i + 1] == 'M' {
                if chars_vec[i + 2] == 'A' {
                    if chars_vec[i + 3] == 'S' {
                        res = res + 1;
                    }
                }
            }
        }
    }
    res
}

fn main() {
    // get the input file name from args
    let args: Vec<String> = env::args().collect();
    let input_file: &String = &args[1];

    println!("input file = {input_file}\n");
    let mut result: i32 = 0;
    let contents = fs::read_to_string(input_file).expect("error reading file into string");

    for line in contents.lines() {
        result += find_in_a_row_forwards(line);
        result += find_in_a_row_backwards(line);
        println!("{:?}", line);
    }
    println!("Result ={}\n", result);
}
