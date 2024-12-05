//! Helper systems for Advent of Code puzzles.

use thiserror::Error;
use std::io::Error as IOError;

#[derive(Error, Debug)]
pub enum AdventError {
    #[error("Error reading file: {0}")]
    /// An error reading the file used as input.
    File(#[from] IOError),
    /// An error in the format of the input file.
    #[error("Parsing error: {0}")]
    Parse(String),
}
