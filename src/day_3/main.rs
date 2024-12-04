use std::env;
use std::fs;

// advent of code 2024 : day 3
// build with: cargo build
// run with  : cargo run --bin day_3 ./src/day_3/intput.txt

fn find_and_multiply(input: &String) -> Vec<i32> {
    let mut results = Vec::new();
    let pattern = "mul(";

    let mut start = 0;
    while let Some(pos) = input[start..].find(pattern) {
        let offset = start + pos + pattern.len();
        if let Some(end) = input[offset..].find(')') {
            let slice = &input[offset..offset + end];
            if let Some((first, second)) = parse_numbers(slice) {
                results.push(first * second);
            }
        }
        start = offset;
    }

    results
}

fn sum_of_products(input: &Vec<i32>) -> i32 {
    let mut sum: i32 = 0;

    for element in input {
        sum = sum + element
    }

    sum
}

fn parse_numbers(slice: &str) -> Option<(i32, i32)> {
    let numbers: Vec<&str> = slice.split(',').collect();
    if numbers.len() == 2 {
        if let (Ok(a), Ok(b)) = (numbers[0].trim().parse(), numbers[1].trim().parse()) {
            return Some((a, b));
        }
    }
    None
}

fn main() {
    // get the input file name from args
    let args: Vec<String> = env::args().collect();
    let input_file: &String = &args[1];

    println!("input file = {input_file}\n");

    let contents = fs::read_to_string(input_file).expect("error reading file into string");

    let products = find_and_multiply(&contents);

    let result: i32 = sum_of_products(&products);

    println!("{:?}", result);
}
