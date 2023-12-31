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
    println!("### Initial grid:");
    let mut i: u32 = 0;
    print_board(&grid, i);
    while !grid.is_solved() {
        i += 1;
        println!();
        let solutions = NakedSingle::solutions(&grid);
        println!("### ({i}) Found solutions:");
        solutions.iter().for_each(|s| println!("{s}"));
        grid.apply(&solutions);
        print_board(&grid, i);
    }
}

fn print_board(grid: &data::Grid, i: u32) {
    println!("### ({i}) Current board state:");
    output::output_grid(&grid);
    println!("### ({i}) Candidates:");
    output::output_candidates(&grid);
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
