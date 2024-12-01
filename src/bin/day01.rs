//! # Day 01: Historian Hysteria
//!
//! The Chief Historian has gone missing! We don't know where he is,
//! but we have a list of places that might be good to check. Small
//! problem, the list is a mess.
//!
//! Are you a good enough elf to save the Historian?

use advent2024::AdventError;
use std::fs::read_to_string;

#[derive(Debug, PartialEq, Eq)]
struct Lists(Vec<(u32, u32)>);

/// Input consists of a list of numbers. Each row contains two
/// numbers separated by whitespace.
fn parse_input(text: &str) -> Result<Lists, AdventError> {
    let lines: Vec<(u32, u32)> = text
        .lines()
        .map(|line| {
            let mut numbers = line.split_ascii_whitespace();
            let one = numbers.next();
            let two = numbers.next();

            (one, two)
        })
        .filter_map(|pair| match pair {
            (Some(one), Some(two)) => Some((one.to_string(), two.to_string())),
            _ => None,
        })
        .map(|(one, two)| {
            let one = one
                .parse::<u32>()
                .map_err(|err| AdventError::Parse(format!("ParseIntError: {}", err)));
            // (one.parse::<u32>(), two.parse::<u32>())
            let two = two
                .parse::<u32>()
                .map_err(|err| AdventError::Parse(format!("ParseIntError: {}", err)));
            match (one, two) {
                (Ok(one), Ok(two)) => Ok((one, two)),
                (Err(one), _) => Err(one),
                (Ok(_), Err(two)) => Err(two),
            }
        })
        .collect::<Result<Vec<_>, AdventError>>()?;
    Ok(Lists(lines))
}

/// Find the total distance between the two lists.
///
/// Distance is calculated by finding the distance between
/// the order of the values. That is, the distance between
/// the smallest value in both lists plus the distance between
/// the second smallest value in both lists, all the way up to
/// the distance between the largest values in both lists.
fn part_one(data: &Lists) -> u32 {
    let (one, two): (Vec<_>, Vec<_>) = data.0.clone().into_iter().unzip();
    let mut one: Vec<_> = one.clone();
    let mut two: Vec<_> = two.clone();

    // Sort both collections.
    one.sort();
    two.sort();

    one.into_iter()
        .zip(two)
        .map(|(one, two)| one.abs_diff(two))
        .sum()
}

/// Find the similarity score.
///
/// The similarity score is measured as the number of times a
/// value in list 1 appears in list 2.
fn part_two(data: &Lists) -> usize {
    let (one, two): (Vec<_>, Vec<_>) = data.0.clone().into_iter().unzip();

    one.into_iter()
        .map(|num| {
            let count = two.iter().filter(|&item| num == *item).count();

            count * (num as usize)
        })
        .sum()
}

fn main() -> Result<(), AdventError> {
    let file = read_to_string("src/input/day01.txt")?;
    let data = parse_input(&file)?;

    println!("The total distance is {}", part_one(&data));
    println!("The similarity score is {}", part_two(&data));
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[cfg(test)]
    fn load_input() -> String {
        let file = read_to_string("src/input/day01-test.txt");

        file.expect("Could not load test data file")
    }

    #[test]
    fn test_parse_input() {
        let file = load_input();
        let data = parse_input(&file);
        let data = data.unwrap();

        assert_eq!(data.0[0], (3, 4));
    }

    #[test]
    fn test_part_one() {
        let file = parse_input(&load_input());
        let data = file.unwrap();

        let value = part_one(&data);

        assert_eq!(value, 11);
    }

    #[test]
    fn test_part_two() {
        let file = parse_input(&load_input());
        let data = file.unwrap();

        let value = part_two(&data);

        assert_eq!(value, 31);
    }
}
