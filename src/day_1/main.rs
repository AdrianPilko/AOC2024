use std::env;
use std::fs;
use std::collections::HashMap;

// advent of code 2024 : day 1
// build with: cargo build
// run with  : cargo run -- ./src/day_1/intput.txt
 
fn parse_two_columns(input: &str) -> Result<(Vec<i32>, Vec<i32>), Box<dyn std::error::Error>> {
    let mut column1 = Vec::new();
    let mut column2 = Vec::new();

    for line in input.lines() {
        let numbers: Vec<&str> = line.split_whitespace().collect();
        if numbers.len() != 2 {
            return Err(format!("Invalid line format: '{}'", line).into());
        }

        let num1: i32 = numbers[0].parse()?;
        let num2: i32 = numbers[1].parse()?;

        column1.push(num1);
        column2.push(num2);
    }
    column1.sort();
    column2.sort();
    Ok((column1, column2))
}

fn main() {
    // get the input file name from args
    let args: Vec<String> = env::args().collect();
    let input_file: &String = &args[1];

    println!("input file = {input_file}");

    let contents = fs::read_to_string(input_file).expect("Should have been able to read the file");

    match parse_two_columns(&contents) {
        Ok((col1, col2)) => {
            let col_1_size = col1.len();
            //println!("Column 1: {:?}\n", col1);
            //println!("Column 2: {:?}\n", col2);

            let mut _sum = 0;
            // now we have sorted col1 and col2 just find difference between pairs
            for i in 0..col_1_size {
                let _diff = col1[i] - col2[i];
                _sum = _sum + _diff.abs();
            }

            println!("Day 1 part 1 answer, Sum is {}", _sum);

                // Count matches and compute the product of counts
            let mut match_counts = HashMap::new();
            let mut total_product: usize = 0;
            // part 2 has to find sum of the matched counts of numbers in second column

            // Count matches
            for &num in &col1 {
                let count = col2.iter().filter(|&&x| x == num).count();
                match_counts.insert(num, count);
        
                // Avoid multiplying by 0; if count is 0, total_product becomes 0
                if count > 0 {
                    total_product += count * (num as usize);
                } 
            }
            println!("Day 1 part 2 answer, Sum is {}", total_product);
        }
        Err(e) => {
            eprintln!("Error parsing input: {}", e);
        }
    }
}
