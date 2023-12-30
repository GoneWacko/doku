mod data;
mod output;

fn main() {

    let grid = dbg!(data::Grid::new(9));

    let mut num = 0;
    for region in grid.regions.iter() {
        num += 1;
        println!("{num}: {:?}", region.cell_coords(&grid));
    }

    println!();

    output::output_grid(&grid)
}


// Set up data structure: x * x cells, n regions
// for each cell, track which regions they belong to, and what their candidates are, and their value, and whether they're given or not
// implement calculating all candidates for the grid: for each cell, take the complement of the set of all given values in its regions
// implement strategies for solving: singles, intersections, naked pairs...
// implement brute force as a fallback
// implement ways to display the state, (including step-by-step solving?)