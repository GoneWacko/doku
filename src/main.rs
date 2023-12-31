mod data;
mod output;
mod strategies;

use data::Coord;
use strategies::naked_single::NakedSingle;
use strategies::SolveStrategy;

use std::fs;

fn main() {
    let mut grid = load_puzzle("puzzles/very_easy_2.txt");
    grid.compute_candidates();

    output::output_grid(&grid);
    output::output_candidates(&grid);

    let solutions = NakedSingle::solutions(&grid);
    solutions.iter().for_each(|s| println!("{s}"));
}

fn load_puzzle(file_path: &str) -> data::Grid {
    let mut grid = data::Grid::new(9);
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

// Set up data structure: x * x cells, n regions
// for each cell, track which regions they belong to, and what their candidates are, and their value, and whether they're given or not
// implement calculating all candidates for the grid: for each cell, take the complement of the set of all given values in its regions
// implement strategies for solving: singles, intersections, naked pairs...
// implement brute force as a fallback
// implement ways to display the state, (including step-by-step solving?)
