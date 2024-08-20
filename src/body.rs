use std::fmt::{self, Write};

use crate::block::Block;
use crate::formatter::Formatter;

/// Defines the types of content that go in functions and blocks.
#[derive(Debug, Clone)]
pub enum Body {
    /// Used to push lines to blocks.
    String(String),
    /// Used to create blocks.
    Block(Block),
}

impl Body {
    /// Formats the string or block with the given formatter.
    /// 
    /// # Arguments
    /// 
    /// * `fmt` - The formatter to use.
    pub fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        match &self {
            Body::String(s) => writeln!(fmt, "{}", s),
            Body::Block(b) => b.fmt(fmt),
        }
    }
}
