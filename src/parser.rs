//! # IDML parser implementation

use crate::errors::*;
use crate::{Node, Token, tokenize};
use std::vec::IntoIter;

/// Parses input text.
pub fn parse(input: &str) -> Result<Node> {
  Parser::new(tokenize(input)?).parse()
}

/// Parses input tokens.
pub fn parse_tokens(tokens: Vec<Token>) -> Result<Node> {
  Parser::new(tokens).parse()
}

/// Parser state.
pub enum ParserState {
  /// Expected the indentation token.
  Indentation,
  /// Expected the node name token.
  NodeName,
  /// Expected the node content token.
  NodeContent,
}

/// Parser.
pub struct Parser {
  pub state: ParserState,
  pub tokens: IntoIter<Token>,
  pub nodes: Vec<Node>,
  pub stack: Vec<Node>,
  pub first_indent: Option<usize>,
  pub last_indent: Option<usize>,
  pub last_name: Option<String>,
}

impl Parser {
  /// Creates a new instance of the parser.
  pub fn new(tokens: Vec<Token>) -> Self {
    Self {
      state: ParserState::Indentation,
      tokens: tokens.into_iter(),
      nodes: vec![],
      stack: vec![],
      first_indent: None,
      last_indent: None,
      last_name: None,
    }
  }

  /// Parses the tokens.
  pub fn parse(mut self) -> Result<Node> {
    loop {
      let Some(token) = self.tokens.next() else {
        break;
      };
      match self.state {
        ParserState::Indentation => {
          if let Token::Indentation(indent) = token {
            if self.first_indent.is_none() && indent > 0 {
              self.first_indent = Some(indent);
            }
            self.last_indent = Some(indent);
            self.state = ParserState::NodeName;
          } else {
            return Err(err_expected_indentation());
          }
        }
        ParserState::NodeName => {
          if let Token::NodeName(name) = token {
            self.last_name = Some(name);
            self.state = ParserState::NodeContent;
          } else {
            return Err(err_expected_node_name());
          }
        }
        ParserState::NodeContent => {
          if let Token::NodeContent(content) = token {
            let Some(indent) = self.last_indent else {
              return Err(err_no_previous_indentation());
            };
            let Some(name) = self.last_name.clone() else {
              return Err(err_no_previous_node_name());
            };
            self.create_node(indent, '.', name, content)?;
            self.last_indent = None;
            self.last_name = None;
            self.state = ParserState::Indentation;
          } else {
            return Err(err_expected_node_content());
          }
        }
      }
    }
    for mut node in self.nodes.drain(..).rev() {
      if self.stack.is_empty() {
        self.stack.push(node);
      } else {
        while !self.stack.is_empty() && node.level() < self.stack.last().unwrap().level() {
          node.add_child(self.stack.pop().unwrap());
        }
        self.stack.push(node);
      }
    }
    let mut root = Node::root('.');
    while let Some(node) = self.stack.pop() {
      root.add_child(node);
    }
    Ok(root)
  }

  /// Creates a new node and adds it to the parsed node list.
  fn create_node(&mut self, indent: usize, delimiter: char, name: String, content: String) -> Result<()> {
    let multiplier = self.first_indent.unwrap_or(0);
    if multiplier > 0 && indent % multiplier != 0 {
      return Err(err_malformed_indentation(indent, multiplier));
    }
    let level = if multiplier > 0 { (indent / multiplier) + 1 } else { 1 };
    let node = Node::new(level, delimiter, name, content);
    self.nodes.push(node);
    Ok(())
  }
}
