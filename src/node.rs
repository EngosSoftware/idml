//! # Tree node implementation

use std::fmt::Write;

const ROOT_LEVEL: usize = 0;
const ROOT_DELIMITER: char = 0 as char;
const ROOT_NAME: &str = "root";
const ROOT_CONTENT: &str = "";

/// Tree node.
#[derive(Debug, Clone)]
pub struct Node {
  /// Indentation level of the node.
  /// Root note has level `0`. Top level nodes have value `1`.
  level: usize,
  /// Name delimiter.
  /// Original name delimiter as defined in the parsed document.
  delimiter: char,
  /// The name of the node.
  /// Original name as defined in the parsed document but without delimiter.
  name: String,
  /// The content of the node.
  /// Original content as defined in the parsed document.
  content: String,
  /// Child nodes.
  /// A list of all child nodes in the document tree.
  children: Vec<Node>,
}

impl Node {
  /// Creates a root node.
  pub(crate) fn root() -> Self {
    Self {
      level: ROOT_LEVEL,
      delimiter: ROOT_DELIMITER,
      name: ROOT_NAME.to_string(),
      content: ROOT_CONTENT.to_string(),
      children: vec![],
    }
  }

  /// Returns `true` when this node is a root node.
  pub(crate) fn is_root(&self) -> bool {
    self.level == ROOT_LEVEL && self.delimiter == ROOT_DELIMITER && self.name == ROOT_NAME && self.content == ROOT_CONTENT
  }

  /// Creates a new node with delimiter, name and content.
  pub fn new(level: usize, delimiter: char, name: String, content: String) -> Self {
    Self {
      level,
      delimiter,
      name,
      content,
      children: vec![],
    }
  }

  /// Adds a child node at the end of the children list.
  pub fn add_child(&mut self, node: Node) {
    self.children.push(node);
  }

  /// Returns the indentation level.
  pub fn level(&self) -> usize {
    self.level
  }

  /// Returns the delimiter.
  pub fn delimiter(&self) -> char {
    self.delimiter
  }

  /// Returns the node name.
  pub fn name(&self) -> &str {
    &self.name
  }

  /// Returns the node content.
  pub fn content(&self) -> &str {
    &self.content
  }

  /// Returns the node text, text is a trimmed node content.
  pub fn text(&self) -> &str {
    self.content.trim()
  }

  /// Returns the iterator over child nodes.
  pub fn children(&self) -> impl Iterator<Item = &Node> {
    self.children.iter()
  }

  /// Returns a document starting from this node.
  pub fn document(&self, indent: usize) -> String {
    let mut buffer = String::new();
    if !self.is_root() {
      let indentation = if self.level > 1 { " ".repeat((self.level - 1) * indent) } else { "".to_string() };
      let _ = write!(&mut buffer, "{}{}{}{}", indentation, self.delimiter, self.name, self.content);
    }
    for child in &self.children {
      let _ = write!(&mut buffer, "{}", child.document(indent));
    }
    buffer
  }
}
