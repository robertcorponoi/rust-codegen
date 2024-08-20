use std::fmt::{self, Debug, Write};

use crate::bound::Bound;
use crate::r#type::Type;

/// The default value to use for any indentation values.
const DEFAULT_INDENT: usize = 4;

/// Configures how a scope is formatted.
///
pub struct Formatter<'a> {
    /// Write destination.
    dst: &'a mut (dyn Write + 'a),
    /// Number of spaces to start a new line with.
    spaces: usize,
    /// Number of spaces per indentiation.
    indent: usize,
    /// Last written char
    last_char: Option<u8>,
    /// Should write a newline
    newline: bool,
}

impl<'a> Debug for Formatter<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Formatter")
            .field("spaces", &self.spaces)
            .field("indent", &self.indent)
            .field("last_char", &self.last_char)
            .finish()
    }
}

/// Trait for formatting into string
///
/// provides a .to_string() method
pub trait FormatCode {
    /// Write this items information to the formatter.
    fn fmt_code(&self, fmt: &mut Formatter<'_>) -> fmt::Result;
}

impl<'a> Formatter<'a> {
    /// Return a new formatter that writes to the given string.
    ///
    /// # Arguments
    ///
    /// * `dst` - The destination of the formatted string.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_codegen::Formatter;
    ///
    /// let mut dest = String::new();
    /// let mut fmt = Formatter::new(&mut dest);
    /// ```
    pub fn new(dst: &'a mut (dyn Write + 'a)) -> Self {
        Formatter {
            dst,
            spaces: 0,
            indent: DEFAULT_INDENT,
            last_char: None,
            newline: false,
        }
    }

    /// Push a &str to this formatters underlying Write object
    pub fn push_str(&mut self, s: &str) -> fmt::Result {
        self.last_char = s.as_bytes().last().copied();
        self.dst.write_str(s)
    }

    /// Wrap the given function inside a block.
    pub fn block(&mut self, f: impl FnOnce(&mut Self) -> fmt::Result) -> fmt::Result {
        if !self.is_start_of_line() {
            write!(self, " ")?;
        }

        writeln!(self, "{{")?;
        self.indent(f)?;
        writeln!(self, "}}")?;
        Ok(())
    }

    /// Call the given function with the indentation level incremented by one.
    pub fn indent<F, R>(&mut self, f: F) -> R
    where
        F: FnOnce(&mut Self) -> R,
    {
        self.spaces += self.indent;
        let ret = f(self);
        self.spaces -= self.indent;
        ret
    }

    /// Check if current destination is the start of a new line.
    pub fn is_start_of_line(&self) -> bool {
        self.last_char.is_none() || self.last_char == Some(b'\n')
    }

    /// Pushes the number of spaces defined for a new line.
    fn push_spaces(&mut self) -> fmt::Result {
        for _ in 0..self.spaces {
            self.push_str(" ")?;
        }
        Ok(())
    }
}

impl<'a> fmt::Write for Formatter<'a> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for (i, line) in s.lines().enumerate() {
            if i != 0 || self.newline {
                self.newline = false;
                self.push_str("\n")?;
            }

            if line.is_empty() {
                continue;
            }

            if self.is_start_of_line() {
                self.push_spaces()?;
            }

            self.push_str(line)?;
        }
        self.last_char = s.as_bytes().last().copied();
        if self.last_char == Some(b'\n') {
            self.newline = true;
        }

        Ok(())
    }
}

/// Format generics.
pub fn fmt_generics(generics: &[String], fmt: &mut Formatter<'_>) -> fmt::Result {
    if !generics.is_empty() {
        write!(fmt, "<")?;

        for (i, ty) in generics.iter().enumerate() {
            if i != 0 {
                write!(fmt, ", ")?
            }
            write!(fmt, "{}", ty)?;
        }

        write!(fmt, ">")?;
    }

    Ok(())
}

/// Format generic bounds.
pub fn fmt_bounds(bounds: &[Bound], fmt: &mut Formatter<'_>) -> fmt::Result {
    if !bounds.is_empty() {
        writeln!(fmt)?;

        // Write first bound
        write!(fmt, "where {}: ", bounds[0].name)?;
        fmt_bound_rhs(&bounds[0].bound, fmt)?;
        writeln!(fmt, ",")?;

        for bound in &bounds[1..] {
            write!(fmt, "      {}: ", bound.name)?;
            fmt_bound_rhs(&bound.bound, fmt)?;
            writeln!(fmt, ",")?;
        }
    }

    Ok(())
}

/// Format multiple generic bounds.
pub fn fmt_bound_rhs(tys: &[Type], fmt: &mut Formatter<'_>) -> fmt::Result {
    for (i, ty) in tys.iter().enumerate() {
        if i != 0 {
            write!(fmt, " + ")?
        }
        FormatCode::fmt_code(ty, fmt)?;
    }

    Ok(())
}
