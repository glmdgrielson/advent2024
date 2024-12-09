//! # Day 08: Resident Collinearity
//!
//! The Easter Bunny is back at it again! We need to
//! put a stop to his plan to sell terrible Easter
//! chocolate for Christmas! (Wait a few months, pal!)

use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

use advent2024::AdventError;

use itertools::Itertools;
use simple_grid::{Grid, GridIndex};

/// Puzzle input consists of a grid of characters,
/// representing a map with antennae.
///
/// Any non-period character represents an antennae
/// sending a particular frequency.
fn parse_input(file: &str) -> Grid<char> {
    let mut grid = Grid::new(0, 0, Vec::new());

    for line in file.lines() {
        let row = line.chars().collect::<Vec<_>>();

        grid.push_row(row);
    }

    grid
}

/// Find the number of antinodes on the map.
///
/// Two antinodes are created for every pair
/// of antennae with the same character.
fn part_one(data: &Grid<char>) -> usize {
    let antennae = data
        .indices()
        .filter(|&idx| data[idx] != '.')
        .map(|idx| (data[idx], idx))
        .into_group_map();

    antennae
        .values()
        .flat_map(|nodes| {
            nodes.into_iter().permutations(2).map(|pair| {
                let one = pair[0];
                let two = pair[1];

                let x1 = one.column() as isize;
                let x2 = two.column() as isize;
                let x = x2 - x1;

                let y1 = one.row() as isize;
                let y2 = two.row() as isize;
                let y = y2 - y1;

                let x = x2 as isize + x;
                let x = x as usize;

                let y = y2 as isize + y;
                let y = y as usize;

                GridIndex::new(x, y)
            })
        })
        .filter(|idx| data.get(*idx).is_some())
        .collect::<HashSet<_>>()
        .len()
}

/// Find the number of resonant antinodes on the map.
///
/// It turns out that thanks to resonant harmonics,
/// two antennae can produce an _infinite_ number of
/// antinodes, not just one each.
fn part_two(data: &Grid<char>) -> usize {
    let antennae = data
        .indices()
        .filter(|&idx| data[idx] != '.')
        .map(|idx| (data[idx], idx))
        .into_group_map();

    antennae
        .values()
        .flat_map(|nodes| {
            nodes.into_iter().permutations(2).flat_map(|pair| {
                let one = pair[0];
                let two = pair[1];

                let x1 = one.column() as isize;
                let x2 = two.column() as isize;
                let x = x2 - x1;

                let y1 = one.row() as isize;
                let y2 = two.row() as isize;
                let y = y2 - y1;

                // Something about this feels hacky
                // but somehow it all compiles nicely!
                (0..)
                    .map(move |count| {
                        let x = x2 as isize + x * count;
                        let x = x as usize;

                        let y = y2 as isize + y * count;
                        let y = y as usize;

                        GridIndex::new(x, y)
                    })
                    .take_while(|idx| data.get(*idx).is_some())
            })
        })
        .filter(|idx| data.get(*idx).is_some())
        .collect::<HashSet<_>>()
        .len()
}

fn main() -> Result<(), AdventError> {
    let file = read_to_string("src/input/day08.txt")?;
    let data = parse_input(&file);

    println!("The number of antinodes is {}", part_one(&data));
    println!("The number of resonant antinodes is {}", part_two(&data));
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    use std::sync::LazyLock;

    static INPUT: LazyLock<Grid<char>> = LazyLock::new(|| {
        let file = read_to_string("src/input/day08-test.txt").unwrap();

        parse_input(&file)
    });

    #[test]
    fn test_parse_input() {
        let data = &*INPUT;

        assert_eq!(data.get((8, 1)), Some(&'0'));
    }

    #[test]
    fn test_part_one() {
        let data = &*INPUT;

        assert_eq!(part_one(data), 14);
    }

    #[test]
    fn test_part_two() {
        let data = &*INPUT;

        assert_eq!(part_two(&data), 34);
    }
}
