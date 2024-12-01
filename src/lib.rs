//! Helper systems for Advent of Code puzzles.

use thiserror::Error;
use std::io::Error as IOError;

#[derive(Error, Debug)]
pub enum AdventError {
    #[error("Error reading file: {0}")]
    File(#[from] IOError),
    #[error("Parsing error: {0}")]
    Parse(String),
}
