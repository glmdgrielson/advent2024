//! # Day 07: Bridge Repair
//!
//! Some dang elephants have been messing with the local
//! engineers and now they can't fix the bridge! We need to
//! do something about it so we can get back to work!

use std::collections::HashSet;
use std::fs::read_to_string;

use advent2024::AdventError;

#[derive(Clone, Debug, PartialEq, Eq)]
struct Equation {
    total: u64,
    operands: Vec<u64>,
}

/// Input consists of a series of equations.
///
/// An equation consists of a numeric total,
/// followed by a colon, and a list of numbers
/// to be used as operands.
fn parse_input(file: &str) -> Result<Vec<Equation>, AdventError> {
    file.lines()
        .map(|line| {
            let numbers = line.split_whitespace().collect::<Vec<_>>();
            let total = numbers[0];
            let operands = &numbers[1..];

            let Some(total) = total.strip_suffix(":") else {
                return Err(AdventError::Parse("Total missing suffix".to_string()));
            };

            let total = total
                .parse::<u64>()
                .map_err(|err| AdventError::Parse(format!("Error parsing total: {}", err)))?;

            let operands = operands
                .into_iter()
                .map(|oper| {
                    oper.parse::<u64>().map_err(|err| {
                        AdventError::Parse(format!("Error parsing operand: {}", err))
                    })
                })
                .collect::<Result<Vec<_>, AdventError>>()?;

            Ok(Equation { total, operands })
        })
        .collect::<Result<Vec<Equation>, _>>()
}

/// Find the sum of all of the valid equations.
///
/// An equation is valid if it can be formed through
/// a series of multiplications or additions. Note that
/// there is no order of operations here; all operations
/// happen in a strictly left to right order.
fn part_one(data: &[Equation]) -> u64 {
    data.iter()
        .filter(|eq| {
            let mut totals = vec![eq.operands[0]];
            for &oper in &eq.operands[1..] {
                totals = totals
                    .into_iter()
                    .flat_map(|total| [total + oper, total * oper])
                    .collect();
            }

            totals.contains(&eq.total)
        })
        .map(|eq| eq.total)
        .sum()
}

fn main() -> Result<(), AdventError> {
    let file = read_to_string("src/input/day07.txt")?;
    let data = parse_input(&file)?;

    println!("Sum of possible equations is {}", part_one(&data));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::sync::LazyLock;

    static INPUT: LazyLock<Vec<Equation>> = LazyLock::new(|| {
        let file = read_to_string("src/input/day07-test.txt").unwrap();

        parse_input(&file).unwrap()
    });

    #[test]
    fn test_parse_input() {
        let data = &*INPUT;

        assert_eq!(
            data[0],
            Equation {
                total: 190,
                operands: vec![10, 19]
            }
        );
    }

    #[test]
    fn test_part_one() {
        let data = &*INPUT;

        assert_eq!(part_one(&data), 3749);
    }
}
