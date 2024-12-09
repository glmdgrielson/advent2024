//! Day 09: Disk Fragmenter
//!
//! We need to do some file system clean up here. If only the
//! computer didn't have the most arcane diagnostic information
//! this side of the `tar` command...

use std::fs::read_to_string;

use advent2024::AdventError;

fn parse_input(file: &str) -> Result<Vec(usize, usize), AdventError> {
    todo!()
}

fn main() -> Result<(), AdventError> {
    let file = read_to_string("src/input/day09.txt");
    let data = parse_input(&file);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    use std::sync::LazyLock;

    static INPUT: LazyLock<Vec(usize, usize)> = LazyLock::new(|| {
        let file = read_to_string("src/input/day09-test.txt").unwrap();

        parse_input(&file).unwrap()
    });

}
