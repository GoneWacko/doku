mod data;
mod output;

use data::Coord;

fn main() {
    let mut grid = dbg!(data::Grid::new(9));

    grid.set_given_value(Coord::new(1, 0), 2);
    grid.set_given_value(Coord::new(2, 0), 1);
    grid.set_given_value(Coord::new(3, 0), 6);
    grid.set_given_value(Coord::new(6, 0), 4);
    grid.set_given_value(Coord::new(7, 0), 9);
    grid.set_given_value(Coord::new(0, 1), 3);
    grid.set_given_value(Coord::new(1, 1), 8);
    grid.set_given_value(Coord::new(3, 1), 1);
    grid.set_given_value(Coord::new(4, 1), 9);
    grid.set_given_value(Coord::new(5, 1), 4);
    grid.set_given_value(Coord::new(0, 2), 5);
    grid.set_given_value(Coord::new(4, 2), 7);
    grid.set_given_value(Coord::new(1, 3), 4);
    grid.set_given_value(Coord::new(2, 3), 5);
    grid.set_given_value(Coord::new(3, 3), 7);
    grid.set_given_value(Coord::new(5, 3), 2);
    grid.set_given_value(Coord::new(6, 3), 1);
    grid.set_given_value(Coord::new(0, 4), 9);
    grid.set_given_value(Coord::new(1, 4), 6);
    grid.set_given_value(Coord::new(4, 4), 5);
    grid.set_given_value(Coord::new(7, 4), 7);
    grid.set_given_value(Coord::new(8, 4), 4);
    grid.set_given_value(Coord::new(2, 5), 2);
    grid.set_given_value(Coord::new(3, 5), 3);
    grid.set_given_value(Coord::new(5, 5), 9);
    grid.set_given_value(Coord::new(6, 5), 8);
    grid.set_given_value(Coord::new(7, 5), 5);
    grid.set_given_value(Coord::new(2, 6), 9);
    grid.set_given_value(Coord::new(4, 6), 2);
    grid.set_given_value(Coord::new(8, 6), 8);
    grid.set_given_value(Coord::new(3, 7), 9);
    grid.set_given_value(Coord::new(4, 7), 3);
    grid.set_given_value(Coord::new(5, 7), 6);
    grid.set_given_value(Coord::new(7, 7), 4);
    grid.set_given_value(Coord::new(8, 7), 5);
    grid.set_given_value(Coord::new(1, 8), 3);
    grid.set_given_value(Coord::new(2, 8), 7);
    grid.set_given_value(Coord::new(5, 8), 8);
    grid.set_given_value(Coord::new(6, 8), 9);
    grid.set_given_value(Coord::new(7, 8), 6);

    grid.compute_candidates();

    output::output_grid(&grid);
    output::output_candidates(&grid)
}

// Set up data structure: x * x cells, n regions
// for each cell, track which regions they belong to, and what their candidates are, and their value, and whether they're given or not
// implement calculating all candidates for the grid: for each cell, take the complement of the set of all given values in its regions
// implement strategies for solving: singles, intersections, naked pairs...
// implement brute force as a fallback
// implement ways to display the state, (including step-by-step solving?)
