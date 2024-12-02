use std::env;
use std::fs;

// advent of code 2024 : day 2
// build with: cargo build
// run with  : cargo run -- ./src/day_2/intput.txt

fn main() {
    // get the input file name from args
    let args: Vec<String> = env::args().collect();
    let input_file: &String = &args[1];

    println!("input file = {input_file}\n");

    let contents = fs::read_to_string(input_file).expect("error reading file into string");

    let mut sum_of_valid_line: i32 = 0;

    for line in contents.lines() {
        let numbers: Vec<&str> = line.split_whitespace().collect();
        let line_length: usize = numbers.len();
        let mut line_safe: bool = true;
        let mut increasing: bool = false;
        let mut _decreasing: bool = false;

        for i in 0..line_length - 1 {
            let num1: i32 = numbers[i].parse::<i32>().unwrap();
            let num2: i32 = numbers[i + 1].parse::<i32>().unwrap();

            if (num1 < num2) && (_decreasing == true) {
                line_safe = false;
            }
            if (num1 > num2) && (increasing == true) {
                line_safe = false;
            }
            if num1 == num2 {
                line_safe = false;
            }
            if (num1 - num2).abs() > 3 {
                line_safe = false;
            }
            if increasing == true && _decreasing == true {
                line_safe = false;
            }

            increasing = num1 < num2;
            _decreasing = num1 > num2;
        }

        println!("line = {} and safe =={}", line, line_safe);

        if line_safe == true {
            sum_of_valid_line += 1;
        }
    }

    println!("Valid lines = {}\n", sum_of_valid_line);
}
