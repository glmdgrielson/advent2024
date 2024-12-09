//! Day 09: Disk Fragmenter
//!
//! We need to do some file system clean up here. If only the
//! computer didn't have the most arcane diagnostic information
//! this side of the `tar` command...

use std::fs::read_to_string;
use std::iter::repeat_n;

use advent2024::AdventError;
use itertools::Itertools;

type Space = Option<usize>;

/// Input consists of a series of digits, representing
/// the disk map of this bizarre computer setup.
fn parse_input(file: &str) -> Result<Vec<Space>, AdventError> {
    let chars = file.trim().chars().collect::<Vec<_>>();
    chars
        .chunks(2)
        .enumerate()
        .map(|(idx, pair)| {
            let one = pair[0];
            let two = pair.get(1).unwrap_or(&'0');

            let one = one
                .to_digit(10)
                .ok_or_else(|| AdventError::Parse(format!("Invalid digit {}", one)))?;
            let one = repeat_n(Some(idx), one as usize);

            let two = two
                .to_digit(10)
                .ok_or_else(|| AdventError::Parse(format!("Invalid digit {}", two)))?;
            let two = repeat_n(None, two as usize);

            // Ok((0..one).map(|_| Some(idx)).chain((0..two).m))
            Ok(one.chain(two))
        })
        .flatten_ok()
        .collect()
}

/// Find the checksum of the filesystem.
///
/// First, defragment the computer by removing as much
/// space as possible.
fn part_one(data: &[Space]) -> usize {
    let mut disk = data.to_vec();
    let mut start = 0;
    let mut end = data.len() - 1;

    while start < end {
        while disk[start].is_some() {
            start += 1;
        }
        while disk[end].is_none() {
            end -= 1;
        }
        if start < end {
            disk.swap(start, end);
        }
        start += 1;
        end -= 1;
    }

    disk.iter()
        .enumerate()
        .filter_map(|(idx, file)| file.map(|id| id * idx))
        .sum()
}

fn main() -> Result<(), AdventError> {
    let file = read_to_string("src/input/day09.txt")?;
    let data = parse_input(&file)?;

    println!("Disk checksum is {}", part_one(&data));
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    use std::sync::LazyLock;

    static INPUT: LazyLock<Vec<Space>> = LazyLock::new(|| {
        let file = read_to_string("src/input/day09-test.txt").unwrap();

        parse_input(&file).unwrap()
    });

    #[test]
    fn test_parse_input() {
        let data = &*INPUT;

        assert_eq!(data[0], Some(0));
        assert_eq!(data[1], Some(0));
        assert_eq!(data[2], None);
        assert!(data[3].is_none());
    }

    #[test]
    fn test_part_one() {
        let data = &*INPUT;

        assert_eq!(part_one(&data), 1928);
    }
}
