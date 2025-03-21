//! # IDML tokenizer implementation

use crate::errors::*;
use std::fmt::Write;
use std::iter::Peekable;
use std::str::Chars;

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

/// Hyphen character.
const HYPHEN: char = '-';

/// Underscore character.
const UNDERSCORE: char = '_';

/// Empty character (zero).
const NULL: char = 0 as char;

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
      Token::NodeName(name, delimiter) => {
        let _ = write!(&mut buffer, "{delimiter}{name}");
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
  /// Node name token with delimiter.
  NodeName(String, char),
  /// Node content token.
  NodeContent(String),
  /// Indentation token.
  Indentation(usize),
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
  /// Expecting node name starting character.
  NodeNameStart,
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
  row: usize,
  /// Current column position in processed content.
  column: usize,
  /// Current tokenizing state.
  state: TokenizerState,
  /// Next state after processing comments.
  next_state: TokenizerState,
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
      next_state: TokenizerState::Start,
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
    if self.chars.peek().is_none() {
      // Report an error when the input is empty.
      return Err(err_empty_input());
    }
    // Process all characters from the input.
    loop {
      // Normalize the end-of-line characters.
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
            (_, NULL, _) => return Err(err_unexpected_end()),
            (_, ch, _) if self.is_allowed_delimiter(ch) => {
              self.delimiter = ch;
              self.tokens.push(Token::Indentation(0));
              self.state = TokenizerState::NodeNameStart;
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
            (_, ch, _) if self.is_delimiter(ch) => {
              self.consume_node_content();
              self.tokens.push(Token::Indentation(0));
              self.state = TokenizerState::NodeNameStart;
            }
            (_, WS, _) => {
              self.indentation.push(WS);
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
        TokenizerState::NodeNameStart => {
          // Consume the node name starting character.
          match self.context() {
            (_, NULL, _) => {
              return Err(err_unexpected_end());
            }
            (_, ch, _) if self.is_node_name_start_char(ch) => {
              self.node_name.push(self.current_char);
              self.state = TokenizerState::NodeName;
            }
            (_, other, _) => {
              return Err(err_unexpected_character(if other == LF { self.line_ending.first() } else { other }, self.row, self.column));
            }
          }
        }
        TokenizerState::NodeName => {
          // Consume the node name.
          match self.context() {
            (_, NULL, _) => {
              return Err(err_unexpected_end());
            }
            (_, ch, _) if self.is_node_name_char(ch) => {
              self.node_name.push(self.current_char);
            }
            (_, WS, _) => {
              self.consume_node_name();
              self.node_content.push(WS);
              self.state = TokenizerState::NodeContent;
            }
            (_, LF, _) => {
              self.new_row();
              self.consume_node_name();
              self.node_content.push_str(self.line_ending.as_ref());
              self.state = TokenizerState::NewLine;
            }
            (_, ch, _) => {
              return Err(err_unexpected_character(ch, self.row, self.column));
            }
          }
        }
        TokenizerState::Indentation => {
          // Consume the indentation at the beginning of the line.
          match self.context() {
            (_, NULL, _) => return Err(err_unexpected_end()),
            (_, ch, _) if self.is_delimiter(ch) => {
              self.consume_node_content();
              self.consume_indentation();
              self.state = TokenizerState::NodeNameStart
            }
            (_, WS, _) => self.indentation.push(WS),
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
            (_, LF, _) => {
              self.new_row();
              self.state = self.next_state
            }
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

  /// Consumes the next character on input.
  fn consume_char(&mut self) {
    self.next(true);
    self.next_char = self.peek();
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

  /// Consumes the indentation.
  fn consume_indentation(&mut self) {
    self.tokens.push(Token::Indentation(self.indentation.len()));
    self.indentation.clear();
  }

  /// Returns `true` when the specified character is allowed delimiter character.
  fn is_allowed_delimiter(&self, ch: char) -> bool {
    !(self.is_node_name_char(ch) || matches!(ch, '\u{0}' | WS | LF | CR | SLASH | UNDERSCORE | HYPHEN))
  }

  /// Returns `true` when the specified character is recognized delimiter.
  fn is_delimiter(&self, ch: char) -> bool {
    ch == self.delimiter
  }

  /// Returns `true` when the specified character is a node name starting character.
  fn is_node_name_start_char(&self, ch: char) -> bool {
    matches!(ch, 'a'..='z' | 'A'..='Z' | UNDERSCORE )
  }

  /// Returns `true` when the specified character is a node name character.
  fn is_node_name_char(&self, ch: char) -> bool {
    matches!(ch, 'a'..='z' | 'A'..='Z' | '0'..='9' | UNDERSCORE | HYPHEN)
  }

  /// Advances the counter to the new row.
  fn new_row(&mut self) {
    self.row += 1;
    self.column = 0;
  }
}
