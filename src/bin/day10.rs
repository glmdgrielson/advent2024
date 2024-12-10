//! # Day 10: Hoof It
//!
//! The Lava Island has a thriving hiking culture apparently!
//! Unfortunately, the obvious happened to the documentation
//! and now we have to fix it.

use std::collections::HashSet;
use std::fs::read_to_string;

use advent2024::AdventError;

use simple_grid::{Grid, GridIndex};

/// Puzzle input consists of a grid of numbers,
/// representing a topological map.
fn parse_input(file: &str) -> Result<Grid<u32>, AdventError> {
    let mut grid = Grid::new(0, 0, Vec::new());

    for line in file.lines() {
        let row = line
            .chars()
            .map(|ch| {
                ch.to_digit(10)
                    .ok_or_else(|| AdventError::Parse(format!("Invalid digit {}", ch)))
            })
            .collect::<Result<Vec<_>, _>>()?;

        grid.push_row(row);
    }

    Ok(grid)
}

/// Find all of the trails on the map.
///
/// A trail is defined as a contiguous sequence of the digits
/// zero through nine cardinally adjacent to one another.
fn part_one(data: &Grid<u32>) -> usize {
    let zeroes = data.indices().filter(|&idx| data[idx] == 0);
    zeroes.map(|idx| find_trail(data, idx)).sum()
}

/// Given a spot on the map, find the number of trails
/// going from it.
///
/// # Panics
///
/// This assumes that the index given corresponds to the
/// start of a trail, i.e. that it represents a `0` in the grid.
/// It will panic if this assumption is violated.
fn find_trail(data: &Grid<u32>, idx: GridIndex) -> usize {
    assert!(data[idx] == 0, "Invalid trailhead");
    let set = data
        .cardinal_neighbor_indices_of(idx)
        .filter(|&idx| data[idx] == 1)
        .flat_map(|idx| data.cardinal_neighbor_indices_of(idx))
        .filter(|&idx| data[idx] == 2)
        .flat_map(|idx| data.cardinal_neighbor_indices_of(idx))
        .filter(|&idx| data[idx] == 3)
        .flat_map(|idx| data.cardinal_neighbor_indices_of(idx))
        .filter(|&idx| data[idx] == 4)
        .flat_map(|idx| data.cardinal_neighbor_indices_of(idx))
        .filter(|&idx| data[idx] == 5)
        .flat_map(|idx| data.cardinal_neighbor_indices_of(idx))
        .filter(|&idx| data[idx] == 6)
        .flat_map(|idx| data.cardinal_neighbor_indices_of(idx))
        .filter(|&idx| data[idx] == 7)
        .flat_map(|idx| data.cardinal_neighbor_indices_of(idx))
        .filter(|&idx| data[idx] == 8)
        .flat_map(|idx| data.cardinal_neighbor_indices_of(idx))
        .filter(|&idx| data[idx] == 9)
        .collect::<HashSet<_>>();

    set.len()
}

/// Find the number of unique trails on the map.
fn part_two(data: &Grid<u32>) -> usize {
    let zeroes = data.indices().filter(|&idx| data[idx] == 0);
    zeroes.map(|idx| find_paths(data, idx)).sum()
}

/// Given a trailhead, find the number of trails leading off from it.
///
/// # Panics
///
/// Panics if `idx` is not a trailhead, i.e. `data[idx] != 0`.
fn find_paths(data: &Grid<u32>, idx: GridIndex) -> usize {
    assert!(data[idx] == 0, "Invalid trailhead");
    data.cardinal_neighbor_indices_of(idx)
        .filter(|&idx| data[idx] == 1)
        .flat_map(|idx| data.cardinal_neighbor_indices_of(idx))
        .filter(|&idx| data[idx] == 2)
        .flat_map(|idx| data.cardinal_neighbor_indices_of(idx))
        .filter(|&idx| data[idx] == 3)
        .flat_map(|idx| data.cardinal_neighbor_indices_of(idx))
        .filter(|&idx| data[idx] == 4)
        .flat_map(|idx| data.cardinal_neighbor_indices_of(idx))
        .filter(|&idx| data[idx] == 5)
        .flat_map(|idx| data.cardinal_neighbor_indices_of(idx))
        .filter(|&idx| data[idx] == 6)
        .flat_map(|idx| data.cardinal_neighbor_indices_of(idx))
        .filter(|&idx| data[idx] == 7)
        .flat_map(|idx| data.cardinal_neighbor_indices_of(idx))
        .filter(|&idx| data[idx] == 8)
        .flat_map(|idx| data.cardinal_neighbor_indices_of(idx))
        .filter(|&idx| data[idx] == 9)
        .count()
}

fn main() -> Result<(), AdventError> {
    let file = read_to_string("src/input/day10.txt")?;
    let data = parse_input(&file)?;

    println!("There are {} trails on the map.", part_one(&data));
    println!("There are {} paths on the map.", part_two(&data));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::sync::LazyLock;

    static INPUT: LazyLock<Grid<u32>> = LazyLock::new(|| {
        let file = read_to_string("src/input/day10-test.txt").unwrap();

        parse_input(&file).unwrap()
    });

    #[test]
    fn test_parse_input() {
        let data = &*INPUT;

        let row = data.row_iter(0).collect::<Vec<_>>();

        assert_eq!(row, vec![&8, &9, &0, &1, &0, &1, &2, &3]);
    }

    #[test]
    fn test_part_one() {
        let data = &*INPUT;

        assert_eq!(part_one(data), 36);
    }

    #[test]
    fn test_find_trail() {
        let data = &*INPUT;

        let index = GridIndex::new(2, 0);

        assert_eq!(find_trail(data, index), 5);
    }

    #[test]
    fn test_part_two() {
        let data = &*INPUT;

        assert_eq!(part_two(data), 81);
    }

    #[test]
    fn test_find_paths() {
        let data = &*INPUT;

        let index = GridIndex::new(2, 0);

        assert_eq!(find_paths(data, index), 20);
    }
}
