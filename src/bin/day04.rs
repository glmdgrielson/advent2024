//! Day 04: Ceres Search
//!
//! ...we need to solve a word search. Gosh dang it.

use std::fs::read_to_string;

use advent2024::AdventError;
use simple_grid::{Grid, GridIndex};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Letter {
    X,
    M,
    A,
    S,
}

/// Input consists of a grid of letters.
fn parse_input(file: &str) -> Result<Grid<Letter>, AdventError> {
    let mut grid = Grid::new(0, 0, vec![]);

    for line in file.lines() {
        let chars = line
            .chars()
            .map(|ch| match ch {
                'X' => Ok(Letter::X),
                'M' => Ok(Letter::M),
                'A' => Ok(Letter::A),
                'S' => Ok(Letter::S),
                _ => Err(AdventError::Parse(format!("Invalid character {}", ch))),
            })
            .collect::<Result<Vec<_>, _>>()?;

        grid.push_row(chars);
    }

    Ok(grid)
}

/// Find the number of times "XMAS" appears in the word search.
///
/// You're _joking_, right? Right?
fn part_one(data: &Grid<Letter>) -> usize {
    data.indices()
        .filter(|&idx| data[idx] == Letter::X)
        .map(|idx| {
            // Gather the direction of any adjacent M's.
            let m_indices = find_neighbors(data, idx)
                .into_iter()
                .enumerate()
                .filter(|(_, cell)| cell.is_some_and(|ch| data.get(ch) == Some(&Letter::M)))
                .collect::<Vec<_>>();
            let a_indices = m_indices.into_iter().filter_map(|(dir, idx)| {
                let Some(idx) = idx else {
                    return None;
                };
                let idx = find_neighbors(data, idx)[dir]?;

                match data.get(idx) {
                    Some(Letter::A) => Some((dir, idx)),
                    _ => None,
                }
            });
            let s_indices = a_indices.into_iter().filter_map(|(dir, idx)| {
                let idx = find_neighbors(data, idx)[dir]?;

                match data.get(idx) {
                    Some(Letter::S) => Some((dir, idx)),
                    _ => None,
                }
            }).count();

            s_indices
        })
        .sum()
}

fn find_neighbors<T>(grid: &Grid<T>, idx: GridIndex) -> Vec<Option<GridIndex>> {
    let neighbors = [
        idx.up(),
        idx.up_right(),
        idx.right(),
        idx.down_right(),
        idx.down(),
        idx.down_left(),
        idx.left(),
        idx.up_left(),
    ];

    neighbors
        .iter()
        .map(|idx| match idx {
            Some(idx) => grid.get(*idx).map(|_| *idx),
            None => None,
        })
        .collect()
}

fn main() -> Result<(), AdventError> {
    let file = read_to_string("src/input/day04.txt")?;
    let data = parse_input(&file)?;

    println!("The number of XMAS instances is {}", part_one(&data));
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    use std::sync::LazyLock;

    static INPUT: LazyLock<Grid<Letter>> = LazyLock::new(|| {
        let file = read_to_string("src/input/day04-test.txt").unwrap();

        parse_input(&file).unwrap()
    });

    #[test]
    fn test_parse_input() {
        use Letter::*;
        let grid = &*INPUT;
        let row = grid.row_iter(0).cloned().collect::<Vec<_>>();

        assert_eq!(row, vec![M, M, M, S, X, X, M, A, S, M]);
    }

    #[test]
    fn test_part_one() {
        let grid = &*INPUT;

        assert_eq!(part_one(&grid), 18);
    }
}
