//! Day 09: Disk Fragmenter
//!
//! We need to do some file system clean up here. If only the
//! computer didn't have the most arcane diagnostic information
//! this side of the `tar` command...

use std::fs::read_to_string;
use std::iter::repeat_n;

use advent2024::AdventError;
use itertools::Itertools;

// type Space = Option<usize>;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Space {
    size: usize,
    index: Option<usize>,
}

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
            let one = one as usize;

            let two = two
                .to_digit(10)
                .ok_or_else(|| AdventError::Parse(format!("Invalid digit {}", two)))?;
            let two = two as usize;

            // Ok((0..one).map(|_| Some(idx)).chain((0..two).m))
            Ok([
                Space {
                    size: one,
                    index: Some(idx),
                },
                Space {
                    size: two,
                    index: None,
                },
            ])
        })
        .flatten_ok()
        .collect()
}

/// Find the checksum of the filesystem.
///
/// First, defragment the computer by removing as much
/// space as possible.
fn part_one(data: &[Space]) -> usize {
    let disk = defragment(data);

    disk.iter()
        .enumerate()
        .filter_map(|(idx, file)| file.map(|id| id * idx))
        .sum()
}

fn defragment(data: &[Space]) -> Vec<Option<usize>> {
    let mut disk = data
        .iter()
        .flat_map(|file| repeat_n(file.index, file.size))
        .collect::<Vec<_>>();
    let mut start = 0;
    let mut end = disk.len() - 1;

    while start < end {
        // Get the first free space.
        while disk[start].is_some() {
            start += 1;
        }

        // Get the last file
        while disk[end].is_none() {
            end -= 1;
        }

        // Check that we're not using space AFTER this file.
        if start < end {
            // Use the space.
            disk.swap(start, end);
        }

        // Increment our counters.
        start += 1;
        end -= 1;
    }
    disk
}

/// Find the checksum of the defragmented filesystem.
fn part_two(data: &[Space]) -> usize {
    let disk = clean_space(data);

    disk.iter()
        .enumerate()
        .filter_map(|(idx, file)| file.index.map(|id| id * idx))
        .sum()
}

fn clean_space(data: &[Space]) -> Vec<Space> {
    let mut disk = data.to_vec();
    let mut start = 0;
    let mut end = disk.len() - 1;
    let mut changed = true;

    loop {
        if !changed {
            break;
        }
        // Find the first space.
        while disk[start].index.is_some() {
            start += 1;
        }

        let end = disk.iter().rposition(|file| file.size <= disk[start].size);
        if let Some(end) = end {
            if start > end {
                changed = false;
                continue;
            } else {
                disk[start].size -= disk[end].size;
                let file = disk.remove(end);
                disk.insert(start, file);
                changed = true;
            }
        }

        start += 1;
        if disk.get(start).is_none() {
            start = 0;
        }
    }

    disk
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
        let data = parse_input("12345").unwrap();

        assert_eq!(
            data,
            vec![
                Space {
                    size: 1,
                    index: Some(0)
                },
                Space {
                    size: 2,
                    index: None
                },
                Space {
                    size: 3,
                    index: Some(1)
                },
                Space {
                    size: 4,
                    index: None
                },
                Space {
                    size: 5,
                    index: Some(2)
                },
                Space {
                    size: 0,
                    index: None
                },
            ]
        );
    }

    #[test]
    fn test_part_one() {
        let data = &*INPUT;

        assert_eq!(part_one(&data), 1928);
    }

    #[test]
    fn test_defragment() {
        let data = &*INPUT;

        let disk = defragment(&data)
            .iter()
            .filter_map(|file| *file)
            .collect::<Vec<_>>();
        assert_eq!(
            disk,
            vec![
                0, 0, 9, 9, 8, 1, 1, 1, 8, 8, 8, 2, 7, 7, 7, 3, 3, 3, 6, 4, 4, 6, 5, 5, 5, 5, 6, 6
            ]
        )
    }

    #[test]
    fn test_clean_space() {
        let data = &*INPUT;

        let disk = clean_space(&data);

        let disk = disk
            .iter()
            .filter_map(|file| file.index)
            .collect::<Vec<_>>();

        assert_eq!(disk, vec![0, 9, 2, 1, 7, 4, 3, 5, 6, 8]);
    }
}
