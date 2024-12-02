use std::env;
use std::fs;

// advent of code 2024 : day 2
// build with: cargo build
// run with  : cargo run -- ./src/day_2/intput.txt

fn string_to_vec(input: &str) -> Result<Vec<i32>, Box<dyn std::error::Error>> {
    input
        .split_whitespace() // Split by whitespace
        .map(|s| s.parse::<i32>()) // Parse each substring as i32
        .collect::<Result<Vec<i32>, _>>() // Collect into a Vec<i32>, handling errors
        .map_err(|e| e.into()) // Convert the error type if any parsing fails
}


fn main() {
    // get the input file name from args
    let args: Vec<String> = env::args().collect();
    let input_file: &String = &args[1];

    println!("input file = {input_file}\n");

    let contents = fs::read_to_string(input_file).expect("error reading file into string");

    let mut sum_of_valid_line: i32 = 0;
    let mut sum_of_valid_line_part_2: i32 = 0;
    

    for line in contents.lines() {
        let Ok(number) = string_to_vec(line) else { continue };
        let line_length: usize = number.len();
        let mut line_safe: bool = true;
        let mut line_would_be_safe: bool = true;
        let mut increasing: bool = false;
        let mut _decreasing: bool = false;

        let mut count_of_faults = 0;


        for i in 0..line_length - 1 {
            let num1: i32 = number[i];
            let num2: i32 = number[i + 1];

            if (num1 < num2) && (_decreasing == true) {
                line_safe = false;
                count_of_faults += 1;
            }
            else if (num1 > num2) && (increasing == true) {
                line_safe = false;
                count_of_faults += 1;
            }
            else if num1 == num2 {
                line_safe = false;
                count_of_faults += 1;
            }
            else if (num1 - num2).abs() > 3 {
                line_safe = false;
                count_of_faults += 1;
            }
            else if increasing == true && _decreasing == true {
                line_safe = false;
                count_of_faults += 1;
            }

            increasing = num1 < num2;
            _decreasing = num1 > num2;

            if count_of_faults >= 2
            {
                line_would_be_safe = false;
            }
        }

        println!("line= {} and safe=={} would be safe=={} fault_count=={}", line, line_safe, line_would_be_safe, count_of_faults);

        if line_safe == true {
            sum_of_valid_line += 1;
            sum_of_valid_line_part_2 += 1;
        }
        else if line_would_be_safe == true
        {
            sum_of_valid_line_part_2 += 1;
        }
    }
    println!("Valid lines = {}, would be valid {}\n", sum_of_valid_line ,sum_of_valid_line_part_2);
}
