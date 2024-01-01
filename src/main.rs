mod data;
mod output;
mod strategies;

use data::Coord;
use strategies::naked_pair::NakedPair;
use strategies::region_intersection::RegionIntersection;
use strategies::single::Single;
use strategies::ReduceStrategy;
use strategies::SolveStrategy;

use std::fs;

fn main() {
    let mut grid = load_puzzle("puzzles/naked_pair.txt");
    grid.compute_candidates();

    println!("### Initial grid:");
    let mut i: u32 = 0;
    print_board(&grid, i);
    while !grid.is_solved() {
        i += 1;
        println!();
        println!("Trying singles");
        let solutions = Single::solutions(&grid);
        if !solutions.is_empty() {
            println!("### ({i}) Found solutions:");
            solutions.iter().for_each(|s| println!("{s}"));
            grid.apply_solutions(&solutions);
            print_board(&grid, i);
            continue;
        }
        println!("No singles found.");
        println!("Trying intersection reduction");
        let reductions = RegionIntersection::reduce_candidates(&grid);
        if !reductions.is_empty() {
            println!("### ({i}) Found reductions:");
            reductions.iter().for_each(|r| println!("{r}"));
            grid.apply_reductions(&reductions);
            print_board(&grid, i);
            continue;
        }
        println!("No intersections found.");
        println!("Trying naked pair reduction");
        let reductions = NakedPair::reduce_candidates(&grid);
        if !reductions.is_empty() {
            println!("### ({i}) Found naked pair reductions:");
            reductions.iter().for_each(|r| println!("{r}"));
            grid.apply_reductions(&reductions);
            print_board(&grid, i);
            continue;
        }
        println!("No naked pairs found.");
        print_board(&grid, i);
        panic!("No implemented strategies can further solve this board!");
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
