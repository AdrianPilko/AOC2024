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

    let contents = fs::read_to_string(input_file).expect("Should have been able to read the file");

    println!("file contents = {}\n", contents);
}
