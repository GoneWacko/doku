use std::collections::HashSet;

use super::ReduceStrategy;
use crate::data::{Cell, Coord, Grid, Reduction};

/// Generates all combinations of size `k` from `items` and passes each combination
/// slice to callback `f`.
fn for_each_combination<'a, T>(
    items: &'a [T],
    k: usize,
    mut f: impl FnMut(&[&'a T]),
) {
    let n = items.len();
    if k == 0 || k > n {
        return;
    }

    let mut indices: Vec<usize> = (0..k).collect();
    let mut combo: Vec<&'a T> = indices.iter().map(|&i| &items[i]).collect();

    loop {
        f(&combo);

        // Find the rightmost index that has not reached its maximum value
        let mut i = k;
        while i > 0 {
            i -= 1;
            if indices[i] != n - k + i {
                break;
            }
        }

        if indices[i] == n - k + i {
            // All indices have reached their maximum
            break;
        }

        indices[i] += 1;
        combo[i] = &items[indices[i]];
        for j in (i + 1)..k {
            indices[j] = indices[j - 1] + 1;
            combo[j] = &items[indices[j]];
        }
    }
}

/// A generalized naked subset strategy (Naked Pair, Naked Triple, Naked Quad, etc.).
///
/// In any region (row, column, square, or extra region), if a subset of N empty cells
/// has a candidate union of size exactly N, those N candidates cannot appear in any
/// other cell of that region.
pub struct NakedTuple;

impl NakedTuple {
    /// Returns the default maximum tuple size to check for a given grid.
    /// In a standard 9x9 puzzle, checking up to size 4 (quads) is standard.
    pub fn default_max_tuple_size(grid: &Grid) -> usize {
        (grid.size as usize / 2).max(2)
    }

    /// Finds candidate eliminations by searching for naked tuples of a specific size `tuple_size`.
    pub fn reduce_candidates_for_size(grid: &Grid, tuple_size: usize) -> Vec<Reduction> {
        if tuple_size < 2 {
            return Vec::new();
        }

        let mut reductions: Vec<Reduction> = Vec::new();
        let mut seen_reductions: HashSet<(Coord, u8)> = HashSet::new();

        for region in &grid.regions {
            let empty_cells: Vec<&Cell> = grid
                .cells_for_region(region)
                .into_iter()
                .filter(|c| c.is_empty())
                .collect();

            // Need more empty cells than the tuple size to have other cells to reduce
            if empty_cells.len() <= tuple_size {
                continue;
            }

            // A cell can only be part of an N-tuple if it has between 2 and N candidates
            let eligible_cells: Vec<&Cell> = empty_cells
                .iter()
                .filter(|c| c.candidates.len() >= 2 && c.candidates.len() <= tuple_size)
                .copied()
                .collect();

            if eligible_cells.len() < tuple_size {
                continue;
            }

            let mut used_in_tuple: HashSet<Coord> = HashSet::new();

            for_each_combination(&eligible_cells, tuple_size, |combo| {
                // If any cell in this combo is already part of a found tuple in this region, skip
                if combo.iter().any(|c| used_in_tuple.contains(&c.coord)) {
                    return;
                }

                let mut candidate_union: HashSet<u8> = HashSet::new();
                for cell in combo {
                    candidate_union.extend(&cell.candidates);
                    if candidate_union.len() > tuple_size {
                        return;
                    }
                }

                if candidate_union.len() == tuple_size {
                    // Lock these cells into the tuple for this region
                    for cell in combo {
                        used_in_tuple.insert(cell.coord);
                    }

                    // Eliminate the tuple's candidates from all other cells in the region
                    for other_cell in &empty_cells {
                        if used_in_tuple.contains(&other_cell.coord) {
                            continue;
                        }

                        for candidate in &other_cell.candidates {
                            if candidate_union.contains(candidate)
                                && seen_reductions.insert((other_cell.coord, *candidate))
                            {
                                reductions.push(Reduction::new(other_cell.coord, *candidate));
                            }
                        }
                    }
                }
            });
        }

        reductions
    }

    /// Finds reductions across tuple sizes from 2 up to `max_size`.
    /// Checks smaller tuple sizes first, returning as soon as reductions are found.
    pub fn reduce_candidates_up_to_size(grid: &Grid, max_size: usize) -> Vec<Reduction> {
        for size in 2..=max_size {
            let reductions = Self::reduce_candidates_for_size(grid, size);
            if !reductions.is_empty() {
                return reductions;
            }
        }
        Vec::new()
    }
}

impl ReduceStrategy for NakedTuple {
    fn reduce_candidates(grid: &Grid) -> Vec<Reduction> {
        Self::reduce_candidates_up_to_size(grid, Self::default_max_tuple_size(grid))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::Coord;

    #[test]
    fn test_combination_generator() {
        let items = [1, 2, 3, 4];
        let mut combos = Vec::new();
        for_each_combination(&items, 2, |c| {
            combos.push(c.iter().map(|&&x| x).collect::<Vec<_>>());
        });
        assert_eq!(combos.len(), 6);
        assert_eq!(combos[0], vec![1, 2]);
        assert_eq!(combos[1], vec![1, 3]);
        assert_eq!(combos[2], vec![1, 4]);
        assert_eq!(combos[3], vec![2, 3]);
        assert_eq!(combos[4], vec![2, 4]);
        assert_eq!(combos[5], vec![3, 4]);
    }

    #[test]
    fn test_naked_pair() {
        let mut grid = Grid::new(9);
        grid.cells[0].candidates = HashSet::from([1, 2]);
        grid.cells[1].candidates = HashSet::from([1, 2]);
        grid.cells[2].candidates = HashSet::from([1, 2, 3]);
        grid.cells[3].candidates = HashSet::from([2, 4]);
        grid.cells[4].candidates = HashSet::from([5, 6]);
        for x in 5..9 {
            grid.cells[x].value = Some((x + 1) as u8);
        }

        let reductions = NakedTuple::reduce_candidates_for_size(&grid, 2);
        let reduction_set: HashSet<(Coord, u8)> =
            reductions.into_iter().map(|r| (r.coord, r.candidate)).collect();

        assert!(reduction_set.contains(&(Coord::new(2, 0), 1)));
        assert!(reduction_set.contains(&(Coord::new(2, 0), 2)));
        assert!(reduction_set.contains(&(Coord::new(3, 0), 2)));
        assert_eq!(reduction_set.len(), 3);
    }

    #[test]
    fn test_naked_triple_partial_subsets() {
        let mut grid = Grid::new(9);
        // Classic naked triple where no single cell has all three candidates:
        // (0,0): {1, 2}
        // (1,0): {2, 3}
        // (2,0): {1, 3}
        // (3,0): {1, 4}
        // (4,0): {3, 5}
        // (5,0): {6, 7}
        grid.cells[0].candidates = HashSet::from([1, 2]);
        grid.cells[1].candidates = HashSet::from([2, 3]);
        grid.cells[2].candidates = HashSet::from([1, 3]);
        grid.cells[3].candidates = HashSet::from([1, 4]);
        grid.cells[4].candidates = HashSet::from([3, 5]);
        grid.cells[5].candidates = HashSet::from([6, 7]);
        for x in 6..9 {
            grid.cells[x].value = Some((x + 1) as u8);
        }

        // Size 2 should find nothing (no pair)
        let pair_reductions = NakedTuple::reduce_candidates_for_size(&grid, 2);
        assert!(pair_reductions.is_empty());

        // Size 3 should find the triple {1, 2, 3}
        let reductions = NakedTuple::reduce_candidates_for_size(&grid, 3);
        let reduction_set: HashSet<(Coord, u8)> =
            reductions.into_iter().map(|r| (r.coord, r.candidate)).collect();

        assert!(reduction_set.contains(&(Coord::new(3, 0), 1)));
        assert!(reduction_set.contains(&(Coord::new(4, 0), 3)));
        assert_eq!(reduction_set.len(), 2);
    }

    #[test]
    fn test_naked_quad() {
        let mut grid = Grid::new(9);
        // Naked quad:
        // (0,0): {1, 2}
        // (1,0): {2, 3}
        // (2,0): {3, 4}
        // (3,0): {1, 4}
        // (4,0): {1, 5}
        // (5,0): {4, 6}
        // (6,0): {7, 8}
        grid.cells[0].candidates = HashSet::from([1, 2]);
        grid.cells[1].candidates = HashSet::from([2, 3]);
        grid.cells[2].candidates = HashSet::from([3, 4]);
        grid.cells[3].candidates = HashSet::from([1, 4]);
        grid.cells[4].candidates = HashSet::from([1, 5]);
        grid.cells[5].candidates = HashSet::from([4, 6]);
        grid.cells[6].candidates = HashSet::from([7, 8]);
        for x in 7..9 {
            grid.cells[x].value = Some((x + 1) as u8);
        }

        assert!(NakedTuple::reduce_candidates_for_size(&grid, 2).is_empty());
        assert!(NakedTuple::reduce_candidates_for_size(&grid, 3).is_empty());

        let reductions = NakedTuple::reduce_candidates_for_size(&grid, 4);
        let reduction_set: HashSet<(Coord, u8)> =
            reductions.into_iter().map(|r| (r.coord, r.candidate)).collect();

        assert!(reduction_set.contains(&(Coord::new(4, 0), 1)));
        assert!(reduction_set.contains(&(Coord::new(5, 0), 4)));
        assert_eq!(reduction_set.len(), 2);
    }
}
