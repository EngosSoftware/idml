//! # Parser for Indented Delimiter Markup Language(s)

#![deny(missing_docs)]
#![deny(rustdoc::broken_intra_doc_links)]
#![deny(rustdoc::missing_crate_level_docs)]

mod defs;
mod errors;
mod node;
mod parser;
mod tokenizer;

pub use defs::{NULL, TAB, WS};
pub use node::Node;
pub use parser::{parse, Parser};
pub use tokenizer::{tokenize, Token, Tokenizer};
