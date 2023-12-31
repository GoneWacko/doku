use crate::data::Grid;
use std::io::stdout;

pub fn output_grid(grid: &Grid) {
    let _lock = stdout().lock();
    for y in 0..grid.size {
        for x in 0..grid.size {
            let cell = &grid.cells[(y * grid.size + x) as usize];
            match cell.value {
                Some(v) => {
                    print!("{v} ")
                }
                None => {
                    print!(". ")
                }
            }
        }
        println!()
    }
}

pub fn output_candidates(grid: &Grid) {
    let _lock = stdout().lock();
    for cell in grid.cells.iter() {
        if cell.is_empty() {
            let mut candidate_vec: Vec<u8> = cell.candidates.iter().cloned().collect();
            candidate_vec.sort();
            println!("{}: {:?}", cell.coord, candidate_vec);
        }
    }
}
