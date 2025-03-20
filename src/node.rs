//! # Node definition

use std::fmt::Write;

const ROOT_NAME: &str = "root";
const ROOT_CONTENT: &str = "";

/// Document node.
#[derive(Debug, Clone)]
pub struct Node {
  /// Indentation level of the node.
  level: usize,
  /// The name of the node.
  name: String,
  /// The text content of the node.
  content: String,
  /// Child nodes.
  children: Vec<Node>,
}

impl Node {
  /// Creates a root node.
  pub(crate) fn root() -> Self {
    Self {
      level: 0,
      name: ROOT_NAME.to_string(),
      content: ROOT_CONTENT.to_string(),
      children: vec![],
    }
  }

  pub(crate) fn is_root(&self) -> bool {
    self.name == ROOT_NAME
  }

  /// Creates a new node with name and content.
  pub fn new(level: usize, name: String, content: String) -> Self {
    Self {
      level,
      name,
      content,
      children: vec![],
    }
  }

  /// Adds a node as a last child.
  pub fn add_child(&mut self, node: Node) {
    self.children.push(node);
  }

  /// Returns the indentation level of the node.
  pub fn level(&self) -> usize {
    self.level
  }

  /// Returns the name of the node.
  pub fn name(&self) -> &str {
    &self.name
  }

  /// Returns the raw content of the node.
  pub fn content(&self) -> &str {
    &self.content
  }

  /// Returns the trimmed content of the node.
  pub fn text(&self) -> &str {
    self.content.trim()
  }

  /// Returns the iterator over child nodes.
  pub fn children(&self) -> impl Iterator<Item = &Node> {
    self.children.iter()
  }

  /// Returns a document recreated starting from this node.
  pub fn document(&self, indent: usize) -> String {
    let mut buffer = String::new();
    if !self.is_root() {
      let indentation = if self.level > 1 { " ".repeat((self.level - 1) * indent) } else { "".to_string() };
      let _ = write!(&mut buffer, "{}.{}{}", indentation, self.name, self.content);
    }
    for child in &self.children {
      let _ = write!(&mut buffer, "{}", child.document(indent));
    }
    buffer
  }
}
