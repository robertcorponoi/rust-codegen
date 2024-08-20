use std::fmt::{self, Write};

use crate::{formatter::Formatter, FormatCode};

/// Used to apply documentation to the module, trait, etc.
#[derive(Debug, Clone)]
pub struct Docs {
    /// The documentation to add.
    docs: String,
}

impl Docs {
    /// Creates new documentation.
    ///
    /// # Arguments
    ///
    /// * `docs` - The docs to add.
    pub fn new(docs: &str) -> Self {
        Docs {
            docs: docs.to_string(),
        }
    }
}
impl FormatCode for Docs {
    /// Formats the documentation using the provided formatter. This will also
    /// add the `///` before each line of documentation.
    ///
    /// # Arguments
    ///
    /// * `fmt` - The formatter to use.
    fn fmt_code(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        for line in self.docs.lines() {
            writeln!(fmt, "/// {}", line)?;
        }

        Ok(())
    }
}
