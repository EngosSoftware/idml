//! # Tokenizer implementation

use crate::errors::*;
use std::fmt::Write;
use std::iter::Peekable;
use std::str::Chars;

/// Character indicating the start of the node name.
pub const NODE_START: char = '.';

/// Whitespace character.
pub const WS: char = ' ';

/// Line feed character.
pub const LF: char = '\n';

/// Carriage return character.
pub const CR: char = '\r';

/// Slash character.
pub const SLASH: char = '/';

/// Asterisk character.
pub const ASTERISK: char = '*';

/// Empty character (zero).
pub const NULL: char = '\u{0}';

/// Tokenizes input text.
pub fn tokenize(input: &str) -> Result<Vec<Token>> {
  Tokenizer::new(input).tokenize()
}

/// Combines tokens back to previously tokenized input text.
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

/// Line endings.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum LineEnding {
  Lf,
  Cr,
  CrLf,
}

impl AsRef<str> for LineEnding {
  fn as_ref(&self) -> &str {
    match self {
      LineEnding::Lf => "\n",
      LineEnding::Cr => "\r",
      LineEnding::CrLf => "\r\n",
    }
  }
}

/// Tokens.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
  NodeName(String),
  NodeContent(String),
  Indentation(usize),
}

/// Tokenizer state.
#[derive(Copy, Clone)]
pub enum TokenizerState {
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

/// Tokenizer.
pub struct Tokenizer<'a> {
  /// Current row position in processed input.
  pub row: usize,
  /// Current column position in processed content.
  pub column: usize,
  /// Current tokenizing state.
  pub state: TokenizerState,
  /// Next state after processing comments.
  pub next_state: TokenizerState,
  /// Input chars.
  pub chars: Peekable<Chars<'a>>,
  /// Currently processed character.
  pub current_char: char,
  /// The previously processed character.
  pub previous_char: char,
  /// The next character on input.
  pub next_char: char,
  /// Last parsed line ending.
  pub line_ending: LineEnding,
  /// The name of currently processed node.
  pub node_name: String,
  /// The content of currently processed node.
  pub node_content: String,
  /// The content of currently processed indentation.
  pub indentation: String,
  /// Processed tokens.
  pub tokens: Vec<Token>,
}

impl<'a> Tokenizer<'a> {
  /// Creates a new instance of the tokenizer.
  pub fn new(input: &'a str) -> Self {
    Self {
      row: 1,
      column: 0,
      state: TokenizerState::Start,
      next_state: TokenizerState::Start,
      chars: input.chars().peekable(),
      current_char: NULL,
      previous_char: NULL,
      next_char: NULL,
      line_ending: LineEnding::Lf,
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
          self.line_ending = LineEnding::Lf;
          (LF, other)
        }
        (CR, LF) => {
          self.row += 1;
          self.column = 0;
          self.line_ending = LineEnding::CrLf;
          (self.next(), self.peek())
        }
        (CR, other) => {
          self.row += 1;
          self.column = 0;
          self.line_ending = LineEnding::Cr;
          (LF, other)
        }
        (other1, other2) => (other1, other2),
      };
      match self.state {
        TokenizerState::Start => {
          // Process the beginning of the file.
          match self.context() {
            (_, NULL, _) => return Err(err_unexpected_end()),
            (_, NODE_START, _) => {
              self.tokens.push(Token::Indentation(0));
              self.state = TokenizerState::NodeName;
            }
            (_, SLASH, NULL) => return Err(err_unexpected_end()),
            (_, SLASH, SLASH) => {
              self.consume_char();
              self.next_state = TokenizerState::Start;
              self.state = TokenizerState::SingleLineComment;
            }
            (_, SLASH, ASTERISK) => {
              self.consume_char();
              self.next_state = TokenizerState::Start;
              self.state = TokenizerState::MultiLineComment;
            }
            (_, SLASH, other) => {
              return Err(err_unexpected_character(other, self.row, self.column + 1));
            }
            (_, other, _) => {
              return Err(err_unexpected_character(other, self.row, self.column));
            }
          }
        }
        TokenizerState::NewLine => {
          // Decide what to do in the new line.
          match self.context() {
            (_, NULL, _) => {
              self.consume_node_content();
              break;
            }
            (_, NODE_START, _) => {
              self.consume_node_content();
              self.tokens.push(Token::Indentation(0));
              self.state = TokenizerState::NodeName;
            }
            (_, WS, _) => {
              self.indentation.push(WS);
              self.state = TokenizerState::Indentation;
            }
            (_, LF, _) => {
              self.node_content.push_str(self.line_ending.as_ref());
            }
            (_, other, _) => {
              self.node_content.push(other);
              self.state = TokenizerState::NodeContent;
            }
          }
        }
        TokenizerState::NodeName => {
          // Consume the node name.
          match self.context() {
            (_, NULL, _) => {
              return Err(err_unexpected_end());
            }
            (_, 'a'..='z' | 'A'..='Z' | '_' | '-', _) => {
              self.node_name.push(self.current_char);
            }
            (_, WS, _) => {
              self.consume_node_name();
              self.node_content.push(WS);
              self.state = TokenizerState::NodeContent;
            }
            (_, LF, _) => {
              self.consume_node_name();
              self.node_content.push_str(self.line_ending.as_ref());
              self.state = TokenizerState::NewLine;
            }
            (_, other, _) => {
              return Err(err_unexpected_character(other, self.row, self.column));
            }
          }
        }
        TokenizerState::Indentation => {
          // Consume the indentation at the beginning of the line.
          match self.context() {
            (_, NULL, _) => return Err(err_unexpected_end()),
            (_, NODE_START, _) => {
              self.consume_node_content();
              self.consume_indentation();
              self.state = TokenizerState::NodeName
            }
            (_, WS, _) => self.indentation.push(WS),
            (_, other, _) => {
              self.node_content.push_str(&self.indentation);
              self.node_content.push(other);
              self.indentation.clear();
              self.state = TokenizerState::NodeContent
            }
          }
        }
        TokenizerState::NodeContent => {
          // Consume the node content.
          match self.context() {
            (_, NULL, _) => return Err(err_unexpected_end()),
            (_, LF, _) => {
              self.node_content.push_str(self.line_ending.as_ref());
              self.state = TokenizerState::NewLine
            }
            (_, other, _) => self.node_content.push(other),
          }
        }
        TokenizerState::MultiLineComment => {
          // Consume the content of a multi line comment.
          match self.context() {
            (_, NULL, _) => return Err(err_unexpected_end()),
            (_, _, NULL) => return Err(err_unexpected_end()),
            (_, ASTERISK, SLASH) => {
              self.consume_char();
              self.state = self.next_state;
            }
            _ => {}
          }
        }
        TokenizerState::SingleLineComment => {
          // Consume the content of a single line comment.
          match self.context() {
            (_, NULL, _) => return Err(err_unexpected_end()),
            (_, _, NULL) => return Err(err_unexpected_end()),
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
    self.chars.next().unwrap_or(NULL)
  }

  /// Peeks the next character on input.
  fn peek(&mut self) -> char {
    self.chars.peek().cloned().unwrap_or(NULL)
  }

  /// Consumes the next character on input.
  fn consume_char(&mut self) {
    self.next();
    self.next_char = self.peek();
  }

  /// Consumes the node name.
  fn consume_node_name(&mut self) {
    self.tokens.push(Token::NodeName(self.node_name.clone()));
    self.node_name.clear();
  }

  /// Consumes the node content.
  fn consume_node_content(&mut self) {
    self.tokens.push(Token::NodeContent(self.node_content.clone()));
    self.node_content.clear();
  }

  /// Consumes the indentation.
  fn consume_indentation(&mut self) {
    self.tokens.push(Token::Indentation(self.indentation.len()));
    self.indentation.clear();
  }
}
