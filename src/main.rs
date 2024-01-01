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

fn main() {
    let mut file_path_arg = env::args().skip(1).take(1);
    let file_path: String = file_path_arg
        .next()
        .unwrap_or(String::from("puzzles/intersection.txt"));

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
}

fn print_board(grid: &Grid, i: u32) {
    println!("### ({i}) Current board state:");
    output::output_grid(&grid);
    println!("### ({i}) Candidates:");
    output::output_candidates(&grid);
}

fn load_puzzle(file_path: &str) -> Grid {
    let mut grid = Grid::new(9);
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    for (y, line) in contents.lines().enumerate() {
        for (x, c) in line.split(' ').enumerate() {
            if c != "." {
                let v: u8 = c.parse().expect("The value should have been a number");
                grid.set_given_value(Coord::new(x as u8, y as u8), v)
            }
        }
    }
    grid
}
