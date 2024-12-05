use std::env;
use std::fs;

// advent of code 2024 : day 5
// build with: cargo build
// run with  : cargo run --bin day_5 ./src/day_5/intput.txt

fn get_rule(slice: &str) -> Option<(i32, i32)> {
    let numbers: Vec<&str> = slice.split('|').collect();
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

    for line in contents.lines() {
        let rule = get_rule(&line);
        
        let mut rule_vec:Vec<(i32,i32)>;

        if rule.is_some() == true { 
            //// DOESNT COMPILE RAN OUT OF TIME
            rule_vec.push((&rule.as_slice()).to_vec());
            println!("{:?}", rule_vec);
        }   
    }
}
