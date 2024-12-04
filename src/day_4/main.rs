use std::env;
use std::fs;

// advent of code 2024 : day 4
// build with: cargo build
// run with  : cargo run --bin day_4 ./src/day_4/intput.txt

fn main() {
    // get the input file name from args
    let args: Vec<String> = env::args().collect();
    let input_file: &String = &args[1];

    println!("input file = {input_file}\n");

    let contents = fs::read_to_string(input_file).expect("error reading file into string");

    for line in contents.lines() {
        
        println!("{:?}", line);
    }
    println!("Result =\n");
}
