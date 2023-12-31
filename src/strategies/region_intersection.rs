use std::ptr;

use super::ReduceStrategy;
use crate::data::{Grid, Reduction, Region};

pub struct RegionIntersection {}

impl ReduceStrategy for RegionIntersection {
    fn reduce_candidates(grid: &Grid) -> Vec<Reduction> {
        let mut reductions: Vec<Reduction> = Vec::new();
        // for each region in the grid
        for region in grid.regions.iter() {
            // for each possible value
            for candidate in grid.possible_values() {
                // collect cells in the region that have the possible value as a candidate
                let cells_in_region_with_candidate = region.cells_with_candidate(grid, candidate);
                if cells_in_region_with_candidate.is_empty() {
                    continue;
                }
                // check intersection with other regions that hold all those cells
                //   We filter out the region we're currently checking. But this pointer comparison is a bit meh. Either we should id the regions some other way, or maybe we can just not do any of this..?
                //   We would just erase the candidate from any of the other cells in the region, but we already know they're not there anyway, because we gathered all the cells_with_candidate just now.
                let intersecting_regions: Vec<&Region> = grid
                    .regions_for_coords(&cells_in_region_with_candidate)
                    .iter()
                    .filter(|&r| !ptr::eq(*r, region as *const Region))
                    .map(|r| *r)
                    .collect();
                // if such a region exists: Remove the candidate value from the cells in that region that do not intersect with the current region and that do have it as a candidate.
                for intersecting_region in intersecting_regions.iter() {
                    for cell in grid.cells_for_region(&intersecting_region) {
                        if cell.is_empty()
                            && !cells_in_region_with_candidate.contains(&cell.coord)
                            && cell.candidates.contains(&candidate)
                        {
                            reductions.push(Reduction::new(cell.coord, candidate));
                        }
                    }
                }
            }
        }
        reductions
    }
}
