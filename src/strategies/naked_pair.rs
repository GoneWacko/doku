// This strategy can be replaced down the line with a strategy that considers both naked and hidden pairs
// And it can probably also be generalized into a strategy that considers tuples of any size, not just pairs...

use std::collections::HashSet;

use super::ReduceStrategy;
use crate::data::{Cell, Coord, Grid, Reduction};

pub struct NakedPair {}

impl ReduceStrategy for NakedPair {
    fn reduce_candidates(grid: &Grid) -> Vec<Reduction> {
        let mut reductions: Vec<Reduction> = Vec::new();
        // for each region in the grid
        for region in grid.regions.iter() {
            // Check if there are two and only two cells that have the same two candidates (and no other candidates).

            // NOTE: This can probably be implemented more efficiently. We now have a bunch of nested loops over the
            //  cells of the region.
            let cells = grid.cells_for_region(region);
            let mut visited: HashSet<Coord> = HashSet::new();
            for cell in cells.iter() {
                if !cell.is_empty() {
                    continue;
                }
                if visited.contains(&cell.coord) {
                    continue;
                }
                if cell.candidates.len() != 2 {
                    continue;
                }
                let shared_cells: Vec<&Cell> = cells
                    .iter()
                    .filter(|c| c.coord != cell.coord && c.candidates == cell.candidates)
                    .cloned()
                    .collect();
                if !shared_cells.is_empty() {
                    if shared_cells.len() > 1 {
                        panic!(
                            "At most one cell sharing the same set of candidates should be found"
                        );
                    }
                    let paired_cell = shared_cells.get(0).unwrap();
                    visited.insert(cell.coord);
                    visited.insert(paired_cell.coord);
                    for other_cell in cells
                        .iter()
                        .filter(|c| c.coord != cell.coord && c.coord != paired_cell.coord)
                        .cloned()
                    {
                        other_cell
                            .candidates
                            .intersection(&cell.candidates)
                            .for_each(|candidate| {
                                reductions.push(Reduction::new(other_cell.coord, *candidate))
                            });
                    }
                }
            }
        }
        reductions
    }
}
