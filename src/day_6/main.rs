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
    //let width = grid[0][0].len();
    let width = grid.len();
    let height = grid[0].len();

    // compile error - need to impl partialEq on Directions enum arrrrrr! (should have used C++ I'd have done this 3 hours ago)
    match direction {
        Direction::Up => {
            println!("current_pos {:?}", current_pos);
            if (current_pos.0 != 0) {
                if current_pos.0 - 1 > 0 {
                    result = true;
                }
            } else {
                result = false;
            }
        }
        Direction::Down => {
            if current_pos.0 + 1 < height {
                result = true;
            }
        }
        Direction::Right => {
            if current_pos.1 + 1 < width {
                result = true;
            }
        }
        Direction::Left => {
            if current_pos.1 - 1 > 0 {
                {
                    result = true;
                }
            }
        }
    }

    // return result
    result
}

fn move_to_next(
    grid: &Vec<Vec<char>>,
    current_pos: &(usize, usize),
    mut direction: &Direction,
) -> Option<(usize, usize)> {
    let mut result: (usize, usize) = (0, 0);

    if can_move(&grid, *current_pos, &direction) {
        match direction {
            Direction::Up => {
                result = (current_pos.0 - 1, current_pos.1);
                println!("direction={}", direction);
                if result == (0, 0) {
                    panic!("pos = 0 0");
                    direction = &Direction::Right;
                    println!("new direction={}", direction);
                }
            }
            Direction::Down => {
                result = (current_pos.0 + 1, current_pos.1);
                println!("direction={}", direction);
                if result == (0, 0) {
                    println!("old direction={}", direction);
                    direction = &Direction::Left;
                    println!("new direction={}", direction);
                }
            }
            Direction::Right => {
                result = (current_pos.0, current_pos.1 + 1);
                println!("direction={}", direction);
                if result == (0, 0) {
                    println!("old direction={}", direction);
                    direction = &Direction::Down;
                    println!("new direction={}", direction);
                }
            }
            Direction::Left => {
                result = (current_pos.0, current_pos.1 - 1);
                println!("direction={}", direction);
                if result == (0, 0) {
                    println!("old direction={}", direction);
                    direction = &Direction::Up;
                    println!("new direction={}", direction);
                }
            }
        }
    } else {
        match direction {
            Direction::Up => {
                result = (current_pos.0, current_pos.1 + 1);
                println!("old direction={}", direction);
                direction = &Direction::Right;
                println!("new direction={}", direction);
                //  panic!("pos = 0 0");
            }
            Direction::Down => {
                result = (current_pos.0 - 1, current_pos.1);
                println!("old direction={}", direction);
                direction = &Direction::Left;
                println!("new direction={}", direction);
            }
            Direction::Right => {
                result = (current_pos.0, current_pos.1 + 1);
                println!("old direction={}", direction);
                direction = &Direction::Down;
                println!("new direction={}", direction);
            }
            Direction::Left => {
                result = (current_pos.0, current_pos.1 - 1);

                println!("old direction={}", direction);
                direction = &Direction::Up;
                println!("new direction={}", direction);
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

            let mut total = 0;

            while Some(pos).is_some() {
                pos = move_to_next(&grid, &pos, &_direction).expect("done");

                println!("New Position = {:?}", pos);

                if (pos.0 > grid.len() || pos.1 > grid[0].len())
                {
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
