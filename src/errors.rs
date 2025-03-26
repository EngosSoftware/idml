//! # Errors implementation

use std::fmt;
use std::fmt::Display;

/// Common result type.
pub type Result<T, E = DmmError> = std::result::Result<T, E>;

/// Error definition used by the parser.
#[derive(Debug, PartialEq, Eq)]
pub struct DmmError(String);

impl Display for DmmError {
  /// Implementation of [Display] trait for [DmmError].
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

impl DmmError {
  /// Creates a new [DmmError] with specified error message.
  pub fn new(message: &str) -> Self {
    Self(message.to_string())
  }
}

/// Reports an empty input.
pub fn err_empty_input() -> DmmError {
  DmmError::new("empty input")
}

/// Reports an unexpected character on input.
pub fn err_unexpected_character(ch: char, row: usize, col: usize) -> DmmError {
  DmmError::new(&format!("unexpected character: '{ch}' 0x{:X} at row {row} and column {col}", ch as usize))
}

/// Reports an unexpected end of input.
pub fn err_unexpected_end() -> DmmError {
  DmmError::new("unexpected end of input")
}

/// Reports expected node name token.
pub fn err_expected_node_name() -> DmmError {
  DmmError::new("expected node name token")
}

/// Reports expected node content token.
pub fn err_expected_node_content() -> DmmError {
  DmmError::new("expected node content token")
}

/// Reports expected indentation token.
pub fn err_expected_indentation() -> DmmError {
  DmmError::new("expected indentation token")
}

/// Reports malformed indentation.
pub fn err_malformed_indentation(indent: usize, multiplier: usize) -> DmmError {
  DmmError::new(&format!("malformed indentation {indent}, expected multiplication of {multiplier}"))
}
