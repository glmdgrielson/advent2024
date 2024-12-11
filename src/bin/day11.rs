//! Day 11: Plutonian Pebbles
//!
//! We're off to Pluto to inspect some weird rocks!

use std::fs::read_to_string;
use std::collections::HashMap;

use advent2024::AdventError;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Stone(u64);

impl Stone {
    fn blink(&self) -> Vec<Stone> {
        if self.0 == 0 {
            vec![Stone(1)]
        } else if self.0.ilog10() % 2 == 1 {
            let power = (self.0.ilog10() + 1) / 2;
            let power = 10u64.pow(power);
            // let (div, rem) = self.0.divm
            vec![Stone(self.0 / power), Stone(self.0 % power)]
        } else {
            vec![Stone(self.0 * 2024)]
        }
    }
}

/// Input consists of a series of numbers, separated by spaces.
fn parse_input(file: &str) -> Result<Vec<Stone>, AdventError> {
    file.split_whitespace()
        .map(|stone| {
            stone
                .parse::<u64>()
                .map(Stone)
                .map_err(|err| AdventError::Parse(format!("Invalid stone {}", err)))
        })
        .collect()
}

/// Count the number of stones after 25 blinks.
fn part_one(data: &[Stone]) -> usize {
    let stones = data.to_vec();

    (0..25).fold(stones, |stones, _| stones.iter().flat_map(Stone::blink).collect()).len()

}

/// Count the number of stones after 75 blinks.
///
/// This is the inevitable "learn to memoize" challenge, wherein the part
/// two ups a limit to the point of being nigh unusable unless you cheat.
fn part_two(data: &[Stone]) -> usize {
    let stones = data.to_vec();
    // This keeps count of how many times a specific number is on a stone.
    let mut stone_map = stones.iter().map(|&stone| (stone, 1)).collect::<HashMap<_, usize>>();

    for _ in 0..75 {
        let mut new_map = HashMap::new();
        for (stone, count) in stone_map.iter() {
            stone.blink().iter().for_each(|&stone| *new_map.entry(stone).or_insert(0) += count);
        }

        stone_map = new_map;
    }

    stone_map.values().sum()
}

fn main() -> Result<(), AdventError> {
    let file = read_to_string("src/input/day11.txt")?;
    let data = parse_input(&file)?;

    println!("{} stones exist after 25 blinks", part_one(&data));
    println!("{} stones exist after 75 blinks", part_two(&data));
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_blink() {
        assert_eq!(Stone(0).blink(), vec![Stone(1)]);

        assert_eq!(Stone(1).blink(), vec![Stone(2024)]);

        assert_eq!(Stone(10).blink(), vec![Stone(1), Stone(0)]);
    }

    #[test]
    fn test_part_one() {
        let data = [Stone(125), Stone(17)];

        assert_eq!(part_one(&data), 55312);
    }
}
