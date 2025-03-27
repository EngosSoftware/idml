//! # Parser for Indented Delimiter Markup Language(s)

#![deny(missing_docs)]
#![deny(rustdoc::broken_intra_doc_links)]
#![deny(rustdoc::missing_crate_level_docs)]

mod defs;
mod errors;
mod node;
mod parser;
mod tokenizer;

pub use node::Node;
pub use parser::{Parser, parse};
pub use tokenizer::{Token, Tokenizer, tokenize};
