pub mod naked_pair;
pub mod region_intersection;
pub mod single;

use crate::data::{Grid, Reduction, Solution};

pub trait SolveStrategy {
    fn solutions(grid: &Grid) -> Vec<Solution>;

    fn try_solution(grid: &mut Grid) -> bool {
        let solutions = Self::solutions(&grid);
        if !solutions.is_empty() {
            println!("### Found solutions:");
            solutions.iter().for_each(|s| println!("{s}"));
            grid.apply_solutions(&solutions);
        }
        !solutions.is_empty()
    }
}

pub trait ReduceStrategy {
    fn reduce_candidates(grid: &Grid) -> Vec<Reduction>;

    fn try_reduction(grid: &mut Grid) -> bool {
        let reductions = Self::reduce_candidates(&grid);
        if !reductions.is_empty() {
            println!("### Found reductions:");
            reductions.iter().for_each(|r| println!("{r}"));
            grid.apply_reductions(&reductions);
        }
        !reductions.is_empty()
    }
}
