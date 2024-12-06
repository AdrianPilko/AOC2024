use std::env;
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

fn can_move(grid: &Vec<Vec<char>>, current_pos: (usize, usize), direction: Direction) -> bool {
    let _ = direction;
    let _ = current_pos;
    let mut result: bool = true;
    //let width = grid[0][0].len();
    let width = 0;
    let height = grid[0].len();

    println!("width={} height={}", width, height);

    // compiler error - need to impl partialEq on Directions enum arrrrrr! (should have used C++ I'd have done this 3 hours ago)
    if Direction::Up == direction {
        result = current_pos.0 - 1 > 0;
    } else if Direction::Down == direction {
        result = current_pos.0 + 1 < height;
    } else if Direction::Left == direction {
        result = current_pos.0 - 1 > 0;
    } else if Direction::Right == direction {
        result = current_pos.1 + 1 < width;
    }
    // return result
    result
}

fn move_to_next(
    grid: &Vec<Vec<char>>,
    current_pos: (usize, usize),
    direction: Direction,
) -> Option<(usize, usize)> {
    let mut result: (usize, usize) = (0, 0);

    if can_move(&grid, current_pos, direction) {
        match direction {
            Direction::Up => result = (current_pos.0 - 1, current_pos.1),
            Direction::Down => result = (current_pos.0 + 1, current_pos.1),
            Direction::Right => result = (current_pos.0, current_pos.1 + 1),
            Direction::Left => result = (current_pos.0, current_pos.1 - 1),
        }
    } else {
        println!("error from can_move");
    }
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

    for row in &grid {
        println!("{:?}", row);
    }

    let target = '^';

    match find_char_in_grid(&grid, target) {
        Some((row, col)) => {
            println!("Found '{}' at position ({}, {})", target, row, col);
            // initial direction is "up", when '#' hit then direction turns to right

            let mut _direction: Direction = Direction::Up;

            let mut pos = (row, col);

            let width_of_lab_space = grid[0].len(); // assuming every row is the same!

            let pos = move_to_next(&grid, pos, _direction);

            println!("New Position = {:?}", pos);
        }

        None => println!("'{}' not found", target),
    }
    Ok(())
}
