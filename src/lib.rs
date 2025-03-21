//! # Parser for Indented Delimiter Markup Language(s)

mod errors;
mod node;
mod parser;
mod tokenizer;

pub use node::Node;
pub use parser::{Parser, parse, parse_tokens};
pub use tokenizer::{Token, Tokenizer, join_tokens, tokenize};
