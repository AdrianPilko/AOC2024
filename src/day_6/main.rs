use std::env;
use std::fmt;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
// advent of code 2024 : day 6
// build with: cargo build
// run with  : cargo run --bin day_6 ./src/day_6/intput.txt

enum Direction {
    Up = 0,
    Down = 1,
    Left = 2,
    Right = 3,
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Direction::Up => write!(f, "Up"),
            Direction::Down => write!(f, "Down"),
            Direction::Left => write!(f, "Left"),
            Direction::Right => write!(f, "Right"),
        }
    }
}

#[derive(PartialEq)]
enum Either<T, U> {
    Left(T),
    Right(U),
}

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

// find index of starting point
fn find_char_in_grid(grid: &[Vec<char>], target: char) -> Option<(usize, usize)> {
    for (row_index, row) in grid.iter().enumerate() {
        for (col_index, &ch) in row.iter().enumerate() {
            if ch == target {
                return Some((row_index, col_index)); // Return the position as (row, column)
            }
        }
    }
    None // Return None if the character is not found
}

fn can_move(grid: &Vec<Vec<char>>, current_pos: (usize, usize), direction: &Direction) -> bool {
    let mut result: bool = false;
    let width = grid.len();
    let height = grid[0].len();
    
    match direction {
        Direction::Up => {
            if current_pos.0 != 0  {
                if current_pos.0 - 1 > 0 && (grid[current_pos.0][current_pos.1] != '#') {
                    result = true;
                }
            } else {
                result = false;
            }
        }
        Direction::Down => {
            if current_pos.0 != height  {
                if current_pos.0 + 1 < height && (grid[current_pos.0+1][current_pos.1] != '#'){
                    result = true;
                }
            } else {
                result = false;
            }
        }
        Direction::Right => {
            if current_pos.1 != width   {
                if current_pos.1 + 1 < width && (grid[current_pos.0][current_pos.1+1] != '#'){
                    result = true;
                }
            } else {
                result = false;
            }
        }
        Direction::Left => {
            if current_pos.1 != 0  {
                if current_pos.1 - 1 > 0 && (grid[current_pos.0-1][current_pos.1] != '#') {
                    result = true;
                }
            } else {
                result = false;
            }
        }
    }

    // return result
    result
}

fn move_to_next(
    grid: &Vec<Vec<char>>,
    current_pos: &(usize, usize),
    direction: &mut Direction,
) -> Option<(usize, usize)> {
    let mut result: (usize, usize) = (0, 0);

    if can_move(&grid, *current_pos, &direction) {
        match direction {
            Direction::Up => {
                result = (current_pos.0 - 1, current_pos.1);
            }
            Direction::Down => {
                result = (current_pos.0 + 1, current_pos.1);
            }
            Direction::Right => {
                result = (current_pos.0, current_pos.1 + 1);
            }
            Direction::Left => {
                result = (current_pos.0, current_pos.1 - 1);
            }
        }
    } else {
        match direction {
            Direction::Up => {
                result = (current_pos.0 + 1, current_pos.1);
                *direction = Direction::Right;
            }
            Direction::Down => {
                result = (current_pos.0 - 1, current_pos.1);
                *direction = Direction::Left;
            }
            Direction::Right => {
                result = (current_pos.0, current_pos.1 + 1);
                *direction = Direction::Down;
            }
            Direction::Left => {
                result = (current_pos.0, current_pos.1 - 1);
                *direction = Direction::Up;
            }
        }
    }
    println!("horiz pos={} vert={}", current_pos.0, current_pos.1);

    Some(result)
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

    let grid = read_grid_from_file(input_file)?;
 
    print!("    ");
    for col_index in 0 .. grid[0].len() {
        print!("{:?}    ", col_index);
    }
    println!();

    let mut row_index = 0;
    for row in &grid {
        println!("{} {:?}", row_index, row);
        row_index += 1;
    }

    let target = '^';

    match find_char_in_grid(&grid, target) {
        Some((row, col)) => {
            println!("Found '{}' at position ({}, {})", target, row, col);
            // initial direction is "up", when '#' hit then direction turns to right

            let mut _direction: Direction = Direction::Up;

            let mut pos = (row, col);

            let mut total = 0;

            while Some(pos).is_some() {
                println!("Old direction={} pos={:?}", _direction, pos);

                pos = move_to_next(&grid, &pos, &mut _direction).expect("done");

                println!("New direction={} pos={:?}", _direction, pos);

                if (pos.0 > grid.len() || pos.1 > grid[0].len() || pos.0 <= 0 || pos.1 <= 0) {
                    break;
                }
                total += 1;
            }
            println!("no can_move, total={}", total);
        }

        None => println!("'{}' not found", target),
    }
    Ok(())
}
