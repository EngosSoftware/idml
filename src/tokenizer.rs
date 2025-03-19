//! # Tokenizer for Decision Modeling Markup input text

use crate::errors::*;
use std::fmt::Write;
use std::iter::Peekable;
use std::str::Chars;

/// Character indicating the start of the node name.
const NODE_START: char = '.';

/// Whitespace character.
const WS: char = ' ';

/// Line feed character.
const LF: char = '\n';

/// Carriage return character.
const CR: char = '\r';

/// Slash character.
const SLASH: char = '/';

/// Asterisk character.
const ASTERISK: char = '*';

/// Empty character, text files do not have zero value characters.
const EMPTY_CHAR: char = '\u{0}';

/// Tokenizes the provided input.
pub fn tokenize(input: &str) -> Result<Vec<Token>> {
  Tokenizer::new(input).tokenize()
}

/// Joins tokens back to the previously tokenized input.
pub fn join_tokens(tokens: Vec<Token>) -> String {
  let mut buffer = String::new();
  for token in tokens {
    match token {
      Token::Indentation(width) => {
        let _ = write!(&mut buffer, "{}", " ".repeat(width));
      }
      Token::NodeName(name) => {
        let _ = write!(&mut buffer, "{NODE_START}{name}");
      }
      Token::NodeContent(content) => {
        let _ = write!(&mut buffer, "{content}");
      }
    }
  }
  buffer
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
  NodeName(String),
  NodeContent(String),
  Indentation(usize),
}

#[derive(Copy, Clone)]
enum State {
  /// At the beginning of the input.
  Start,
  /// At the beginning of the line.
  NewLine,
  /// Inside the indentation at the beginning of the line.
  Indentation,
  /// Inside the node name.
  NodeName,
  /// Inside the node content.
  NodeContent,
  /// Inside a multiline line comment.
  MultiLineComment,
  /// Inside the single line comment.
  SingleLineComment,
}

/// A structure representing the DMM tokenizer.
struct Tokenizer<'a> {
  /// Current row position in processed input.
  row: usize,
  /// Current column position in processed content.
  column: usize,
  /// Current tokenizing state.
  state: State,
  /// Next state after processing comments.
  next_state: State,
  /// Input chars.
  chars: Peekable<Chars<'a>>,
  /// Currently processed character.
  current_char: char,
  /// The previously processed character.
  previous_char: char,
  /// The next character on input.
  next_char: char,
  /// The name of currently processed node.
  node_name: String,
  /// The content of currently processed node.
  node_content: String,
  /// The content of currently processed indentation.
  indentation: String,
  /// Processed tokens.
  tokens: Vec<Token>,
}

impl<'a> Tokenizer<'a> {
  /// Creates a new instance of the tokenizer.
  pub fn new(input: &'a str) -> Self {
    Self {
      row: 1,
      column: 0,
      state: State::Start,
      next_state: State::Start,
      chars: input.chars().peekable(),
      current_char: EMPTY_CHAR,
      previous_char: EMPTY_CHAR,
      next_char: EMPTY_CHAR,
      node_name: "".to_string(),
      node_content: "".to_string(),
      indentation: "".to_string(),
      tokens: vec![],
    }
  }

  /// Tokenizes the input text.
  pub fn tokenize(mut self) -> Result<Vec<Token>> {
    if self.chars.peek().is_none() {
      // Report an empty input and quit.
      return Err(err_empty_input());
    }
    loop {
      // Normalize end-of-line characters.
      (self.current_char, self.next_char) = match (self.next(), self.peek()) {
        (LF, other) => {
          self.row += 1;
          self.column = 0;
          (LF, other)
        }
        (CR, LF) => {
          self.row += 1;
          self.column = 0;
          (self.next(), self.peek())
        }
        (CR, other) => {
          self.row += 1;
          self.column = 0;
          (LF, other)
        }
        (other1, other2) => (other1, other2),
      };
      match self.state {
        State::Start => {
          // Process the beginning of the file.
          match self.context() {
            (_, EMPTY_CHAR, _) => return Err(err_unexpected_end()),
            (_, NODE_START, _) => {
              self.state = State::NodeName;
            }
            (_, SLASH, EMPTY_CHAR) => return Err(err_unexpected_end()),
            (_, SLASH, SLASH) => {
              self.consume_char();
              self.next_state = State::Start;
              self.state = State::SingleLineComment;
            }
            (_, SLASH, ASTERISK) => {
              self.consume_char();
              self.next_state = State::Start;
              self.state = State::MultiLineComment;
            }
            (_, SLASH, other) => return Err(err_unexpected_character(other, self.row, self.column + 1)),
            (_, other, _) => return Err(err_unexpected_character(other, self.row, self.column)),
          }
        }
        State::NewLine => {
          // Decide what to do in the new line.
          match self.context() {
            (_, EMPTY_CHAR, _) => {
              self.consume_node_content();
              break;
            }
            (_, NODE_START, _) => {
              self.consume_node_content();
              self.state = State::NodeName;
            }
            (_, WS, _) => {
              self.indentation.push(WS);
              self.state = State::Indentation;
            }
            (_, LF, _) => {
              self.node_content.push(LF);
            }
            (_, other, _) => {
              self.node_content.push(other);
              self.state = State::NodeContent;
            }
          }
        }
        State::NodeName => {
          // Consume the node name.
          match self.context() {
            (_, EMPTY_CHAR, _) => {
              return Err(err_unexpected_end());
            }
            (_, 'a'..='z' | 'A'..='Z' | '_' | '-', _) => {
              self.node_name.push(self.current_char);
            }
            (_, WS, _) => {
              self.consume_node_name();
              self.node_content.push(WS);
              self.state = State::NodeContent;
            }
            (_, LF, _) => {
              self.consume_node_name();
              self.node_content.push(LF);
              self.state = State::NewLine;
            }
            (_, other, _) => return Err(err_unexpected_character(other, self.row, self.column)),
          }
        }
        State::Indentation => {
          // Consume the indentation at the beginning of the line.
          match self.context() {
            (_, EMPTY_CHAR, _) => return Err(err_unexpected_end()),
            (_, NODE_START, _) => {
              self.consume_node_content();
              self.consume_indentation();
              self.state = State::NodeName
            }
            (_, WS, _) => self.indentation.push(WS),
            (_, other, _) => {
              self.node_content.push_str(&self.indentation);
              self.node_content.push(other);
              self.indentation.clear();
              self.state = State::NodeContent
            }
          }
        }
        State::NodeContent => {
          // Consume the node content.
          match self.context() {
            (_, EMPTY_CHAR, _) => return Err(err_unexpected_end()),
            (_, LF, _) => {
              self.node_content.push(LF);
              self.state = State::NewLine
            }
            (_, other, _) => self.node_content.push(other),
          }
        }
        State::MultiLineComment => {
          // Consume the content of a multi line comment.
          match self.context() {
            (_, EMPTY_CHAR, _) => return Err(err_unexpected_end()),
            (_, _, EMPTY_CHAR) => return Err(err_unexpected_end()),
            (_, ASTERISK, SLASH) => {
              self.consume_char();
              self.state = self.next_state;
            }
            _ => {}
          }
        }
        State::SingleLineComment => {
          // Consume the content of a single line comment.
          match self.context() {
            (_, EMPTY_CHAR, _) => return Err(err_unexpected_end()),
            (_, _, EMPTY_CHAR) => return Err(err_unexpected_end()),
            (_, LF, _) => self.state = self.next_state,
            _ => {}
          }
        }
      }
    }
    Ok(self.tokens.clone())
  }

  /// Returns the context, i.e. characters around the current position.
  fn context(&self) -> (char, char, char) {
    (self.previous_char, self.current_char, self.next_char)
  }

  /// Consumes the next character on input.
  fn next(&mut self) -> char {
    self.previous_char = self.current_char;
    self.column += 1;
    self.chars.next().unwrap_or(EMPTY_CHAR)
  }

  /// Peeks the next character on input.
  fn peek(&mut self) -> char {
    self.chars.peek().cloned().unwrap_or(EMPTY_CHAR)
  }

  /// Consumes the next character on input.
  fn consume_char(&mut self) {
    self.next();
    self.next_char = self.peek();
  }

  /// Consumes the node name.
  fn consume_node_name(&mut self) {
    if !self.node_name.is_empty() {
      self.tokens.push(Token::NodeName(self.node_name.clone()));
      self.node_name.clear();
    }
  }

  /// Consumes the node content.
  fn consume_node_content(&mut self) {
    if !self.node_content.is_empty() {
      self.tokens.push(Token::NodeContent(self.node_content.clone()));
    }
    self.node_content.clear();
  }

  /// Consumes the indentation.
  fn consume_indentation(&mut self) {
    if !self.indentation.is_empty() {
      self.tokens.push(Token::Indentation(self.indentation.len()));
      self.indentation.clear();
    }
  }
}
