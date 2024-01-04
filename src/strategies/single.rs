use super::Solution;
use super::SolveStrategy;
use crate::data::Grid;

/// Singles are cells which have only one candidate.
pub struct Single {}

// We could distinguish between naked singles and hidden singles but that's really only interesting from a presentation
// standpoint (or when generating puzzles, if we ever decide to implement that)

impl SolveStrategy for Single {
    fn solutions(grid: &Grid) -> Vec<Solution> {
        let mut found: Vec<Solution> = Vec::new();
        for cell in grid.cells.iter() {
            if cell.is_empty() && cell.candidates.len() == 1 {
                let value = cell
                    .candidates
                    .iter()
                    .next().unwrap();
                found.push(Solution::new(cell.coord, *value));
            }
        }
        found
    }
}
