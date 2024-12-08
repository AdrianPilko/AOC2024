use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


// advent of code 2024 : day 4
// build with: cargo build
// run with  : cargo run --bin day_4 ./src/day_4/intput.txt



fn read_grid_from_file(file_path: &str) -> io::Result<Vec<Vec<char>>> {
    let path = Path::new(file_path);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut grid = Vec::new();

    for line in reader.lines() {
        let line = line?; // Read the line as a Result<String>
        let row: Vec<char> = line.chars().collect(); // Convert each line into Vec<char>
        grid.push(row);
    }

    Ok(grid)
}

fn print_grid(grid: &Vec<Vec<char>>) {
    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            print!("{}", grid[row][col]);
        }
        println!();
    }
}

#[warn(dead_code)]
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
#[warn(dead_code)]
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

fn find_target(grid: &Vec<Vec<char>>,target:&str,search_direction: (i32,i32)) -> i32
{
    let mut total = 0;

    let target_str = target.to_string();

    // this assumes consistent column length
    println!("target length = {} grid[0].len() {}\n",target.len(),grid[0].len());
    // search horizontally 
    if search_direction.0 == 0
    {
        // search index increments by 1 for each grid location until found then resets
        let mut search_index:usize = 0;
        let search_limit:usize = target.len(); 
        for row in 0..grid.len() {
            for col in 0..grid[0].len()  {
                // the idea is we use the search index to go through and if we get a hit keep going
                // if not then just reset the search
                if grid[row][col] != target_str[search_index]  {
                    search_index = 0;
                    println!("found {}, search for {}, row {} col {} not found", grid[row][col], target_str[search_index],row, col);
                } 
                else
                {
                    search_index += 1;
                    if search_index == search_limit{
                       println!("FOUND row {} col {} ", row, col);
                       search_index = 0;
                       total += 1;
                    }
                }
            }
        }  
    }

   
    total
}

fn main() -> Result<(), Box<dyn std::error::Error>>  {
    // get the input file name from args
    let args: Vec<String> = env::args().collect();
    let input_file: &String = &args[1];

    match args.len() <= 1 {
        true => {
            panic!("Expected at least one argument, but none were provided.");
        }
        false => {
            println!("input file = {input_file}\n");
        }
    }

    // read in the file 
    let grid = read_grid_from_file(input_file)?;
    
    // print what we read in
    print_grid(&grid);

    // find XMAS horizontally vertically and diagonally (forwards and in reverse)
    
    // search direction is the offset in row col index to where to look for the next character
    let search_direction: (i32, i32) = (0,0);
    let target = "XMAS";

    let mut total = 0;
    total = find_target(&grid,target,search_direction);

    println!("total = {}\n", total);
    Ok(())
}
