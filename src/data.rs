use core::fmt::Debug;
use std::collections::{HashMap, HashSet};
use std::vec::Vec;

#[derive(Debug, Copy, Clone, PartialEq, Hash, Eq)]
pub struct Coord {
    x: u8,
    y: u8,
}

impl Coord {
    pub fn new(x: u8, y: u8) -> Coord {
        Coord { x, y }
    }
}

impl std::fmt::Display for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

#[derive(Debug)]
pub struct Cell {
    pub coord: Coord,
    pub candidates: HashSet<u8>,
    pub value: Option<u8>,
    is_given: bool,
}

// trait Region {
//     fn cell_coords(self, grid: &Grid) -> Vec<Coord>;
// }

#[derive(Debug, Copy, Clone)]
struct Row {
    y: u8,
}

#[derive(Debug, Copy, Clone)]
struct Column {
    x: u8,
}

#[derive(Debug, Copy, Clone)]
struct Square {
    size: u8,
    top_left: Coord,
}

#[derive(Debug, Copy, Clone)]
pub enum Region {
    Row(Row),
    Column(Column),
    Square(Square),
}

pub struct Solution {
    coord: Coord,
    value: u8,
}

impl Solution {
    pub fn new(coord: Coord, value: u8) -> Solution {
        Solution { coord, value }
    }
}

impl std::fmt::Display for Solution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.coord, self.value)
    }
}

impl Region {
    // TODO This should be computed once when the region is created and kept in memory.
    //  We should probably switch to structs instead of an enum? But then we probably end up in dynamic dispatch territory...
    pub fn cell_coords(self: &Self, grid: &Grid) -> HashSet<Coord> {
        let mut coords: HashSet<Coord> = HashSet::with_capacity(grid.size as usize);
        match self {
            Region::Row(row) => {
                for x in 0..grid.size {
                    coords.insert(Coord { x, y: row.y });
                }
            }
            Region::Column(column) => {
                for y in 0..grid.size {
                    coords.insert(Coord { x: column.x, y });
                }
            }
            Region::Square(subgrid) => {
                for y in 0..subgrid.size {
                    for x in 0..subgrid.size {
                        coords.insert(Coord {
                            x: subgrid.top_left.x + x,
                            y: subgrid.top_left.y + y,
                        });
                    }
                }
            }
        }
        coords
    }

    fn contains(self: &Self, cell: &Cell) -> bool {
        self.contains_coord(&cell.coord)
    }

    fn contains_coord(self: &Self, coord: &Coord) -> bool {
        match self {
            Region::Row(row) => coord.y == row.y,
            Region::Column(column) => coord.x == column.x,
            Region::Square(subgrid) => {
                let horizontal_range = subgrid.top_left.x..(subgrid.top_left.x + subgrid.size);
                let vertical_range = subgrid.top_left.y..(subgrid.top_left.y + subgrid.size);
                horizontal_range.contains(&coord.x) && vertical_range.contains(&coord.y)
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
                grid.cells.push(Cell {
                    coord: Coord { x, y },
                    candidates: HashSet::new(),
                    value: None,
                    is_given: false,
                })
            }

            // Since we're iterating over the size anyway we can set up our row & column regions here:
            grid.regions.push(Region::Row(Row { y }));
            grid.regions.push(Region::Column(Column { x: y }));
        }

        // TODO Make this more generic; It should be something like:
        //  "if the square root is a round number, let that be the subgrid size"
        if size == 9 {
            let subgrid_size = 3;
            for y in 0..subgrid_size {
                for x in 0..subgrid_size {
                    grid.regions.push(Region::Square(Square {
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

    fn coord_to_cell_index(self: &Self, coord: &Coord) -> usize {
        (coord.x + self.size * coord.y) as usize
    }

    pub fn set_given_value(self: &mut Self, coord: Coord, value: u8) {
        let index = self.coord_to_cell_index(&coord);
        self.cells[index].value = Some(value);
        self.cells[index].is_given = true;
    }

    fn regions_for_cell(self: &Self, cell: &Cell) -> Vec<&Region> {
        self.regions.iter().filter(|r| r.contains(cell)).collect()
    }

    fn regions_for_coord(self: &Self, coord: &Coord) -> Vec<Region> {
        self.regions
            .iter()
            .filter(|r| r.contains_coord(coord))
            .cloned()
            .collect()
    }

    fn cells_for_region(self: &Self, region: &Region) -> Vec<&Cell> {
        let coords = region.cell_coords(self);
        self.cells
            .iter()
            .filter(|cell| coords.contains(&cell.coord))
            .collect()
    }
    fn cells_for_region_mut(self: &mut Self, region: &Region) -> Vec<&mut Cell> {
        let coords = region.cell_coords(self);
        self.cells
            .iter_mut()
            .filter(|cell| coords.contains(&cell.coord))
            .collect()
    }

    fn grid_cell(self: &mut Self, coord: Coord) -> &mut Cell {
        self.cells
            .get_mut(coord.x as usize + coord.y as usize * self.size as usize)
            .expect("Coord should be in bounds")
    }

    pub fn compute_candidates(self: &mut Self) {
        let mut candidates: HashMap<Coord, HashSet<u8>> = HashMap::new();
        for cell in self.cells.iter() {
            if let Some(_) = cell.value {
                continue;
            }
            let regions = self.regions_for_cell(&cell);
            // Start out with all candidates
            let mut cell_candidates: HashSet<u8> = HashSet::from_iter(1..=self.size);
            for region in regions.iter() {
                let cells = self.cells_for_region(*region);
                for v in cells.iter().filter_map(|v| v.value) {
                    cell_candidates.remove(&v);
                }
            }
            candidates.insert(cell.coord, cell_candidates);
        }
        for cell in self.cells.iter_mut() {
            if let Some(cell_candidates) = candidates.get(&cell.coord) {
                cell.candidates = cell_candidates.clone();
            }
        }
    }

    pub fn apply(self: &mut Self, solutions: &[Solution]) {
        for solution in solutions {
            {
                let cell = self.grid_cell(solution.coord);
                cell.value = Some(solution.value);
                cell.candidates.clear();
            }
            {
                // Remove candidate from all other cells that share a region with the cell
                for region in self.regions_for_coord(&solution.coord) {
                    for other_cell in self.cells_for_region_mut(&region) {
                        other_cell.candidates.remove(&solution.value);
                    }
                }
            }
        }
    }

    pub fn is_solved(self: &Self) -> bool {
        !self.cells.iter().any(|c| c.value.is_none())
    }
}

// Calculate candidates:
//  - for each cell, get all regions, then get all given values in those regions, the take the complement of those values as candidates.
