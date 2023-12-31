use core::fmt::Debug;
use std::{
    collections::{HashMap, HashSet},
    ops::RangeInclusive,
};

#[derive(Debug, Copy, Clone, PartialEq, Hash, Eq)]
pub struct Coord {
    x: u8,
    y: u8,
}

impl Coord {
    pub fn new(x: u8, y: u8) -> Self {
        Self { x, y }
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

impl Cell {
    pub fn is_empty(&self) -> bool {
        self.value.is_none()
    }
}

pub struct Solution {
    coord: Coord,
    value: u8,
}

impl Solution {
    pub fn new(coord: Coord, value: u8) -> Self {
        Self { coord, value }
    }
}

impl std::fmt::Display for Solution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.coord, self.value)
    }
}

pub struct Reduction {
    coord: Coord,
    candidate: u8,
}

impl Reduction {
    pub fn new(coord: Coord, candidate: u8) -> Self {
        Self { coord, candidate }
    }
}

impl std::fmt::Display for Reduction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: -{}", self.coord, self.candidate)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Row {
    y: u8,
}

#[derive(Debug, Copy, Clone)]
pub struct Column {
    x: u8,
}

#[derive(Debug, Copy, Clone)]
pub struct Square {
    size: u8,
    top_left: Coord,
}

#[derive(Debug, Clone)]
pub enum RegionKind {
    Row(Row),
    Column(Column),
    Square(Square),
}

#[derive(Debug, Clone)]
pub struct Region {
    kind: RegionKind,
    coords: HashSet<Coord>,
}
impl Region {
    pub fn new(kind: RegionKind, grid: &Grid) -> Self {
        let mut region = Self {
            kind,
            coords: HashSet::with_capacity(grid.size as usize),
        };
        region.compute_coords(grid);
        region
    }

    fn contains(self: &Self, cell: &Cell) -> bool {
        self.contains_coord(&cell.coord)
    }

    fn contains_coord(self: &Self, coord: &Coord) -> bool {
        self.coords.contains(coord)
    }

    fn contains_coords(self: &Self, coords: &HashSet<Coord>) -> bool {
        coords.is_subset(&self.coords)
    }

    pub fn cell_coords(self: &Self) -> HashSet<Coord> {
        self.coords.clone()
    }

    pub fn cells_with_candidate(self: &Self, grid: &Grid, candidate: u8) -> HashSet<Coord> {
        let mut coords: HashSet<Coord> = HashSet::new();
        for cell in grid.cells_for_region(self) {
            if cell.candidates.contains(&candidate) {
                coords.insert(cell.coord);
            }
        }
        coords
    }

    fn compute_coords(self: &mut Self, grid: &Grid) {
        match self.kind {
            RegionKind::Row(row) => {
                for x in 0..grid.size {
                    self.coords.insert(Coord { x, y: row.y });
                }
            }
            RegionKind::Column(column) => {
                for y in 0..grid.size {
                    self.coords.insert(Coord { x: column.x, y });
                }
            }
            RegionKind::Square(subgrid) => {
                for y in 0..subgrid.size {
                    for x in 0..subgrid.size {
                        self.coords.insert(Coord {
                            x: subgrid.top_left.x + x,
                            y: subgrid.top_left.y + y,
                        });
                    }
                }
            }
        }
    }
}

impl std::fmt::Display for Region {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{{:?}, {:?}}}", self.kind, self.coords)
    }
}

#[derive(Debug)]
pub struct Grid {
    pub size: u8,
    pub cells: Vec<Cell>,
    pub regions: Vec<Region>,
}

/// Returns the subgrid size if the square root of the grid size is a whole number
fn subgrid_size(size: u8) -> Option<u8> {
    let square_root = (size as f32).sqrt();
    let integer_part = square_root.trunc();
    if square_root == integer_part {
        Some(integer_part as u8)
    } else {
        None
    }
}

impl Grid {
    pub fn new(size: u8) -> Self {
        let mut grid = Grid {
            size,
            cells: Vec::with_capacity(size as usize * size as usize),
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
            grid.regions
                .push(Region::new(RegionKind::Row(Row { y }), &grid));
            grid.regions
                .push(Region::new(RegionKind::Column(Column { x: y }), &grid));
        }

        if let Some(s) = subgrid_size(size) {
            for y in 0..s {
                for x in 0..s {
                    grid.regions.push(Region::new(
                        RegionKind::Square(Square {
                            size: s,
                            top_left: Coord { x: x * s, y: y * s },
                        }),
                        &grid,
                    ));
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

    pub fn regions_for_coords(self: &Self, coords: &HashSet<Coord>) -> Vec<&Region> {
        self.regions
            .iter()
            .filter(|r| r.contains_coords(coords))
            .collect()
    }

    pub fn cells_for_region(self: &Self, region: &Region) -> Vec<&Cell> {
        let coords = region.cell_coords();
        self.cells
            .iter()
            .filter(|cell| coords.contains(&cell.coord))
            .collect()
    }
    fn cells_for_region_mut(self: &mut Self, region: &Region) -> Vec<&mut Cell> {
        let coords = region.cell_coords();
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

    pub fn apply_solutions(self: &mut Self, solutions: &[Solution]) {
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

    pub fn apply_reductions(self: &mut Self, reductions: &[Reduction]) {
        for reduction in reductions {
            let cell = self.grid_cell(reduction.coord);
            cell.candidates.remove(&reduction.candidate);
        }
    }

    pub fn is_solved(self: &Self) -> bool {
        !self.cells.iter().any(|c| c.value.is_none())
    }

    pub fn possible_values(self: &Self) -> RangeInclusive<u8> {
        return 1..=self.size;
    }
}

// Calculate candidates:
//  - for each cell, get all regions, then get all given values in those regions, the take the complement of those values as candidates.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grid_of_nine_has_subgrids() {
        let grid = Grid::new(9);
        let squares: Vec<&Region> = grid
            .regions
            .iter()
            .filter(|r| matches!(r.kind, RegionKind::Square(_)))
            .collect();
        let num_squares = squares.len();
        assert_eq!(num_squares, 9);
        if let RegionKind::Square(square) = squares[0].kind {
            assert_eq!(square.size, 3);
        }
    }
    #[test]
    fn grid_of_sixteen_has_subgrids() {
        let grid = Grid::new(16);
        let squares: Vec<&Region> = grid
            .regions
            .iter()
            .filter(|r| matches!(r.kind, RegionKind::Square(_)))
            .collect();
        let num_squares = squares.len();
        assert_eq!(num_squares, 16);
        if let RegionKind::Square(square) = squares[0].kind {
            assert_eq!(square.size, 4);
        }
    }
    #[test]
    fn grid_of_twelve_does_not_have_subgrids() {
        let grid = Grid::new(12);
        let squares: Vec<&Region> = grid
            .regions
            .iter()
            .filter(|r| matches!(r.kind, RegionKind::Square(_)))
            .collect();
        let num_squares = squares.len();
        assert_eq!(num_squares, 0);
    }
}
