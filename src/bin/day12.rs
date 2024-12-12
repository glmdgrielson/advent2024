//! Day 12: Garden Groups
//!
//! Some Elves want some help with their garden.
//! They don't know how much budget they're gonna
//! need for their complicated garden plots. Oh boy...

use std::collections::{HashSet, VecDeque};
use std::fs::read_to_string;

use advent2024::AdventError;

use simple_grid::{Grid, GridIndex};

#[derive(Clone, Debug, PartialEq, Eq)]
struct Plot(char, Vec<GridIndex>);

impl Plot {
    /// Find the area of this plot.
    ///
    /// Area is defined as the number of cells
    /// this plot contains.
    fn area(&self) -> usize {
        self.1.len()
    }

    fn perimeter(&self, grid: &Grid<char>) -> usize {
        self.1
            .iter()
            .flat_map(|&cell| {
                [
                    grid.up_cell(cell),
                    grid.left_cell(cell),
                    grid.right_cell(cell),
                    grid.down_cell(cell),
                ]
            })
            .filter(|symbol| match *symbol {
                Some(symbol) => *symbol != self.0,
                None => true,
            })
            .count()
    }
}

/// File consists of a grid of characters.
///
/// I realize now this function is technically infalliable...
fn parse_input(file: &str) -> Result<Grid<char>, AdventError> {
    let mut grid = Grid::new(0, 0, Vec::new());
    for line in file.lines() {
        grid.push_row(line.chars().collect());
    }
    Ok(grid)
}

fn find_region(grid: &Grid<char>, point: GridIndex) -> Plot {
    let symbol = grid[point];
    let mut visited = HashSet::new();
    let mut frontier = VecDeque::new();

    frontier.push_front(point);
    visited.insert(point);

    while let Some(pos) = frontier.pop_front() {
        for neighbor in grid.cardinal_neighbor_indices_of(pos) {
            if visited.contains(&neighbor) {
                continue;
            } else if grid[neighbor] == symbol {
                frontier.push_back(neighbor);
                visited.insert(neighbor);
            }
        }
    }

    Plot(symbol, visited.into_iter().collect())
}

fn part_one(data: &Grid<char>) -> usize {
    let mut regions = Vec::new();
    let mut visited = HashSet::new();

    for cell in data.indices() {
        if visited.contains(&cell) {
            continue;
        }

        let region = find_region(data, cell);
        let cells = region.1.clone();
        regions.push(region);
        for cell in cells {
            visited.insert(cell);
        }
    }

    regions
        .iter()
        .map(|region| region.area() * region.perimeter(data))
        .sum()
}

fn main() -> Result<(), AdventError> {
    let file = read_to_string("src/input/day12.txt")?;
    let data = parse_input(&file)?;

    println!("The price of all the fencing is {}", part_one(&data));
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    use std::sync::LazyLock;

    static INPUT: LazyLock<Grid<char>> = LazyLock::new(|| {
        let file = read_to_string("src/input/day12-test.txt").unwrap();

        parse_input(&file).unwrap()
    });

    #[test]
    fn test_part_one() {
        let data = &*INPUT;

        assert_eq!(part_one(data), 1930);
    }
}
