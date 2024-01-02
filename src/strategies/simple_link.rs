// This strategy can be replaced down the line with a strategy that considers both naked and hidden pairs
// And it can probably also be generalized into a strategy that considers tuples of any size, not just pairs...

use super::ReduceStrategy;
use crate::data::{Grid, Reduction, Region};

pub struct SimpleLink {}

impl ReduceStrategy for SimpleLink {
    fn reduce_candidates(grid: &Grid) -> Vec<Reduction> {
        let mut reductions: Vec<Reduction> = Vec::new();
        // for each region in the grid
        for region in grid.regions.iter() {
            // Check if there are two and only two cells that have the same two candidates (amongst others).
            // We just want to know that the region can only have a particular value in one or two spots

            for candidate in grid.possible_values() {
                let cells = region.cells_with_candidate(grid, candidate);
                if cells.len() != 2 {
                    continue;
                }
                let mut it = cells.iter();
                let a = it.next().unwrap();
                let b = it.next().unwrap();
                let a_regions: Vec<Region> = grid
                    .regions_for_coord(a)
                    .iter()
                    .filter(|r| region != *r)
                    .cloned()
                    .collect();
                let b_regions: Vec<Region> = grid
                    .regions_for_coord(b)
                    .iter()
                    .filter(|r| region != *r)
                    .cloned()
                    .collect();
                // Among the regions of cell A, find a cell C that shares a different region with B
                for c_region in a_regions.iter() {
                    if c_region == region {
                        continue;
                    }
                    for c_cell in grid.cells_for_region(c_region) {
                        if !c_cell.is_empty() {
                            continue;
                        }
                        if !c_cell.candidates.contains(&candidate) {
                            continue;
                        }
                        if region.contains(c_cell) {
                            continue;
                        }
                        if grid
                            .regions_for_coord(&c_cell.coord)
                            .iter()
                            .any(|r| b_regions.iter().any(|b_region| b_region == r))
                        {
                            reductions.push(Reduction::new(c_cell.coord, candidate));
                        }
                    }
                }
            }
        }
        reductions
    }
}
