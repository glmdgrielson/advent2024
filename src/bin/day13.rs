//! # Day 13: Claw Contraption
//!
//! We're playing with a claw machine! And this one
//! isn't rigged like the usual ones! It's _worse!_

use std::fs::read_to_string;
use std::sync::LazyLock;

use advent2024::AdventError;

use regex::Regex;

const OFFSET: i64 = 10_000_000_000_000;

static MACHINE_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"Button A: X\+(\d+), Y\+(\d+)\nButton B: X\+(\d+), Y\+(\d+)\nPrize: X=(\d+), Y=(\d+)",
    )
    .expect("Regex should be valid")
});

#[derive(Clone, Debug, PartialEq, Eq)]
struct Machine {
    alpha: (i64, i64),
    bravo: (i64, i64),
    prize: (i64, i64),
}

impl Machine {
    /// Attempt to get the prize in this machine.
    ///
    /// If [`Option::Some`] is returned, it corresponds
    /// to the number of times each button needs to be
    /// pressed, with the first element being Button A
    /// and the second element being Button B.
    ///
    /// If [`Option::None`] is returned, this machine
    /// cannot return a prize at all! What a ripoff!
    fn get_prize(&self) -> Option<(i64, i64)> {
        let determinant = self.alpha.0 * self.bravo.1 - self.alpha.1 * self.bravo.0;

        let a_presses = (self.prize.0 * self.bravo.1 - self.prize.1 * self.bravo.0) / determinant;
        let b_presses = (self.prize.1 * self.alpha.0 - self.prize.0 * self.alpha.1) / determinant;

        let x = self.alpha.0 * a_presses + self.bravo.0 * b_presses;
        let y = self.alpha.1 * a_presses + self.bravo.1 * b_presses;

        if self.prize.0 == x && self.prize.1 == y {
            Some((a_presses, b_presses))
        } else {
            None
        }
    }

    fn get_offset_prize(&self) -> Option<(i64, i64)> {
        let prize = (self.prize.0 + OFFSET, self.prize.1 + OFFSET);
        let determinant = self.alpha.0 * self.bravo.1 - self.alpha.1 * self.bravo.0;

        let a_presses = (prize.0 * self.bravo.1 - prize.1 * self.bravo.0) / determinant;
        let b_presses = (prize.1 * self.alpha.0 - prize.0 * self.alpha.1) / determinant;

        let x = self.alpha.0 * a_presses + self.bravo.0 * b_presses;
        let y = self.alpha.1 * a_presses + self.bravo.1 * b_presses;

        if prize.0 == x && prize.1 == y {
            Some((a_presses, b_presses))
        } else {
            None
        }
    }
}

fn parse_input(file: &str) -> Result<Vec<Machine>, AdventError> {
    let machines = file.split("\n\n");
    machines
        .map(|machine| {
            let Some(captures) = MACHINE_RE.captures(machine) else {
                return Err(AdventError::Parse(format!("Invalid syntax: {}", machine)));
            };

            let Some(alpha_x) = captures.get(1) else {
                return Err(AdventError::Parse(format!("Invalid Button A: {}", machine)));
            };
            let alpha_x = alpha_x
                .as_str()
                .parse::<i64>()
                .map_err(|err| AdventError::Parse(format!("Bad coordinate: {}", err)))?;

            let Some(alpha_y) = captures.get(2) else {
                return Err(AdventError::Parse(format!("Invalid Button A: {}", machine)));
            };
            let alpha_y = alpha_y
                .as_str()
                .parse::<i64>()
                .map_err(|err| AdventError::Parse(format!("Bad coordinate: {}", err)))?;

            let Some(bravo_x) = captures.get(3) else {
                return Err(AdventError::Parse(format!("Invalid Button B: {}", machine)));
            };
            let bravo_x = bravo_x
                .as_str()
                .parse::<i64>()
                .map_err(|err| AdventError::Parse(format!("Bad coordinate: {}", err)))?;

            let Some(bravo_y) = captures.get(4) else {
                return Err(AdventError::Parse(format!("Invalid Button B: {}", machine)));
            };
            let bravo_y = bravo_y
                .as_str()
                .parse::<i64>()
                .map_err(|err| AdventError::Parse(format!("Bad coordinate: {}", err)))?;

            let Some(prize_x) = captures.get(5) else {
                return Err(AdventError::Parse(format!("Invalid prize: {}", machine)));
            };
            let prize_x = prize_x
                .as_str()
                .parse::<i64>()
                .map_err(|err| AdventError::Parse(format!("Bad coordinate: {}", err)))?;

            let Some(prize_y) = captures.get(6) else {
                return Err(AdventError::Parse(format!("Invalid prize: {}", machine)));
            };
            let prize_y = prize_y
                .as_str()
                .parse::<i64>()
                .map_err(|err| AdventError::Parse(format!("Bad coordinate: {}", err)))?;

            Ok(Machine {
                alpha: (alpha_x, alpha_y),
                bravo: (bravo_x, bravo_y),
                prize: (prize_x, prize_y),
            })
        })
        .collect()
}

/// Get the minimum number of tokens required to get the maximum number of possible prizes.
fn part_one(data: &[Machine]) -> i64 {
    data.iter()
        .filter_map(|machine| machine.get_prize())
        .map(|(a, b)| a * 3 + b)
        .sum()
}

fn part_two(data: &[Machine]) -> i64 {
    data.iter()
        .filter_map(|machine| machine.get_offset_prize())
        .map(|(a, b)| a * 3 + b)
        .sum()
}

fn main() -> Result<(), AdventError> {
    let file = read_to_string("src/input/day13.txt")?;
    let data = parse_input(&file)?;

    println!(
        "The number of tokens to get the most prizes is {}",
        part_one(&data)
    );
    println!("The tokens needed to get the offset prizes are {}", part_two(&data));
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    use std::sync::LazyLock;

    static INPUT: LazyLock<Vec<Machine>> = LazyLock::new(|| {
        let file = read_to_string("src/input/day13-test.txt").unwrap();

        parse_input(&file).unwrap()
    });

    #[test]
    fn test_parse_input() {
        let data = &*INPUT;

        assert_eq!(
            data[0],
            Machine {
                alpha: (94, 34),
                bravo: (22, 67),
                prize: (8400, 5400),
            }
        );
    }

    #[test]
    fn test_get_prize() {
        let data = &*INPUT;

        assert!(data[0].get_prize().is_some_and(|(a, b)| (a, b) == (80, 40)));
        assert!(data[1].get_prize().is_none());
        assert!(data[2].get_prize().is_some_and(|(a, b)| (a, b) == (38, 86)));
        assert!(data[3].get_prize().is_none());
    }

    #[test]
    fn test_part_one() {
        let data = &*INPUT;

        assert_eq!(part_one(&data), 480);
    }
}
