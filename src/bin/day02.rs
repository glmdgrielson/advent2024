//! # Day 02: Red-Nosed Reports
//!
//! We're searching the \*checks notes\* _nuclear fusion/fission_
//! plant of the North Pole today! Apparently they've been getting
//! some odd readings lately, so we might as well try to help.

#![warn(clippy::all)]

use advent2024::AdventError;
use std::fs::read_to_string;

#[derive(Clone, PartialEq, Eq, Debug)]
struct Report(Vec<u32>);

// This impl block basically exists to make sure
// I can test both of the conditions for part one's puzzle.
impl Report {
    /// Check if this report is sorted,
    /// in either ascending or descending order.
    fn is_sorted(&self) -> bool {
        self.0.is_sorted_by(|a, b| a <= b) || self.0.is_sorted_by(|a, b| b <= a)
    }

    /// Check that the values in this report don't change by
    /// less than 1 or more than 3.
    fn has_safe_delta(&self) -> bool {
        self.0
            // Get pairs of numbers. (This iterates over slices
            // and not tuples, which I assume is for memory
            // safety reasons, but is still a bit annoying.)
            .windows(2)
            // Get the change in value for each pair.
            .map(|pair| pair[0].abs_diff(pair[1]))
            // Test that every pair changed by a safe value.
            .all(|diff| (1..=3).contains(&diff))
    }
}

/// Puzzle input consists of a series of reports. A report in this
/// context is defined as a series of numbers separated by spaces.
fn parse_input(text: &str) -> Result<Vec<Report>, AdventError> {
    text.lines()
        .map(|line| {
            let report = line
                .split_whitespace()
                .map(|num| {
                    num.parse::<u32>()
                        .map_err(|err| AdventError::Parse(format!("ParseIntError: {}", err)))
                })
                .collect::<Result<Vec<u32>, AdventError>>();

            report.map(Report)
        })
        .collect::<Result<Vec<_>, AdventError>>()
}

/// Find the number of safe reports.
///
/// A report is defined as safe if both of the following
/// conditions hold true:
/// 1. All of the numbers are sorted in either ascending
///    descending order.
/// 2. No number changes by less than one or more than three.
fn part_one(data: &[Report]) -> usize {
    data.iter()
        // Check condition 1. The `is_sorted_by` function is
        // used so that we can check a reverse ordering.
        .filter(|report| report.is_sorted())
        // Check condition 2.
        .filter(|report| report.has_safe_delta())
        .count()
}

/// Find the number of reports that are safe with the Problem Dampener.
///
/// The definition of safe remains the same, but now there's a new
/// wrinkle: we can remove one item from a report and still have it
/// qualify as safe.
fn part_two(data: &[Report]) -> usize {
    let (passed, failed): (Vec<_>, Vec<_>) = data
        .iter()
        .partition(|report| report.is_sorted() && report.has_safe_delta());

    let failed = failed
        .into_iter()
        .filter(|&report| {
            report.0.iter().enumerate().any(|(idx, _)| {
                let mut items = report.0.clone();
                items.remove(idx);
                let report = Report(items);

                report.is_sorted() && report.has_safe_delta()
            })
        })
        .count();

    passed.len() + failed
}

fn main() -> Result<(), AdventError> {
    let file = read_to_string("src/input/day02.txt")?;
    let data = parse_input(&file)?;

    println!("The number of safe reports is {}", part_one(&data));
    println!(
        "The number of safe reports after dampening is {}",
        part_two(&data)
    );

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    use std::sync::LazyLock;

    /// The test input, owned in a manner such that it only needs
    /// to be built once. I _think_ this is worth it?
    static INPUT: LazyLock<Vec<Report>> = LazyLock::new(|| {
        let file = read_to_string("src/input/day02-test.txt").unwrap();

        parse_input(&file).unwrap()
    });

    #[test]
    fn test_parse_input() {
        let reports = &*INPUT;

        assert_eq!(reports[0], Report(vec![7, 6, 4, 2, 1]));
    }

    #[test]
    fn test_is_sorted() {
        let reports = &*INPUT;

        assert!(reports[0].is_sorted());
        assert!(!reports[3].is_sorted());
    }

    #[test]
    fn test_has_safe_delta() {
        let reports = &*INPUT;

        assert!(reports[0].has_safe_delta());
        assert!(!reports[1].has_safe_delta());
    }

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(&*INPUT), 2);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(&INPUT), 4);
    }
}
