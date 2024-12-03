//! Day 03: Mull It Over
//!
//! The shopkeeper at the North Pole Toboggan Rental Shop is having
//! a few issues. Their computer is having a real bad time dealing
//! with bad data and it's up to us to fix it.

#![warn(clippy::all)]

use advent2024::AdventError;

use std::fs::read_to_string;
use std::sync::LazyLock;

use regex::Regex;

static INSTRUCTION_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"do\(\)|don't\(\)|mul\(\d+,\d+\)").expect("Invalid regex found")
});

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// The instructions the machine can perform.
enum Instruction {
    Multiply(u32, u32),
    Enable,
    Disable,
}

impl Instruction {
    /// Perform the multiplication implied by this instruction.
    fn compute(&self) -> Option<u32> {
        match self {
            Instruction::Multiply(one, two) => Some(one * two),
            _ => None,
        }
    }
}

/// Input consists of a chunk of memory.
///
/// The main focus of this puzzle is multiply instructions,
/// which take the form of `mul(X,Y)` where X and Y are
/// numbers to be multiplied together.
///
/// Important for part two are control flow instructions,
/// which in this case are `do()` and `don't()`.
///
/// Syntax errors are to be expected, as this data is
/// corrupted. As such we shouldn't fail when we see
/// something we don't expect.
fn parse_input(file: &str) -> Vec<Instruction> {
    INSTRUCTION_RE
        .find_iter(file)
        .map(|m| m.as_str())
        .map(|code| match code {
            "do()" => Instruction::Enable,
            "don't()" => Instruction::Disable,
            mul => {
                let Some(arg) = mul.strip_prefix("mul(") else {
                    unreachable!("Invalid regext match {}", mul);
                };
                let Some((one, trail)) = arg.split_once(",") else {
                    unreachable!("Invalid regex match {}", mul);
                };

                let Ok(one) = one.parse::<u32>() else {
                    unreachable!("Invalid regex match {}", mul);
                };

                let Some(two) = trail.strip_suffix(")") else {
                    unreachable!("Invalid regex match {}", mul);
                };

                let Ok(two) = two.parse::<u32>() else {
                    unreachable!("Invalid regex match {}", mul);
                };

                Instruction::Multiply(one, two)
            }
        })
        .collect()
}

/// Find the sum of the results of all of the multiply instructions.
fn part_one(data: &[Instruction]) -> u32 {
    data.iter().filter_map(|mul| mul.compute()).sum()
}

/// Find the sume of the enabled multiplications.
///
/// We can't just pay attention to the `mul` instructions,
/// because we have to keep track of the state of the
/// machine. `do()` and `don't()` respectively enable and
/// disable the machine's ability to do multiplications.
fn part_two(data: &[Instruction]) -> u32 {
    let (_, out) = data
        .iter()
        .fold((true, 0), |(power, acc), &code| match code {
            Instruction::Multiply(one, two) => match power {
                true => (power, acc + one * two),
                false => (power, acc),
            },
            Instruction::Enable => (true, acc),
            Instruction::Disable => (false, acc),
        });

    out
}

fn main() -> Result<(), AdventError> {
    let file = read_to_string("src/input/day03.txt")?;
    let data = parse_input(&file);

    println!(
        "The sum of the multiply instructions is {}",
        part_one(&data)
    );
    print!(
        "The sum of the enabled multiply instructions is {}",
        part_two(&data)
    );
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use std::sync::LazyLock;

    static INPUT: LazyLock<Vec<Instruction>> = LazyLock::new(|| {
        let file = read_to_string("src/input/day03-test.txt").unwrap();

        parse_input(&file)
    });

    #[test]
    fn test_parse_input() {
        let muls = &*INPUT;

        println!("{:?}", muls);
        assert_eq!(muls.len(), 4);
        assert_eq!(muls[0], Instruction::Multiply(2, 4));
    }

    #[test]
    fn test_part_one() {
        let muls = &*INPUT;

        assert_eq!(part_one(&muls), 161);
    }

    #[test]
    fn test_part_two() {
        let file = read_to_string("src/input/day03-test2.txt").unwrap();
        let codes = parse_input(&file);

        assert_eq!(part_two(&codes), 48);
    }
}
