use std::fmt::{self, Write};

use crate::body::Body;
use crate::formatter::Formatter;
use crate::FormatCode;

/// Defines a code block. This is used to define a function body.
#[derive(Debug, Clone)]
pub struct Block {
    /// Content before the block.
    before: Option<String>,
    /// Content after the block.
    after: Option<String>,
    /// The contents inside the block.
    body: Vec<Body>,
}

impl Block {
    /// Returns an empty code block.
    ///
    /// # Arguments
    ///
    /// * `before` - The contents to add before the block. This can be an empty "" if you don't want anything before the block.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_codegen::Block;
    ///
    /// let mut block = Block::new("");
    /// ```
    pub fn new(before: &str) -> Self {
        Block {
            before: Some(before.to_string()),
            after: None,
            body: vec![],
        }
    }

    /// Push a line to the code block.
    ///
    /// # Arguments
    ///
    /// * `line` - The line to add to the code block.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_codegen::Block;
    ///
    /// let mut block = Block::new("");
    /// block.line("println!(\"Hello, world!\");");
    /// ```
    pub fn line(&mut self, line: impl ToString) -> &mut Self {
        self.body.push(Body::String(line.to_string()));
        self
    }

    /// Push a nested block to this block.
    ///
    /// # Arguments
    ///
    /// * `block` - The block to push to this block.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_codegen::Block;
    ///
    /// let mut block_1 = Block::new("");
    /// block_1.line("println!(\"Hello, world!\");");
    ///
    /// let mut block_2 = Block::new("");
    /// block_2.line("println!(\"from Rust!!\");");
    ///
    /// block_1.push_block(block_2);
    /// ```
    pub fn push_block(&mut self, block: Block) -> &mut Self {
        self.body.push(Body::Block(block));
        self
    }

    /// Add a snippet after the block.
    ///
    /// # Arguments
    ///
    /// * `after` - The snippet to add after the code block.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_codegen::Block;
    ///
    /// let mut block = Block::new("This is before");
    /// block.after("This is after");
    /// ```
    pub fn after(&mut self, after: &str) -> &mut Self {
        self.after = Some(after.to_string());
        self
    }
}
impl FormatCode for Block {
    /// Formats the block using the given formatter.
    ///
    /// # Arguments
    ///
    /// * `fmt` - The formatter to use.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_codegen::*;
    ///
    /// let mut dest = String::new();
    /// let mut fmt = Formatter::new(&mut dest);
    ///
    /// let mut block = Block::new("This is before");
    /// block.fmt_code(&mut fmt);
    /// ```
    fn fmt_code(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        if let Some(ref before) = self.before {
            write!(fmt, "{}", before)?;
        }

        // Inlined `Formatter::fmt`

        if !fmt.is_start_of_line() {
            write!(fmt, " ")?;
        }

        writeln!(fmt, "{{")?;

        fmt.indent(|fmt| {
            for b in &self.body {
                b.fmt_code(fmt)?;
            }

            Ok(())
        })?;

        write!(fmt, "}}")?;

        if let Some(ref after) = self.after {
            write!(fmt, "{}", after)?;
        }

        writeln!(fmt)?;
        Ok(())
    }
}
