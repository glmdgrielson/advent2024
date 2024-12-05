//! Day 05: Print Queue
//!
//! We need to help the elves figure out
//! their printing situation. They have lots
//! of pages, lots of rules, and not enough
//! time to figure it out manually!

use advent2024::AdventError;
use std::cmp::Ordering;
use std::fs::read_to_string;

#[derive(Debug, Clone, PartialEq, Eq)]
struct PageOrdering(Vec<(u32, u32)>);

impl PageOrdering {
    /// Check to see whether or not `one` and `two`
    /// are considered sorted by this set of rules.
    ///
    /// Given that we don't care about numbers not
    /// constrained by the rules, we can simply check
    /// for a rule that is directly contradicted.
    fn compare(&self, one: u32, two: u32) -> bool {
        !self.0.contains(&(two, one))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Update(Vec<u32>);

impl Update {
    /// Get the median page of this update.
    fn median_page(&self) -> u32 {
        self.0[self.0.len() / 2]
    }
}

/// Input consists of a list of page ordering rules
/// and series of pages.
///
/// A page ordering rule consists of two numbers
/// separated by a bar: `X|Y`.
///
/// A series of pages consists of a series of
/// comma separated numbers.
fn parse_input(file: &str) -> Result<(PageOrdering, Vec<Update>), AdventError> {
    let (rules, update) = file.split_once("\n\n").ok_or(AdventError::Parse(
        "Expected two-part data format".to_string(),
    ))?;

    let rules = rules
        .lines()
        .map(|line| {
            let (one, two) = line.split_once("|").ok_or(AdventError::Parse(format!(
                "Invalid page ordering rule {}",
                line
            )))?;

            let Ok(one) = one.parse::<u32>() else {
                return Err(AdventError::Parse(format!("Invalid page number {}", one)));
            };
            let Ok(two) = two.parse::<u32>() else {
                return Err(AdventError::Parse(format!("Invalid page number {}", two)));
            };

            Ok((one, two))
        })
        .collect::<Result<Vec<(u32, u32)>, AdventError>>()?;

    let ordering = PageOrdering(rules);

    let updates = update
        .lines()
        .map(|line| {
            let updates = line
                .split(",")
                .map(|num| {
                    num.parse::<u32>()
                        .map_err(|_| AdventError::Parse(format!("Invalid page number {}", num)))
                })
                .collect::<Result<Vec<u32>, AdventError>>()?;

            Ok(Update(updates))
        })
        .collect::<Result<Vec<_>, AdventError>>()?;

    Ok((ordering, updates))
}

/// Find the sum of the median of successful updates.
///
/// An update is successful if its pages are ordered
/// according to the [PageOrdering] given by `rules`.
fn part_one(rules: &PageOrdering, updates: &[Update]) -> u32 {
    updates
        .iter()
        .filter(|up| up.0.is_sorted_by(|a, b| rules.compare(*a, *b)))
        .map(|up| up.median_page())
        .sum()
}

/// Fix the unsuccessful updates and sum their medians.
fn part_two(rules: &PageOrdering, updates: &[Update]) -> u32 {
    let failed = updates
        .iter()
        .filter(|up| !up.0.is_sorted_by(|&a, &b| rules.compare(a, b)));
    failed
        .map(|up| {
            let mut pages = up.0.clone();
            pages.sort_by(|&a, &b| match rules.compare(a, b) {
                true => Ordering::Less,
                false => Ordering::Greater,
            });
            Update(pages)
        })
        .map(|up| up.median_page())
        .sum()
}

fn main() -> Result<(), AdventError> {
    let file = read_to_string("src/input/day05.txt")?;
    let (rules, updates) = parse_input(&file)?;

    println!(
        "The medians of successful updates sum to {}",
        part_one(&rules, &updates)
    );
    println!("The medians of fixed updates sum to {}", part_two(&rules, &updates));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::sync::LazyLock;

    static INPUT: LazyLock<(PageOrdering, Vec<Update>)> = LazyLock::new(|| {
        let file = read_to_string("src/input/day05-test.txt").unwrap();

        parse_input(&file).unwrap()
    });

    #[test]
    fn test_parse_input() {
        let (rules, updates) = &*INPUT;

        assert_eq!(rules.0[0], (47, 53));
        assert_eq!(updates[0].0, vec![75, 47, 61, 53, 29]);
    }

    #[test]
    fn test_part_one() {
        let (rules, updates) = &*INPUT;

        assert_eq!(part_one(rules, updates), 143);
    }

    #[test]
    fn test_part_two() {
        let (rules, updates) = &*INPUT;

        assert_eq!(part_two(rules, updates), 123);
    }
}
