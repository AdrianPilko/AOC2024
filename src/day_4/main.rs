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


/// Extracts all main diagonals (top-left to bottom-right)
fn get_all_main_diagonals<T: Clone>(matrix: &Vec<Vec<T>>) -> Vec<Vec<T>> {
    let mut diagonals = Vec::new();

    // Get diagonals starting from each row in the first column
    for row_start in 0..matrix.len() {
        let mut diagonal = Vec::new();
        let mut row = row_start;
        let mut col = 0;

        while row < matrix.len() && col < matrix[row].len() {
            diagonal.push(matrix[row][col].clone());
            row += 1;
            col += 1;
        }

        diagonals.push(diagonal);
    }

    // Get diagonals starting from each column in the first row (excluding the first element)
    for col_start in 1..matrix[0].len() {
        let mut diagonal = Vec::new();
        let mut row = 0;
        let mut col = col_start;

        while row < matrix.len() && col < matrix[row].len() {
            diagonal.push(matrix[row][col].clone());
            row += 1;
            col += 1;
        }

        diagonals.push(diagonal);
    }

    diagonals
}

/// Extracts all anti-diagonals (top-right to bottom-left)
fn get_all_anti_diagonals<T: Clone>(matrix: &Vec<Vec<T>>) -> Vec<Vec<T>> {
    let mut diagonals = Vec::new();

    // Get anti-diagonals starting from each row in the last column
    for row_start in 0..matrix.len() {
        let mut diagonal = Vec::new();
        let mut row = row_start;
        let mut col = matrix[row].len() - 1;

        while row < matrix.len() && col < matrix[row].len() {
            diagonal.push(matrix[row][col].clone());
            row += 1;
            if col == 0 {
                break;
            }
            col -= 1;
        }

        diagonals.push(diagonal);
    }

    // Get anti-diagonals starting from each column in the first row (excluding the last element)
    for col_start in (0..matrix[0].len() - 1).rev() {
        let mut diagonal = Vec::new();
        let mut row = 0;
        let mut col = col_start;

        while row < matrix.len() && col < matrix[row].len() {
            diagonal.push(matrix[row][col].clone());
            row += 1;
            if col == 0 {
                break;
            }
            col -= 1;
        }

        diagonals.push(diagonal);
    }

    diagonals
}


fn find_target(line: &Vec<char>, target: &Vec<char>) -> i32 {
    let mut total = 0;

    // this assumes consistent column length
    println!(" line = {:?} ", line);

    let mut search_index: usize = 0;
    let search_limit: usize = target.len();

    for c in line  {
        if *c == target[search_index] {
            print!("{} ", c);
            search_index += 1;
            if search_index == search_limit {
                println!(" FOUND WHOLE WORD");
                search_index = 0;
                total += 1;
            }
        } else {
            // check the previous search first before resetting
            // this allows for multiple on the start letter, for example XX
                if *c == 'X' && (search_index == 1 || search_index == 0){
                 // do nothing - keep same search index
                } else {
                    search_index = 0;
                }
        }
    }

    total
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
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
    //let mut answer_grid = vec![vec!['.'; grid.len()]; grid[0].len()];

    // print what we read in
    print_grid(&grid);

    // find XMAS horizontally vertically and diagonally (forwards and in reverse)

    let mut target: Vec<char> = Vec::new();

    target.push('X');
    target.push('M');
    target.push('A');
    target.push('S');
    
    let mut total = 0;

    for i in 0..grid.len() {
        total += find_target(&grid[i], &target);
    }
    println!("total after horiz forward {}", total);
    
    for i in 0..grid.len() {
        let mut line: Vec<char> = grid[i].clone();
        line.reverse();
        total += find_target(&line, &target);
    }
    println!("total after horiz back {}", total);

    for col in 0..grid.len(){
        let mut line:Vec<char> = Vec::new();
        for row in 0..grid[col].len() {
            line.push(grid[row][col]);
        } 
        total += find_target(&line, &target);
    }
    println!("total after vert down {}", total);

    for col in 0..grid[0].len(){
        let mut line:Vec<char> = Vec::new();
        for row in (0..grid[col].len()).rev() {
            line.push(grid[row][col]);
        } 
        total += find_target(&line, &target);
    }
    println!("total after vert up {}", total);
    
    let main_diagonals = get_all_main_diagonals(&grid);
    let cloned_main_diag = main_diagonals.clone();

    let anti_diagonals = get_all_anti_diagonals(&grid);
    let cloned_anti_diag = anti_diagonals.clone();

    for diag in main_diagonals{
        total += find_target(&diag, &target);
    }
    println!("total after diag main down {}", total);

    for diag in anti_diagonals{
        total += find_target(&diag, &target);
    }
    println!("total after diag anti down {}", total);
    
    for the_vecs in cloned_main_diag{
        let mut diag_reverse = Vec::new();

        for c in (0..the_vecs.len()).rev(){
            diag_reverse.push(the_vecs[c]);
        }
        total += find_target(&diag_reverse, &target);
    }
    println!("total after diag main up {}", total);

    for the_vecs in cloned_anti_diag{
        let mut diag_reverse = Vec::new();

        for c in (0..the_vecs.len()).rev(){
            diag_reverse.push(the_vecs[c]);
        }
        total += find_target(&diag_reverse, &target);
    }
    println!("total after diag anti up {}", total);

    println!("total = {}\n", total);
    Ok(())
}

