//! Day 03: Mull It Over
//!
//! The shopkeeper at the North Pole Toboggan Rental Shop is having
//! a few issues. Their computer is having a real bad time dealing
//! with bad data and it's up to us to fix it.

#![warn(clippy::all)]
#![allow(dead_code)]

use advent2024::AdventError;
use std::fs::read_to_string;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Multiply(u32, u32);

impl Multiply {
    /// Perform the multiplication implied by this instruction.
    fn compute(&self) -> u32 {
        self.0 * self.1
    }
}

/// Input consists of a chunk of memory.
///
/// For right now, we only care about multiply instructions,
/// which take the form of `mul(X,Y)` where X and Y are
/// numbers to be multiplied together.
fn parse_input(file: &str) -> Vec<Multiply> {

    // Split the string by the instruction's prefix.
    file.split("mul(").filter_map(|mem| {
        let Some((one, trail)) = mem.split_once(",") else {
            return None;
        };

        let Ok(one) = one.parse::<u32>() else {
            return None;
        };

        let Some((two, _)) = trail.split_once(")") else {
            return None;
        };

        match two.parse::<u32>() {
            Ok(two) => Some(Multiply(one, two)),
            Err(_) => None,
        }
    }).collect()
    // loop {
    //     let Some(index) = memory.find("mul(") else {
    //         // There are no more instructions in the string
    //         // so we stop here.
    //         break;
    //     };
    //
    //     // Remove `mul(` prefix.
    //     memory = &memory[index + 4..];
    //
    //     // Try to get the first number by itself.
    //     let Some((one, trail)) = memory.split_once(",") else {
    //         // The comma is missing, so
    //         // there can't possibly be any
    //         // more valid instructions.
    //         break;
    //     };
    //     memory = trail;
    //
    //     // Check that we actually _have_ the first number.
    //     let Ok(one) = one.parse::<u32>() else {
    //         // Syntax error, oh well, let's move on.
    //         continue;
    //     };
    //
    //     let Some((two, trail)) = memory.split_once(")") else {
    //         // We couldn't find the close paren
    //         // so there can't be any more valid
    //         // instructions. Break the loop.
    //         break;
    //     };
    //     memory = trail;
    //
    //     let Ok(two) = two.parse::<u32>() else {
    //         // Syntax error, oh well, try again.
    //         continue;
    //     };
    //
    //     muls.push(Multiply(one, two));
    // }
}

fn part_one(data: &[Multiply]) -> u32 {
    data.iter().map(|mul| mul.compute()).sum()
}

fn main() -> Result<(), AdventError> {
    let file = read_to_string("src/input/day03.txt")?;
    let data = parse_input(&file);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use std::sync::LazyLock;

    static INPUT: LazyLock<Vec<Multiply>> = LazyLock::new(|| {
        let file = read_to_string("src/input/day03-test.txt").unwrap();

        parse_input(&file)
    });

    #[test]
    fn test_parse_input() {
        let muls = &*INPUT;

        println!("{:?}", muls);
        assert_eq!(muls.len(), 4);
        assert_eq!(muls[0], Multiply(2, 4));
    }
}
