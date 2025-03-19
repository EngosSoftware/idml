//! # The result and error types

use std::fmt;
use std::fmt::Display;

/// Common result type.
pub type Result<T, E = TokenizingError> = std::result::Result<T, E>;

/// Error definition used by the parser.
#[derive(Debug, PartialEq, Eq)]
pub struct TokenizingError(String);

impl Display for TokenizingError {
  /// Implementation of [Display] trait for [TokenizingError].
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

impl TokenizingError {
  /// Creates a new [TokenizingError] with specified error message.
  pub fn new(message: &str) -> Self {
    Self(message.to_string())
  }
}

/// Reports an empty input.
pub fn err_empty_input() -> TokenizingError {
  TokenizingError::new("empty input")
}

/// Reports an unexpected character on input.
pub fn err_unexpected_character(ch: char, row: usize, col: usize) -> TokenizingError {
  TokenizingError::new(&format!("unexpected character: '{ch}' 0x{:X} at row {row} and column {col}", ch as usize))
}

/// Reports an unexpected end of input.
pub fn err_unexpected_end() -> TokenizingError {
  TokenizingError::new("unexpected end of input")
}
