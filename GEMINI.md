# Doku Developer Guide

`doku` is a logic-based Sudoku solver written in Rust (2021 edition) as a learning project. Rather than using backtracking / brute-force search, it solves puzzles step-by-step using human-style logical deduction strategies.

## Build and Run Commands

- **Build**: `cargo build`
- **Run (default puzzle)**: `cargo run` (loads a hardcoded default puzzle configured in `src/main.rs`)
- **Run (specific puzzle)**: `cargo run puzzles/<puzzle_file>.txt`
- **Run Tests**: `cargo test`
  - *Note*: `data::tests::grid_of_twelve_does_not_have_subgrids` currently fails because `Grid::new` enforces that grid sizes must have an integer square root.
- **Lint / Check**: `cargo clippy` or `cargo check`

## Architecture Overview

- **`src/main.rs`**: Entry point and solving loop.
  - Parses CLI arguments to select the puzzle file.
  - Loads puzzle files via `load_puzzle`.
  - Loops over solving and reduction strategies sequentially (`Single` -> `RegionIntersection` -> `NakedTuple` -> `SimpleLink`) until the grid is solved or stuck.
  - Validates the final solved grid via `grid.validate()`.
- **`src/data.rs`**: Core data models.
  - `Coord`: Zero-indexed `(x, y)` grid coordinates.
  - `Cell`: Contains coordinates, placed value (`Option<u8>`), candidate set (`HashSet<u8>`), and `is_given` flag.
  - `RegionKind` / `Region`: Models logical regions enforcing uniqueness (`Row`, `Column`, standard `Square`, or arbitrary custom square regions).
  - `Grid`: Holds puzzle dimensions, cells, and registered regions. Manages candidate computation (`compute_candidates`), applying cell solutions (`apply_solutions`), and candidate pruning (`apply_reductions`).
  - `Solution`: Represents assigning a definite value to a cell `(coord, value)`.
  - `Reduction`: Represents removing an invalid candidate from a cell `(coord, candidate)`.
- **`src/strategies.rs`**: Defines the traits for solving techniques.
  - `SolveStrategy`: Generates cell values (`Vec<Solution>`).
  - `ReduceStrategy`: Generates candidate eliminations (`Vec<Reduction>`).
- **`src/strategies/`**: Concrete strategy implementations.
  - `single.rs` (`Single`): Finds cells with only one remaining candidate.
  - `naked_tuple.rs` (`NakedTuple`): Identifies naked subsets of size N (pairs, triples, quads) sharing N candidates within a region and eliminates those candidates from other cells in that region.
  - `region_intersection.rs` (`RegionIntersection`): Intersections between regions (pointing/claiming) where candidates restricted to an intersection eliminate candidates in the rest of the intersecting region.
  - `simple_link.rs` (`SimpleLink`): Two-cell candidate links across intersecting regions (useful in puzzles with overlapping/extra regions).
- **`src/output.rs`**: Formats the grid and candidate lists for terminal output.
- **`puzzles/`**: Text-based puzzle definitions.
  - Header: `size <N>` (e.g. `size 9`).
  - Rows: Space-separated numbers and `.` for empty cells.
  - Optional extras: `extra square <x>,<y>` for non-standard variants (e.g. Windoku / Hyper Sudoku).

## Design Philosophy & Guidelines

- **Human-Centric Solving**: Favor logical deduction techniques that can explain *why* a candidate is removed or placed, avoiding brute-force guess-and-backtrack solvers.
- **Extensible Regions**: Puzzles can define extra regions beyond classic rows, columns, and 3x3 boxes. Strategies should query regions abstractly where possible.
- **Code Style Context**: The codebase was written while learning Rust. When refactoring or making improvements:
  - Transition non-idiomatic `self: &Self` / `self: &mut Self` signatures to standard `&self` / `&mut self`.
  - Address clippy suggestions thoughtfully without sacrificing readability.
