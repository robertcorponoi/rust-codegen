use std::fmt::{self, Write};

use crate::block::Block;
use crate::formatter::Formatter;
use crate::FormatCode;

/// Defines the types of content that go in functions and blocks.
#[derive(Debug, Clone)]
pub enum Body {
    /// Used to push lines to blocks.
    String(String),
    /// Used to create blocks.
    Block(Block),
}

impl FormatCode for Body {
    /// Formats the string or block with the given formatter.
    ///
    /// # Arguments
    ///
    /// * `fmt` - The formatter to use.
    fn fmt_code(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        match &self {
            Body::String(s) => writeln!(fmt, "{}", s),
            Body::Block(b) => b.fmt_code(fmt),
        }
    }
}
