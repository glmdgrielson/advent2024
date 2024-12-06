//! Day 06: Guard Gallivant
//!
//! We done did a time travel. Oops. Now we
//! need to be careful to avoid the locals
//! so we don't cause time whoopsies!

#![allow(dead_code)]

use std::collections::HashSet;
use std::fs::read_to_string;

use advent2024::AdventError;

use simple_grid::{Grid, GridIndex};

#[derive(Clone, Debug, PartialEq, Eq)]
/// The direction a guard is facing.
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Clone, PartialEq, Eq, Debug)]
/// A guard from the good old year of 1518.
struct Guard {
    position: GridIndex,
    direction: Direction,
}

impl Default for Guard {
    fn default() -> Self {
        Guard {
            position: GridIndex::new(0, 0),
            direction: Direction::North,
        }
    }
}

/// Input consists of a grid of characters representing the map.
///
/// The grid contains one of three characters.
/// 1. `.` representing empty space.
/// 2. `#` representing one of the many bits of junk making a mess of
///    the space.
/// 3. `^` representing the guard's initial position.
fn parse_input(file: &str) -> Result<(Guard, Grid<bool>), AdventError> {
    let mut grid = Grid::new(0, 0, vec![]);
    let mut guard = Guard {
        ..Default::default()
    };

    for (row, line) in file.lines().enumerate() {
        let row = line
            .chars()
            .enumerate()
            .map(|(col, ch)| match ch {
                '.' => Ok(false),
                '#' => Ok(true),
                '^' => {
                    guard.position = GridIndex::new(col, row);
                    Ok(false)
                }
                _ => Err(AdventError::Parse(format!("Invalid character {}", ch))),
            })
            .collect::<Result<Vec<_>, _>>()?;

        grid.push_row(row);
    }

    Ok((guard, grid))
}

/// Find the amount of space the guard takes up.
///
/// The guard will move as far as they can in their current
/// direction until they find an obstacle, which in this
/// case is represented by [Grid::get] returning `Some(true)`.
///
/// When `Some(false)` is returned, that says that there is
/// still space to move. When [Option::None] is returned,
/// that means that the guard has moved off the space the
/// grid represents and we have our answer.
fn part_one(guard: &Guard, grid: &Grid<bool>) -> usize {
    let mut set = HashSet::new();
    set.insert(guard.position);

    let mut guard = guard.clone();
    loop {
        match guard.direction {
            Direction::North => match grid.up_cell(guard.position) {
                Some(true) => {
                    guard.direction = Direction::East;
                }
                Some(false) => {
                    // This `.expect()` is safe since we _just_ proved it exists.
                    let next = grid.up_index(guard.position).expect("Index should exist");
                    set.insert(next);
                    guard.position = next;
                }
                None => break,
            },
            Direction::East => match grid.right_cell(guard.position) {
                Some(true) => {
                    guard.direction = Direction::South;
                }
                Some(false) => {
                    let next = grid
                        .right_index(guard.position)
                        .expect("Index should exist");
                    set.insert(next);
                    guard.position = next;
                }
                None => break,
            },
            Direction::South => match grid.down_cell(guard.position) {
                Some(true) => {
                    guard.direction = Direction::West;
                }
                Some(false) => {
                    let next = grid.down_index(guard.position).expect("Index should exist");
                    set.insert(next);
                    guard.position = next;
                }
                None => break,
            },
            Direction::West => match grid.left_cell(guard.position) {
                Some(true) => {
                    guard.direction = Direction::North;
                }
                Some(false) => {
                    let next = grid.left_index(guard.position).expect("Index should exist");
                    set.insert(next);
                    guard.position = next;
                }
                None => break,
            },
        }
    }

    set.len()
}

/// Find the number of points where a loop can
/// be created in the guard's path.
fn part_two(guard: &Guard, grid: &Grid<bool>) -> usize {
    todo!()
}

fn main() -> Result<(), AdventError> {
    let file = read_to_string("src/input/day06.txt")?;
    let (guard, grid) = parse_input(&file)?;

    println!("The guard passes through {} points", part_one(&guard, &grid));
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    use std::sync::LazyLock;

    static INPUT: LazyLock<(Guard, Grid<bool>)> = LazyLock::new(|| {
        let file = read_to_string("src/input/day06-test.txt").unwrap();

        parse_input(&file).unwrap()
    });

    #[test]
    fn test_parse_input() {
        let (guard, grid) = &*INPUT;

        assert_eq!(guard.position, GridIndex::new(4, 6));

        assert_eq!(grid.height(), 10, "Grid size incorrectly parsed");
        assert_eq!(grid.width(), 10, "Grid size incorrectly parsed");
        assert_eq!(grid[(4, 0)], true, "Grid data incorrectly parsed");
    }

    #[test]
    fn test_part_one() {
        let (guard, grid) = &*INPUT;

        assert_eq!(part_one(guard, grid), 41);
    }

    #[test]
    fn test_part_two() {
        let (guard, grid) = &*INPUT;

        assert_eq!(part_two(guard, grid), 6);
    }
}
