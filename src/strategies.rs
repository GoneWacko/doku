pub mod region_intersection;
pub mod single;

use crate::data::{Grid, Reduction, Solution};

pub trait SolveStrategy {
    fn solutions(grid: &Grid) -> Vec<Solution>;
}

pub trait ReduceStrategy {
    fn reduce_candidates(grid: &Grid) -> Vec<Reduction>;
}
