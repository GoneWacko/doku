use core::fmt::Debug;
use std::vec::Vec;

#[derive(Debug, Copy, Clone)]
pub struct Coord {
    x: u8,
    y: u8,
}

#[derive(Debug, Clone)]
pub struct Cell {
    coord: Coord,
    candidates: Vec<u8>,
    pub value: Option<u8>,
    is_given: bool,
}

// trait Region {
//     fn cell_coords(self, grid: &Grid) -> Vec<Coord>;
// }

#[derive(Debug)]
struct Row {
    x: u8,
}

#[derive(Debug)]
struct Column {
    y: u8,
}

#[derive(Debug)]
struct Subgrid {
    size: u8,
    top_left: Coord,
}

#[derive(Debug)]
pub enum Region {
    Row(Row),
    Column(Column),
    Subgrid(Subgrid),
}

impl Region {
    pub fn cell_coords(self: &Self, grid: &Grid) -> Vec<Coord> {
        let mut coords: Vec<Coord> = Vec::new();
        match self {
            Region::Row(row) => {
                coords.reserve_exact(grid.size as usize);
                for y in 0..grid.size {
                    coords.push(Coord { x: row.x, y });
                }
                coords
            }
            Region::Column(column) => {
                coords.reserve_exact(grid.size as usize);
                for x in 0..grid.size {
                    coords.push(Coord { x, y: column.y });
                }
                coords
            }
            Region::Subgrid(subgrid) => {
                coords.reserve_exact((subgrid.size * subgrid.size) as usize);
                for y in 0..subgrid.size {
                    for x in 0..subgrid.size {
                        coords.push(Coord {
                            x: subgrid.top_left.x + x,
                            y: subgrid.top_left.y + y,
                        })
                    }
                }
                coords
            }
        }
    }
}

#[derive(Debug)]
pub struct Grid {
    pub size: u8,
    pub cells: Vec<Cell>,
    pub regions: Vec<Region>,
}

impl Grid {
    pub fn new(size: u8) -> Self {
        let mut grid = Grid {
            size,
            cells: Vec::with_capacity((size * size) as usize),
            regions: Vec::new(),
        };

        for y in 0..size {
            for x in 0..size {
                grid.cells.push(Cell{
                    coord: Coord{x,y},
                    candidates: Vec::new(),
                    value: Some(x+1),
                    is_given: false 
                })
            }
            
            // Since we're iterating over the size anyway we can set up our row & column regions here:
            grid.regions.push(Region::Row(Row { x: y }));
            grid.regions.push(Region::Column(Column { y }));
        }

        // TODO Make this more generic; It should be something like:
        //  "if the square root is a round number, let that be the subgrid size"
        if size == 9 {
            let subgrid_size = 3;
            for y in 0..subgrid_size {
                for x in 0..subgrid_size {
                    grid.regions.push(Region::Subgrid(Subgrid {
                        size: subgrid_size,
                        top_left: Coord {
                            x: x * subgrid_size,
                            y: y * subgrid_size,
                        },
                    }));
                }
            }
        }

        grid
    }
}
