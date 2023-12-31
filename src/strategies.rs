pub mod naked_single;

use crate::data::{Grid, Solution};


pub trait SolveStrategy {
    fn solutions(grid: &Grid) -> Vec<Solution>;
}

pub trait ReduceStrategy {}