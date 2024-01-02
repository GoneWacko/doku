mod data;
mod output;
mod strategies;

use data::{Coord, Grid};
use strategies::naked_pair::NakedPair;
use strategies::region_intersection::RegionIntersection;
use strategies::single::Single;
use strategies::ReduceStrategy;
use strategies::SolveStrategy;

use std::env;
use std::fs;

use regex::Regex;

fn main() {
    let mut file_path_arg = env::args().skip(1).take(1);
    let file_path: String = file_path_arg
        .next()
        .unwrap_or(String::from("puzzles/simple_link.txt"));

    let mut grid = load_puzzle(file_path.as_str());
    grid.compute_candidates();

    let mut i: u32 = 0;
    while !grid.is_solved() {
        print_board(&grid, i);
        i += 1;
        println!();
        println!("Trying singles");
        if Single::try_solution(&mut grid) {
            continue;
        }
        println!("No singles found.");
        println!("Trying intersections");
        if RegionIntersection::try_reduction(&mut grid) {
            continue;
        }
        println!("No intersections found.");
        println!("Trying naked pair reduction");
        if NakedPair::try_reduction(&mut grid) {
            continue;
        }
        println!("No naked pairs found.");
        print_board(&grid, i);
        panic!("No implemented strategies can further solve this board!");
    }
    println!("Solved:");
    print_board(&grid, i);
    match grid.validate() {
        Ok(()) => {
            println!("The solution is valid!");
            std::process::exit(0);
        }
        Err((msg, coord)) => {
            println!("There was an error with the cell at {}: {}", coord, msg);
            std::process::exit(1)
        }
    }
}

fn print_board(grid: &Grid, i: u32) {
    println!("### ({i}) Current board state:");
    output::output_grid(&grid);
    println!("### ({i}) Candidates:");
    output::output_candidates(&grid);
}

fn load_puzzle(file_path: &str) -> Grid {
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let mut lines = contents.lines();
    let first_line = lines
        .next()
        .expect("There should be multiple lines of text");
    let size: u8 = Regex::new(r"^size (\d+)$")
        .unwrap()
        .captures(first_line)
        .expect("The first line should be 'size n' where n is the width of the puzzle.")
        .get(1)
        .unwrap()
        .as_str()
        .parse()
        .expect("The size must be a number");

    let mut grid = Grid::new(size);
    for y in 0..size {
        let grid_line = lines
            .next()
            .expect("There must be enough lines to build the full grid.");
        let mut highest_x: u8 = 0;
        for (x, c) in grid_line.split(' ').enumerate() {
            highest_x = x as u8;
            if c != "." {
                let v: u8 = c.parse().expect("The value should have been a number");
                grid.set_given_value(Coord::new(x as u8, y as u8), v);
            }
        }
        assert_eq!(highest_x, size - 1);
    }

    let extra_regex = Regex::new(r"^extra square (\d+),(\d+)$").unwrap();
    for extra_line in lines {
        if let Some(m) = extra_regex.captures(extra_line) {
            let square_x: u8 = m
                .get(1)
                .unwrap()
                .as_str()
                .parse()
                .expect("The extra square X coordinate must be a number");
            let square_y: u8 = m
                .get(2)
                .unwrap()
                .as_str()
                .parse()
                .expect("The extra square Y coordinate must be a number");
            grid.add_extra_square(square_x, square_y)
        }
    }

    grid
}
