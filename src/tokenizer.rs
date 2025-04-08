//! # Tokenizer implementation

use crate::defs::*;
use crate::errors::*;
use std::iter::Peekable;
use std::str::Chars;

/// Tokenizes input text.
pub fn tokenize(input: &str) -> Result<Vec<Token>> {
  Tokenizer::new(input).tokenize()
}

/// Line endings.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum LineEnding {
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

impl LineEnding {
  fn first(&self) -> char {
    match self {
      LineEnding::Lf => '\n',
      LineEnding::Cr => '\r',
      LineEnding::CrLf => '\r',
    }
  }
}

/// Tokens.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
  /// Node name token with delimiter character.
  NodeName(String, char),
  /// Node content token.
  NodeContent(String),
  /// Indentation token with indentation character.
  Indentation(usize, char),
}

/// Tokenizer state.
#[derive(Copy, Clone)]
enum TokenizerState {
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
}

/// Tokenizer.
pub struct Tokenizer<'a> {
  /// Current row position in processed input.
  row: usize,
  /// Current column position in processed content.
  column: usize,
  /// Current tokenizing state.
  state: TokenizerState,
  /// Input chars.
  chars: Peekable<Chars<'a>>,
  /// Currently processed character.
  current_char: char,
  /// The previously processed character.
  previous_char: char,
  /// The next character on input.
  next_char: char,
  /// Last parsed line ending.
  line_ending: LineEnding,
  /// The content of currently processed indentation.
  indentation: String,
  /// Delimiter used in processed document.
  delimiter: char,
  /// The name of currently processed node.
  node_name: String,
  /// The content of currently processed node.
  node_content: String,
  /// List of already processed tokens.
  tokens: Vec<Token>,
}

impl<'a> Tokenizer<'a> {
  /// Creates a new instance of the tokenizer.
  pub fn new(input: &'a str) -> Self {
    Self {
      row: 1,
      column: 0,
      state: TokenizerState::Start,
      chars: input.chars().peekable(),
      current_char: NULL,
      previous_char: NULL,
      next_char: NULL,
      line_ending: LineEnding::Lf,
      indentation: "".to_string(),
      delimiter: NULL,
      node_name: "".to_string(),
      node_content: "".to_string(),
      tokens: vec![],
    }
  }

  /// Tokenizes the input text.
  pub fn tokenize(mut self) -> Result<Vec<Token>> {
    loop {
      // Normalize the end-of-line character(s).
      (self.current_char, self.next_char) = match (self.next(true), self.peek()) {
        (LF, ch) => {
          self.line_ending = LineEnding::Lf;
          (LF, ch)
        }
        (CR, LF) => {
          self.line_ending = LineEnding::CrLf;
          (self.next(false), self.peek())
        }
        (CR, ch) => {
          self.line_ending = LineEnding::Cr;
          (LF, ch)
        }
        (ch1, ch2) => (ch1, ch2),
      };
      match self.state {
        TokenizerState::Start => {
          // Process the beginning of the file.
          match self.context() {
            (_, NULL, _) => return Err(err_empty_input()),
            (_, ch, _) if self.is_allowed_delimiter(ch) => {
              self.delimiter = ch;
              self.tokens.push(Token::Indentation(0, NULL));
              self.state = TokenizerState::NodeName;
            }
            (_, other, _) => {
              return Err(err_unexpected_character(if other == LF { self.line_ending.first() } else { other }, self.row, self.column));
            }
          }
        }
        TokenizerState::NewLine => {
          // Decide what at the beginning of the line.
          match self.context() {
            (_, NULL, _) => {
              self.consume_node_content();
              break;
            }
            (_, ch, _) if self.is_delimiter(ch) => {
              self.consume_node_content();
              self.tokens.push(Token::Indentation(0, NULL));
              self.state = TokenizerState::NodeName;
            }
            (_, WS, _) => {
              self.indentation.push(WS);
              self.state = TokenizerState::Indentation;
            }
            (_, TAB, _) => {
              self.indentation.push(TAB);
              self.state = TokenizerState::Indentation;
            }
            (_, LF, _) => {
              self.new_row();
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
            (_, WS, _) => {
              self.consume_node_name();
              self.node_content.push(WS);
              self.state = TokenizerState::NodeContent;
            }
            (_, TAB, _) => {
              self.consume_node_name();
              self.node_content.push(TAB);
              self.state = TokenizerState::NodeContent;
            }
            (_, LF, _) => {
              self.new_row();
              self.consume_node_name();
              self.node_content.push_str(self.line_ending.as_ref());
              self.state = TokenizerState::NewLine;
            }
            (_, ch, _) if self.is_node_name_char(ch) => {
              self.node_name.push(self.current_char);
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
            (_, ch, _) if self.is_delimiter(ch) => {
              self.consume_node_content();
              self.consume_indentation()?;
              self.state = TokenizerState::NodeName
            }
            (_, WS, _) => self.indentation.push(WS),
            (_, TAB, _) => self.indentation.push(TAB),
            (_, ch, _) => {
              self.node_content.push_str(&self.indentation);
              self.node_content.push(ch);
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
              self.new_row();
              self.node_content.push_str(self.line_ending.as_ref());
              self.state = TokenizerState::NewLine
            }
            (_, other, _) => self.node_content.push(other),
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
  fn next(&mut self, increment_column: bool) -> char {
    self.previous_char = self.current_char;
    if increment_column {
      self.column += 1;
    }
    self.chars.next().unwrap_or(NULL)
  }

  /// Peeks the next character on input.
  fn peek(&mut self) -> char {
    self.chars.peek().cloned().unwrap_or(NULL)
  }

  /// Consumes the indentation.
  fn consume_indentation(&mut self) -> Result<()> {
    if self.indentation.chars().all(|ch| ch == WS) {
      self.tokens.push(Token::Indentation(self.indentation.len(), WS));
      self.indentation.clear();
      Ok(())
    } else if self.indentation.chars().all(|ch| ch == TAB) {
      self.tokens.push(Token::Indentation(self.indentation.len(), TAB));
      self.indentation.clear();
      Ok(())
    } else {
      Err(err_inconsistent_indentation())
    }
  }

  /// Consumes the node name.
  fn consume_node_name(&mut self) {
    self.tokens.push(Token::NodeName(self.node_name.clone(), self.delimiter));
    self.node_name.clear();
  }

  /// Consumes the node content.
  fn consume_node_content(&mut self) {
    self.tokens.push(Token::NodeContent(self.node_content.clone()));
    self.node_content.clear();
  }

  /// Returns `true` when the specified character is allowed delimiter character.
  fn is_allowed_delimiter(&self, ch: char) -> bool {
    matches!(ch, '\u{0021}'..='\u{10FFFF}')
  }

  /// Returns `true` when the specified character is recognized delimiter.
  fn is_delimiter(&self, ch: char) -> bool {
    ch == self.delimiter
  }

  /// Returns `true` when the specified character is a node name character.
  fn is_node_name_char(&self, ch: char) -> bool {
    matches!(ch, '\u{0021}'..='\u{10FFFF}')
  }

  /// Advances the counter to the new row.
  fn new_row(&mut self) {
    self.row += 1;
    self.column = 0;
  }
}
