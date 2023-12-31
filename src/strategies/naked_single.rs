use super::Solution;
use super::SolveStrategy;
use crate::data::Grid;

pub struct NakedSingle {}

impl SolveStrategy for NakedSingle {
    fn solutions(grid: &Grid) -> Vec<Solution> {
        let mut found: Vec<Solution> = Vec::new();
        for cell in grid.cells.iter() {
            if cell.value.is_none() && cell.candidates.len() == 1 {
                let value = cell
                    .candidates
                    .iter()
                    .next()
                    .expect("The candidates set should contain at least one value");
                found.push(Solution::new(cell.coord, *value));
            }
        }
        found
    }
}
